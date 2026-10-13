// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/bake_mod_textures.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 35cfcfb4ef4e93e4a7aff26e8bf61057f5728f35accebbdb729bd2c95d246c40
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: bake_mod_textures.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 528, 608, 1647, 1651, 1656, 1658, 1682, 1749, 1822
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use asset_pipe; // namespace candidate
//   package identities: [{"name":"asset-pipe","source":"local_or_path","version":"0.1.0"}]
// use bevy_tasks; // namespace candidate
//   package identities: [{"name":"bevy_tasks","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use futures_lite; // namespace candidate
//   package identities: [{"name":"futures-lite","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use turbosloth; // namespace candidate
//   package identities: [{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":608,"record_offset":29572,"rva":13100384,"va":5381809504}
// 编译实例 RVAs: 0xc7e560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2984564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8748a2d00","size":5,"soft_fail":false,"target_va":5384795984,"va":5381811415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::DirEntry>::path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2876743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e847e52b00","size":5,"soft_fail":false,"target_va":5384689312,"va":5381812564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2955824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8301a2d00","size":5,"soft_fail":false,"target_va":5384768304,"va":5381812475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2835270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e846432b00","size":5,"soft_fail":false,"target_va":5384644912,"va":5381809637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2843621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8e5632b00","size":5,"soft_fail":false,"target_va":5384656240,"va":5381812614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::is_file
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2844121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8d9652b00","size":5,"soft_fail":false,"target_va":5384656800,"va":5381812674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::sys::fs::windows::FindNextFileHandle as core::ops::drop::Drop>::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2949169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e831002d00","size":5,"soft_fail":false,"target_va":5384764464,"va":5381815290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::elapsed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2829385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8492c2b00","size":5,"soft_fail":false,"target_va":5384648448,"va":5381819058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2830574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8ee302b00","size":5,"soft_fail":false,"target_va":5384648032,"va":5381817453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t2797644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e84cb02a00","size":5,"soft_fail":false,"target_va":5384609408,"va":5381811759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-6132822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8aa6ba2ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381811505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=41
//     evidence={"assembly":"\tcallq\t-6132033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8bf6ea2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381810732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13089305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8e74538ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381811492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f920
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12591842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e81edd3fff","size":5,"soft_fail":false,"target_va":5369231648,"va":5381823485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140089010
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12554258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8ee6f40ff","size":5,"soft_fail":false,"target_va":5369270288,"va":5381824541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140098ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12485897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8f77a41ff","size":5,"soft_fail":false,"target_va":5369335008,"va":5381820900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12471777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e81fb241ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5381810620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ffa0
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12458121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e877e741ff","size":5,"soft_fail":false,"target_va":5369364384,"va":5381822500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12155049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8578746ff","size":5,"soft_fail":false,"target_va":5369660368,"va":5381815412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403ce4e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9119021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8d3da74ff","size":5,"soft_fail":false,"target_va":5372699872,"va":5381818888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040fae0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8851935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e821ee78ff","size":5,"soft_fail":false,"target_va":5372967648,"va":5381819578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048d130
//     owner=unresolved_namespace; count=5
//     target_aliases=3 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8328893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e843e980ff","size":5,"soft_fail":false,"target_va":5373481264,"va":5381810152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409711d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3206674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8ee11cfff","size":5,"soft_fail":false,"target_va":5378609616,"va":5381816285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972470
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3207081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e85710cfff","size":5,"soft_fail":false,"target_va":5378614384,"va":5381821460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972c40
//     owner=unresolved_namespace; count=3
//     target_aliases=4 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3200881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e88f28cfff","size":5,"soft_fail":false,"target_va":5378616384,"va":5381817260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409787f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3176569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e88787cfff","size":5,"soft_fail":false,"target_va":5378639856,"va":5381816420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097c720
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3164543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e881b6cfff","size":5,"soft_fail":false,"target_va":5378656032,"va":5381820570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140985070
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3127832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8e845d0ff","size":5,"soft_fail":false,"target_va":5378691184,"va":5381819011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f4c0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3086960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e890e5d0ff","size":5,"soft_fail":false,"target_va":5378733248,"va":5381820203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f9c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3091945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e817d2d0ff","size":5,"soft_fail":false,"target_va":5378734528,"va":5381826468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c85120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e820300000","size":5,"soft_fail":false,"target_va":5381837088,"va":5381824763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f5120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9917309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e87d539700","size":5,"soft_fail":false,"target_va":5391733024,"va":5381815710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f3840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10957656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e85833a700","size":5,"soft_fail":false,"target_va":5392775232,"va":5381817571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10955212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8cc29a700","size":5,"soft_fail":false,"target_va":5392780768,"va":5381825551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14179d860
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11658137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e899e3b100","size":5,"soft_fail":false,"target_va":5393471584,"va":5381813442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141a85910
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14701581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e80d54e000","size":5,"soft_fail":false,"target_va":5396519184,"va":5381817598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c35830
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16472908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e84c5bfb00","size":5,"soft_fail":false,"target_va":5398288432,"va":5381815519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34565403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e81b6d0f02","size":5,"soft_fail":false,"target_va":5416385024,"va":5381819616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d9fd20
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34730126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e88ef01102","size":5,"soft_fail":false,"target_va":5416549664,"va":5381819533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35246742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e896d21902","size":5,"soft_fail":false,"target_va":5417057040,"va":5381810293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35246464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e880d11902","size":5,"soft_fail":false,"target_va":5417057056,"va":5381810587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34556214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e836490f02","size":5,"soft_fail":false,"target_va":5416371088,"va":5381814869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34804414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8be121302","size":5,"soft_fail":false,"target_va":5416620794,"va":5381816375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t307765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e835b20400","size":5,"soft_fail":false,"target_va":5382122800,"va":5381815030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2806179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8a3d12a00","size":5,"soft_fail":false,"target_va":5384624080,"va":5381817896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::BuildTextureAsset::new_array
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1143251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8d3711100","size":5,"soft_fail":false,"target_va":5382959040,"va":5381815784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::texture::BuildTextureAsset::new_single
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1143070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e81e711100","size":5,"soft_fail":false,"target_va":5382958976,"va":5381815901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34801719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e837081302","size":5,"soft_fail":false,"target_va":5416621408,"va":5381819684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<tg_mods::overlay::ModTextureLayers> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8de2e0000","size":5,"soft_fail":false,"target_va":5381836944,"va":5381824941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8224f0000","size":5,"soft_fail":false,"target_va":5381837216,"va":5381816953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34801377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8e1061302","size":5,"soft_fail":false,"target_va":5416621376,"va":5381819994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::add_or_replace
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2542117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8db35d9ff","size":5,"soft_fail":false,"target_va":5379276736,"va":5381818848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::load_texture::create_game_texture
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-982293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8eb02f1ff","size":5,"soft_fail":false,"target_va":5380836016,"va":5381818304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::bake_mod_textures::texture_mappings
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e87aefffff","size":5,"soft_fail":false,"target_va":5381808480,"va":5381812705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mod_textures::packaged_textures
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2733162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e8964bd6ff","size":5,"soft_fail":false,"target_va":5379076672,"va":5381809829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2146383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e84fc02000","size":5,"soft_fail":false,"target_va":5383963792,"va":5381817404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2154891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e88be12000","size":5,"soft_fail":false,"target_va":5383972224,"va":5381817328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2146936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e878c22000","size":5,"soft_fail":false,"target_va":5383964912,"va":5381817971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2145898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e86abe2000","size":5,"soft_fail":false,"target_va":5383962816,"va":5381816913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::read_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2914622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e83e792c00","size":5,"soft_fail":false,"target_va":5384725152,"va":5381810525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2910519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e837692c00","size":5,"soft_fail":false,"target_va":5384726688,"va":5381816164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34799703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e857001302","size":5,"soft_fail":false,"target_va":5416619360,"va":5381819652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::is_mod_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1070177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e861541000","size":5,"soft_fail":false,"target_va":5382884432,"va":5381814250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::resolve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1105423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381809504,"raw_hex":"e80fde1000","size":5,"soft_fail":false,"target_va":5382919648,"va":5381814220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":31080,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":64688,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":64744,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":63572,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":29796,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":59668,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":608,"record_offset":64916,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":7484,"source_line":1488,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":608,"record_offset":65968,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":21763,"source_line":34,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":608,"record_offset":65940,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":21762,"source_line":30,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::bake_mod_textures::plan_bakes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/bake_mod_textures.rs
//     evidence={"module":608,"record_offset":31108,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":38708,"source_line":98,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] futures_lite::future::block_on
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"futures-lite","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-lite-2.3.0/src/future.rs
//     evidence={"module":608,"record_offset":63408,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":5014,"source_line":56,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":608,"record_offset":63628,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":63488,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":608,"record_offset":62572,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":608,"record_offset":61168,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":608,"record_offset":61560,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":608,"record_offset":66664,"parent_record_offset":29572,"ancestor_record_offset":29572,"inlinee":13816,"source_line":1042,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":53840,"parent_record_offset":53816,"ancestor_record_offset":29572,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":53864,"parent_record_offset":53840,"ancestor_record_offset":29572,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34836,"parent_record_offset":34812,"ancestor_record_offset":29572,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34860,"parent_record_offset":34836,"ancestor_record_offset":29572,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":53888,"parent_record_offset":53864,"ancestor_record_offset":29572,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34884,"parent_record_offset":34860,"ancestor_record_offset":29572,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":53816,"parent_record_offset":53660,"ancestor_record_offset":29572,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=44
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34812,"parent_record_offset":34748,"ancestor_record_offset":29572,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":43424,"parent_record_offset":43392,"ancestor_record_offset":29572,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":53584,"parent_record_offset":53548,"ancestor_record_offset":29572,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":34780,"parent_record_offset":34748,"ancestor_record_offset":29572,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":34748,"parent_record_offset":34720,"ancestor_record_offset":29572,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":43472,"parent_record_offset":43448,"ancestor_record_offset":29572,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":43448,"parent_record_offset":43424,"ancestor_record_offset":29572,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":53660,"parent_record_offset":53620,"ancestor_record_offset":29572,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":53620,"parent_record_offset":53584,"ancestor_record_offset":29572,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":34720,"parent_record_offset":34692,"ancestor_record_offset":29572,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":53704,"parent_record_offset":53660,"ancestor_record_offset":29572,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":608,"record_offset":53436,"parent_record_offset":53400,"ancestor_record_offset":29572,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":608,"record_offset":53472,"parent_record_offset":53436,"ancestor_record_offset":29572,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":608,"record_offset":53508,"parent_record_offset":53472,"ancestor_record_offset":29572,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":608,"record_offset":53400,"parent_record_offset":53364,"ancestor_record_offset":29572,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":608,"record_offset":53364,"parent_record_offset":53264,"ancestor_record_offset":29572,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":608,"record_offset":66012,"parent_record_offset":65968,"ancestor_record_offset":29572,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":608,"record_offset":32244,"parent_record_offset":32216,"ancestor_record_offset":29572,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":608,"record_offset":46168,"parent_record_offset":46132,"ancestor_record_offset":29572,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":608,"record_offset":46132,"parent_record_offset":46108,"ancestor_record_offset":29572,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":43392,"parent_record_offset":43344,"ancestor_record_offset":29572,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":31304,"parent_record_offset":31108,"ancestor_record_offset":29572,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":43308,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":43344,"parent_record_offset":43308,"ancestor_record_offset":29572,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":53548,"parent_record_offset":53508,"ancestor_record_offset":29572,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":54688,"parent_record_offset":54664,"ancestor_record_offset":29572,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":31332,"parent_record_offset":31108,"ancestor_record_offset":29572,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":34560,"parent_record_offset":34536,"ancestor_record_offset":29572,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":38648,"parent_record_offset":37956,"ancestor_record_offset":29572,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":37956,"parent_record_offset":37928,"ancestor_record_offset":29572,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":38868,"parent_record_offset":38844,"ancestor_record_offset":29572,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":31388,"parent_record_offset":31108,"ancestor_record_offset":29572,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":38720,"parent_record_offset":38696,"ancestor_record_offset":29572,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":608,"record_offset":32272,"parent_record_offset":32244,"ancestor_record_offset":29572,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::blob::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":608,"record_offset":41748,"parent_record_offset":41724,"ancestor_record_offset":29572,"inlinee":38726,"source_line":197,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41544,"parent_record_offset":41444,"ancestor_record_offset":29572,"inlinee":38718,"source_line":34,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41632,"parent_record_offset":41444,"ancestor_record_offset":29572,"inlinee":38720,"source_line":55,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$44::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41688,"parent_record_offset":41444,"ancestor_record_offset":29572,"inlinee":38724,"source_line":82,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$56::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41660,"parent_record_offset":41444,"ancestor_record_offset":29572,"inlinee":38722,"source_line":103,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41516,"parent_record_offset":41444,"ancestor_record_offset":29572,"inlinee":38716,"source_line":9,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$67::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41444,"parent_record_offset":41412,"ancestor_record_offset":29572,"inlinee":38714,"source_line":122,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":608,"record_offset":53728,"parent_record_offset":53704,"ancestor_record_offset":29572,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":62928,"parent_record_offset":62900,"ancestor_record_offset":29572,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":30392,"parent_record_offset":30364,"ancestor_record_offset":29572,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":30292,"parent_record_offset":30088,"ancestor_record_offset":29572,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":49456,"parent_record_offset":49400,"ancestor_record_offset":29572,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":62868,"parent_record_offset":62836,"ancestor_record_offset":29572,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":62900,"parent_record_offset":62868,"ancestor_record_offset":29572,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":63016,"parent_record_offset":62900,"ancestor_record_offset":29572,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":63128,"parent_record_offset":63104,"ancestor_record_offset":29572,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":41412,"parent_record_offset":41380,"ancestor_record_offset":29572,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":42884,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":41596,"parent_record_offset":41572,"ancestor_record_offset":29572,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":32216,"parent_record_offset":32188,"ancestor_record_offset":29572,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":608,"record_offset":35832,"parent_record_offset":35808,"ancestor_record_offset":29572,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":608,"record_offset":31776,"parent_record_offset":31752,"ancestor_record_offset":29572,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":608,"record_offset":31860,"parent_record_offset":31836,"ancestor_record_offset":29572,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":608,"record_offset":32384,"parent_record_offset":31668,"ancestor_record_offset":29572,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":608,"record_offset":30708,"parent_record_offset":30676,"ancestor_record_offset":29572,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":608,"record_offset":49140,"parent_record_offset":49116,"ancestor_record_offset":29572,"inlinee":16013,"source_line":23,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":608,"record_offset":49660,"parent_record_offset":49636,"ancestor_record_offset":29572,"inlinee":17047,"source_line":142,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":608,"record_offset":37008,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":608,"record_offset":30768,"parent_record_offset":30744,"ancestor_record_offset":29572,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":608,"record_offset":30876,"parent_record_offset":30816,"ancestor_record_offset":29572,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":608,"record_offset":54712,"parent_record_offset":54688,"ancestor_record_offset":29572,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":608,"record_offset":38696,"parent_record_offset":38672,"ancestor_record_offset":29572,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":608,"record_offset":38672,"parent_record_offset":38648,"ancestor_record_offset":29572,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":49172,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":49116,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":16011,"source_line":957,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":37032,"parent_record_offset":37008,"ancestor_record_offset":29572,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":37088,"parent_record_offset":37056,"ancestor_record_offset":29572,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":30744,"parent_record_offset":30616,"ancestor_record_offset":29572,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":30952,"parent_record_offset":30928,"ancestor_record_offset":29572,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":37056,"parent_record_offset":37032,"ancestor_record_offset":29572,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":608,"record_offset":37684,"parent_record_offset":37656,"ancestor_record_offset":29572,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":608,"record_offset":37708,"parent_record_offset":37684,"ancestor_record_offset":29572,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":608,"record_offset":30256,"parent_record_offset":30232,"ancestor_record_offset":29572,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":36760,"parent_record_offset":36732,"ancestor_record_offset":29572,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":36788,"parent_record_offset":36760,"ancestor_record_offset":29572,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":31016,"parent_record_offset":30816,"ancestor_record_offset":29572,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":35344,"parent_record_offset":35316,"ancestor_record_offset":29572,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":42552,"parent_record_offset":42516,"ancestor_record_offset":29572,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":30420,"parent_record_offset":30364,"ancestor_record_offset":29572,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":608,"record_offset":32008,"parent_record_offset":31980,"ancestor_record_offset":29572,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":608,"record_offset":31980,"parent_record_offset":31952,"ancestor_record_offset":29572,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":608,"record_offset":37144,"parent_record_offset":37116,"ancestor_record_offset":29572,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":608,"record_offset":30000,"parent_record_offset":29964,"ancestor_record_offset":29572,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":608,"record_offset":37116,"parent_record_offset":37088,"ancestor_record_offset":29572,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":608,"record_offset":32556,"parent_record_offset":31108,"ancestor_record_offset":29572,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":608,"record_offset":42908,"parent_record_offset":42884,"ancestor_record_offset":29572,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":608,"record_offset":30120,"parent_record_offset":30088,"ancestor_record_offset":29572,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":608,"record_offset":41488,"parent_record_offset":41444,"ancestor_record_offset":29572,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::new_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":608,"record_offset":63456,"parent_record_offset":63408,"ancestor_record_offset":29572,"inlinee":5016,"source_line":1347,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":608,"record_offset":35864,"parent_record_offset":35780,"ancestor_record_offset":29572,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":608,"record_offset":49372,"parent_record_offset":49344,"ancestor_record_offset":29572,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":608,"record_offset":53944,"parent_record_offset":53508,"ancestor_record_offset":29572,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":608,"record_offset":38792,"parent_record_offset":38768,"ancestor_record_offset":29572,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":31800,"parent_record_offset":31776,"ancestor_record_offset":29572,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=209
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":33776,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":31356,"parent_record_offset":31332,"ancestor_record_offset":29572,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":36320,"parent_record_offset":36296,"ancestor_record_offset":29572,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":38768,"parent_record_offset":38744,"ancestor_record_offset":29572,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":30208,"parent_record_offset":30088,"ancestor_record_offset":29572,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":32124,"parent_record_offset":32100,"ancestor_record_offset":29572,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":42728,"parent_record_offset":42704,"ancestor_record_offset":29572,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":30848,"parent_record_offset":30816,"ancestor_record_offset":29572,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":46200,"parent_record_offset":46168,"ancestor_record_offset":29572,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":61980,"parent_record_offset":61956,"ancestor_record_offset":29572,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":38744,"parent_record_offset":38720,"ancestor_record_offset":29572,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":31476,"parent_record_offset":31388,"ancestor_record_offset":29572,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":61956,"parent_record_offset":61932,"ancestor_record_offset":29572,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":31420,"parent_record_offset":31388,"ancestor_record_offset":29572,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":31500,"parent_record_offset":31476,"ancestor_record_offset":29572,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":30232,"parent_record_offset":30208,"ancestor_record_offset":29572,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":42752,"parent_record_offset":42728,"ancestor_record_offset":29572,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":608,"record_offset":33736,"parent_record_offset":33712,"ancestor_record_offset":29572,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":608,"record_offset":37172,"parent_record_offset":37144,"ancestor_record_offset":29572,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":608,"record_offset":32300,"parent_record_offset":32272,"ancestor_record_offset":29572,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":608,"record_offset":62264,"parent_record_offset":62236,"ancestor_record_offset":29572,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":608,"record_offset":32328,"parent_record_offset":32300,"ancestor_record_offset":29572,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":608,"record_offset":62236,"parent_record_offset":62208,"ancestor_record_offset":29572,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":608,"record_offset":61908,"parent_record_offset":61884,"ancestor_record_offset":29572,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":608,"record_offset":61932,"parent_record_offset":61908,"ancestor_record_offset":29572,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":608,"record_offset":62292,"parent_record_offset":62264,"ancestor_record_offset":29572,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":608,"record_offset":30816,"parent_record_offset":30792,"ancestor_record_offset":29572,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":608,"record_offset":61884,"parent_record_offset":61768,"ancestor_record_offset":29572,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":608,"record_offset":61296,"parent_record_offset":61272,"ancestor_record_offset":29572,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":608,"record_offset":61596,"parent_record_offset":61560,"ancestor_record_offset":29572,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":608,"record_offset":61736,"parent_record_offset":61704,"ancestor_record_offset":29572,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":608,"record_offset":61704,"parent_record_offset":61668,"ancestor_record_offset":29572,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":608,"record_offset":61644,"parent_record_offset":61620,"ancestor_record_offset":29572,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":608,"record_offset":61668,"parent_record_offset":61644,"ancestor_record_offset":29572,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":608,"record_offset":61620,"parent_record_offset":61596,"ancestor_record_offset":29572,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":608,"record_offset":62208,"parent_record_offset":61668,"ancestor_record_offset":29572,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":608,"record_offset":61272,"parent_record_offset":61240,"ancestor_record_offset":29572,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":608,"record_offset":61240,"parent_record_offset":61212,"ancestor_record_offset":29572,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":608,"record_offset":61400,"parent_record_offset":61368,"ancestor_record_offset":29572,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":608,"record_offset":61456,"parent_record_offset":61400,"ancestor_record_offset":29572,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":608,"record_offset":61368,"parent_record_offset":61336,"ancestor_record_offset":29572,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":608,"record_offset":61212,"parent_record_offset":61168,"ancestor_record_offset":29572,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":608,"record_offset":61480,"parent_record_offset":61456,"ancestor_record_offset":29572,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":608,"record_offset":61768,"parent_record_offset":61736,"ancestor_record_offset":29572,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":608,"record_offset":62052,"parent_record_offset":61768,"ancestor_record_offset":29572,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":608,"record_offset":62024,"parent_record_offset":61768,"ancestor_record_offset":29572,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":608,"record_offset":61828,"parent_record_offset":61768,"ancestor_record_offset":29572,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":65088,"parent_record_offset":65052,"ancestor_record_offset":29572,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":42824,"parent_record_offset":42800,"ancestor_record_offset":29572,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":42848,"parent_record_offset":42824,"ancestor_record_offset":29572,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":46228,"parent_record_offset":46168,"ancestor_record_offset":29572,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":46260,"parent_record_offset":46132,"ancestor_record_offset":29572,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\tuple.rs
//     evidence={"module":608,"record_offset":41380,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":27934,"source_line":34,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/bake_mod_textures.rs
//     evidence={"module":608,"record_offset":30900,"parent_record_offset":30876,"ancestor_record_offset":29572,"inlinee":38732,"source_line":55,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/bake_mod_textures.rs
//     evidence={"module":608,"record_offset":64952,"parent_record_offset":64916,"ancestor_record_offset":29572,"inlinee":38730,"source_line":77,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::bake_mod_textures::bake_recipe
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/bake_mod_textures.rs
//     evidence={"module":608,"record_offset":53264,"parent_record_offset":31108,"ancestor_record_offset":29572,"inlinee":38712,"source_line":156,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::bake_mod_textures::mod_new_texture_opts
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/bake_mod_textures.rs
//     evidence={"module":608,"record_offset":32668,"parent_record_offset":32616,"ancestor_record_offset":29572,"inlinee":38711,"source_line":239,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::bake_mod_textures::plan_bakes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/bake_mod_textures.rs
//     evidence={"module":608,"record_offset":32616,"parent_record_offset":32556,"ancestor_record_offset":29572,"inlinee":38710,"source_line":115,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::bake_mod_textures::upload_baked_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/bake_mod_textures.rs
//     evidence={"module":608,"record_offset":64980,"parent_record_offset":64952,"ancestor_record_offset":29572,"inlinee":38731,"source_line":203,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":608,"record_offset":31920,"parent_record_offset":31892,"ancestor_record_offset":29572,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":608,"record_offset":31952,"parent_record_offset":31920,"ancestor_record_offset":29572,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":608,"record_offset":31892,"parent_record_offset":31668,"ancestor_record_offset":29572,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":31752,"parent_record_offset":31668,"ancestor_record_offset":29572,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":35808,"parent_record_offset":35780,"ancestor_record_offset":29572,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":32412,"parent_record_offset":31668,"ancestor_record_offset":29572,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":42124,"parent_record_offset":42084,"ancestor_record_offset":29572,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":31836,"parent_record_offset":31668,"ancestor_record_offset":29572,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":608,"record_offset":31724,"parent_record_offset":31668,"ancestor_record_offset":29572,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":608,"record_offset":42676,"parent_record_offset":41848,"ancestor_record_offset":29572,"inlinee":7447,"source_line":28,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":608,"record_offset":32188,"parent_record_offset":32160,"ancestor_record_offset":29572,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":43732,"parent_record_offset":43708,"ancestor_record_offset":29572,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":31560,"parent_record_offset":31536,"ancestor_record_offset":29572,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":66712,"parent_record_offset":66688,"ancestor_record_offset":29572,"inlinee":4348,"source_line":2005,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":66740,"parent_record_offset":66712,"ancestor_record_offset":29572,"inlinee":4349,"source_line":2044,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":30648,"parent_record_offset":30616,"ancestor_record_offset":29572,"inlinee":8392,"source_line":1103,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":32160,"parent_record_offset":32052,"ancestor_record_offset":29572,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":30616,"parent_record_offset":30588,"ancestor_record_offset":29572,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":30976,"parent_record_offset":30952,"ancestor_record_offset":29572,"inlinee":12038,"source_line":4661,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":31592,"parent_record_offset":31560,"ancestor_record_offset":29572,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":43012,"parent_record_offset":42988,"ancestor_record_offset":29572,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42988,"parent_record_offset":42964,"ancestor_record_offset":29572,"inlinee":8503,"source_line":573,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36296,"parent_record_offset":35692,"ancestor_record_offset":29572,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":32100,"parent_record_offset":32076,"ancestor_record_offset":29572,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36148,"parent_record_offset":35928,"ancestor_record_offset":29572,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42704,"parent_record_offset":41848,"ancestor_record_offset":29572,"inlinee":7448,"source_line":561,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":32504,"parent_record_offset":31668,"ancestor_record_offset":29572,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35780,"parent_record_offset":35724,"ancestor_record_offset":29572,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35928,"parent_record_offset":35900,"ancestor_record_offset":29572,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":32076,"parent_record_offset":32052,"ancestor_record_offset":29572,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":31644,"parent_record_offset":31620,"ancestor_record_offset":29572,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":31620,"parent_record_offset":31560,"ancestor_record_offset":29572,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":41848,"parent_record_offset":41816,"ancestor_record_offset":29572,"inlinee":7438,"source_line":1114,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":30676,"parent_record_offset":30648,"ancestor_record_offset":29572,"inlinee":8393,"source_line":955,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36700,"parent_record_offset":36668,"ancestor_record_offset":29572,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42592,"parent_record_offset":41848,"ancestor_record_offset":29572,"inlinee":7446,"source_line":2382,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35752,"parent_record_offset":35724,"ancestor_record_offset":29572,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35692,"parent_record_offset":35620,"ancestor_record_offset":29572,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35620,"parent_record_offset":35588,"ancestor_record_offset":29572,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":31668,"parent_record_offset":31644,"ancestor_record_offset":29572,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":41940,"parent_record_offset":41896,"ancestor_record_offset":29572,"inlinee":7440,"source_line":1992,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_index_in_group
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42084,"parent_record_offset":41940,"ancestor_record_offset":29572,"inlinee":7441,"source_line":1787,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42272,"parent_record_offset":41940,"ancestor_record_offset":29572,"inlinee":7443,"source_line":1750,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36668,"parent_record_offset":35620,"ancestor_record_offset":29572,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42308,"parent_record_offset":42272,"ancestor_record_offset":29572,"inlinee":7444,"source_line":2685,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35660,"parent_record_offset":35620,"ancestor_record_offset":29572,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35724,"parent_record_offset":35692,"ancestor_record_offset":29572,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":41896,"parent_record_offset":41848,"ancestor_record_offset":29572,"inlinee":7439,"source_line":1950,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":43924,"parent_record_offset":43840,"ancestor_record_offset":29572,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42492,"parent_record_offset":41896,"ancestor_record_offset":29572,"inlinee":7445,"source_line":2540,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36732,"parent_record_offset":36700,"ancestor_record_offset":29572,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35588,"parent_record_offset":35560,"ancestor_record_offset":29572,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35900,"parent_record_offset":35692,"ancestor_record_offset":29572,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":32052,"parent_record_offset":31668,"ancestor_record_offset":29572,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\rustc_entry.rs
//     evidence={"module":608,"record_offset":42964,"parent_record_offset":42940,"ancestor_record_offset":29572,"inlinee":38728,"source_line":307,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\rustc_entry.rs
//     evidence={"module":608,"record_offset":41816,"parent_record_offset":41784,"ancestor_record_offset":29572,"inlinee":7470,"source_line":512,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":608,"record_offset":66688,"parent_record_offset":66664,"ancestor_record_offset":29572,"inlinee":13817,"source_line":1175,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":608,"record_offset":30588,"parent_record_offset":30560,"ancestor_record_offset":29572,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":43128,"parent_record_offset":43104,"ancestor_record_offset":29572,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":608,"record_offset":43080,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":608,"record_offset":43104,"parent_record_offset":43080,"ancestor_record_offset":29572,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":43152,"parent_record_offset":43128,"ancestor_record_offset":29572,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":42800,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":608,"record_offset":62608,"parent_record_offset":62572,"ancestor_record_offset":29572,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":608,"record_offset":63832,"parent_record_offset":63800,"ancestor_record_offset":29572,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":608,"record_offset":64312,"parent_record_offset":64096,"ancestor_record_offset":29572,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":608,"record_offset":62764,"parent_record_offset":62736,"ancestor_record_offset":29572,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":608,"record_offset":62836,"parent_record_offset":62688,"ancestor_record_offset":29572,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":608,"record_offset":63052,"parent_record_offset":62836,"ancestor_record_offset":29572,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":43708,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":21897,"source_line":1231,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":41312,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":8388,"source_line":975,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":31536,"parent_record_offset":31108,"ancestor_record_offset":29572,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":32952,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":6553,"source_line":271,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":49560,"parent_record_offset":49208,"ancestor_record_offset":29572,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":42940,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":38727,"source_line":2667,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":41784,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":8397,"source_line":2881,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$78::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":49208,"parent_record_offset":49172,"ancestor_record_offset":29572,"inlinee":12269,"source_line":2919,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$79::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":49588,"parent_record_offset":49208,"ancestor_record_offset":29572,"inlinee":12226,"source_line":2936,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":32980,"parent_record_offset":32952,"ancestor_record_offset":29572,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":41340,"parent_record_offset":41312,"ancestor_record_offset":29572,"inlinee":19384,"source_line":2976,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":608,"record_offset":30532,"parent_record_offset":29832,"ancestor_record_offset":29572,"inlinee":11864,"source_line":234,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":608,"record_offset":30560,"parent_record_offset":29832,"ancestor_record_offset":29572,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$9::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":608,"record_offset":29832,"parent_record_offset":29796,"ancestor_record_offset":29572,"inlinee":11863,"source_line":1149,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":608,"record_offset":33712,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":608,"record_offset":29908,"parent_record_offset":29880,"ancestor_record_offset":29572,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":608,"record_offset":30168,"parent_record_offset":30144,"ancestor_record_offset":29572,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":608,"record_offset":30028,"parent_record_offset":30000,"ancestor_record_offset":29572,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":608,"record_offset":30364,"parent_record_offset":29964,"ancestor_record_offset":29572,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":608,"record_offset":29880,"parent_record_offset":29832,"ancestor_record_offset":29572,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":608,"record_offset":37260,"parent_record_offset":32668,"ancestor_record_offset":29572,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":608,"record_offset":30056,"parent_record_offset":30028,"ancestor_record_offset":29572,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":608,"record_offset":30088,"parent_record_offset":30056,"ancestor_record_offset":29572,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":608,"record_offset":29964,"parent_record_offset":29936,"ancestor_record_offset":29572,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":608,"record_offset":29936,"parent_record_offset":29908,"ancestor_record_offset":29572,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x974C
// 首条位置: {"module":608,"inlinee":38732}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x974A
// 首条位置: {"module":608,"inlinee":38730}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":65028,"parent_record_offset":64952,"ancestor_record_offset":64952,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::bake_mod_textures::upload_baked_texture
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/bake_mod_textures.rs
//     evidence={"module":608,"record_offset":64980,"parent_record_offset":64952,"ancestor_record_offset":64952,"inlinee":38731,"source_line":203,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":608,"record_offset":65052,"parent_record_offset":65028,"ancestor_record_offset":64952,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":65088,"parent_record_offset":65052,"ancestor_record_offset":64952,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":65112,"parent_record_offset":65088,"ancestor_record_offset":64952,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":65144,"parent_record_offset":65052,"ancestor_record_offset":64952,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::bake_mod_textures::plan_bakes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9734
// 首条位置: {"module":608,"inlinee":38708}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":31304,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":57368,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":31332,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":31388,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":55488,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":608,"record_offset":32556,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":4101,"source_line":1805,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":55908,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::bake_mod_textures::bake_recipe
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/bake_mod_textures.rs
//     evidence={"module":608,"record_offset":53264,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":38712,"source_line":156,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":608,"record_offset":55544,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":55404,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":31536,"parent_record_offset":31108,"ancestor_record_offset":31108,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":53840,"parent_record_offset":53816,"ancestor_record_offset":31108,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":53864,"parent_record_offset":53840,"ancestor_record_offset":31108,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34836,"parent_record_offset":34812,"ancestor_record_offset":31108,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34860,"parent_record_offset":34836,"ancestor_record_offset":31108,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":53888,"parent_record_offset":53864,"ancestor_record_offset":31108,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34884,"parent_record_offset":34860,"ancestor_record_offset":31108,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":53816,"parent_record_offset":53660,"ancestor_record_offset":31108,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=39
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34812,"parent_record_offset":34748,"ancestor_record_offset":31108,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":43424,"parent_record_offset":43392,"ancestor_record_offset":31108,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":53584,"parent_record_offset":53548,"ancestor_record_offset":31108,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":34780,"parent_record_offset":34748,"ancestor_record_offset":31108,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":34748,"parent_record_offset":34720,"ancestor_record_offset":31108,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":43472,"parent_record_offset":43448,"ancestor_record_offset":31108,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":43448,"parent_record_offset":43424,"ancestor_record_offset":31108,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":53660,"parent_record_offset":53620,"ancestor_record_offset":31108,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":53620,"parent_record_offset":53584,"ancestor_record_offset":31108,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=37
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":34720,"parent_record_offset":34692,"ancestor_record_offset":31108,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":53704,"parent_record_offset":53660,"ancestor_record_offset":31108,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":608,"record_offset":53436,"parent_record_offset":53400,"ancestor_record_offset":31108,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":608,"record_offset":53472,"parent_record_offset":53436,"ancestor_record_offset":31108,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":608,"record_offset":53508,"parent_record_offset":53472,"ancestor_record_offset":31108,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":608,"record_offset":53400,"parent_record_offset":53364,"ancestor_record_offset":31108,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":608,"record_offset":53364,"parent_record_offset":53264,"ancestor_record_offset":31108,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":608,"record_offset":32244,"parent_record_offset":32216,"ancestor_record_offset":31108,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":608,"record_offset":46168,"parent_record_offset":46132,"ancestor_record_offset":31108,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":608,"record_offset":46132,"parent_record_offset":46108,"ancestor_record_offset":31108,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":43392,"parent_record_offset":43344,"ancestor_record_offset":31108,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":33684,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":43308,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":43344,"parent_record_offset":43308,"ancestor_record_offset":31108,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":53548,"parent_record_offset":53508,"ancestor_record_offset":31108,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":54688,"parent_record_offset":54664,"ancestor_record_offset":31108,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":37656,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":34560,"parent_record_offset":34536,"ancestor_record_offset":31108,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":38648,"parent_record_offset":37956,"ancestor_record_offset":31108,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":37956,"parent_record_offset":37928,"ancestor_record_offset":31108,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":38868,"parent_record_offset":38844,"ancestor_record_offset":31108,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":37772,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":38720,"parent_record_offset":38696,"ancestor_record_offset":31108,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":608,"record_offset":32272,"parent_record_offset":32244,"ancestor_record_offset":31108,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::blob::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":608,"record_offset":41748,"parent_record_offset":41724,"ancestor_record_offset":31108,"inlinee":38726,"source_line":197,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41544,"parent_record_offset":41444,"ancestor_record_offset":31108,"inlinee":38718,"source_line":34,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41632,"parent_record_offset":41444,"ancestor_record_offset":31108,"inlinee":38720,"source_line":55,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$44::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41688,"parent_record_offset":41444,"ancestor_record_offset":31108,"inlinee":38724,"source_line":82,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$56::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41660,"parent_record_offset":41444,"ancestor_record_offset":31108,"inlinee":38722,"source_line":103,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41516,"parent_record_offset":41444,"ancestor_record_offset":31108,"inlinee":38716,"source_line":9,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$67::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41444,"parent_record_offset":41412,"ancestor_record_offset":31108,"inlinee":38714,"source_line":122,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":608,"record_offset":53728,"parent_record_offset":53704,"ancestor_record_offset":31108,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":33536,"parent_record_offset":33508,"ancestor_record_offset":31108,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":33436,"parent_record_offset":33232,"ancestor_record_offset":31108,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":49456,"parent_record_offset":49400,"ancestor_record_offset":31108,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":41412,"parent_record_offset":41380,"ancestor_record_offset":31108,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":42884,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":41596,"parent_record_offset":41572,"ancestor_record_offset":31108,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":32216,"parent_record_offset":32188,"ancestor_record_offset":31108,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":608,"record_offset":35832,"parent_record_offset":35808,"ancestor_record_offset":31108,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":608,"record_offset":31776,"parent_record_offset":31752,"ancestor_record_offset":31108,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":608,"record_offset":31860,"parent_record_offset":31836,"ancestor_record_offset":31108,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":608,"record_offset":32384,"parent_record_offset":31668,"ancestor_record_offset":31108,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":608,"record_offset":53776,"parent_record_offset":53752,"ancestor_record_offset":31108,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":608,"record_offset":49140,"parent_record_offset":49116,"ancestor_record_offset":31108,"inlinee":16013,"source_line":23,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":608,"record_offset":49660,"parent_record_offset":49636,"ancestor_record_offset":31108,"inlinee":17047,"source_line":142,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":608,"record_offset":37008,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":608,"record_offset":54712,"parent_record_offset":54688,"ancestor_record_offset":31108,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":608,"record_offset":38696,"parent_record_offset":38672,"ancestor_record_offset":31108,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":608,"record_offset":38672,"parent_record_offset":38648,"ancestor_record_offset":31108,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":49172,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":49116,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":16011,"source_line":957,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":37032,"parent_record_offset":37008,"ancestor_record_offset":31108,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":37088,"parent_record_offset":37056,"ancestor_record_offset":31108,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":49636,"parent_record_offset":49612,"ancestor_record_offset":31108,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":37056,"parent_record_offset":37032,"ancestor_record_offset":31108,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":608,"record_offset":37684,"parent_record_offset":37656,"ancestor_record_offset":31108,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":608,"record_offset":37708,"parent_record_offset":37684,"ancestor_record_offset":31108,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":608,"record_offset":33400,"parent_record_offset":33376,"ancestor_record_offset":31108,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":36760,"parent_record_offset":36732,"ancestor_record_offset":31108,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":36788,"parent_record_offset":36760,"ancestor_record_offset":31108,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":35344,"parent_record_offset":35316,"ancestor_record_offset":31108,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":42552,"parent_record_offset":42516,"ancestor_record_offset":31108,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":33564,"parent_record_offset":33508,"ancestor_record_offset":31108,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":608,"record_offset":32008,"parent_record_offset":31980,"ancestor_record_offset":31108,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":608,"record_offset":31980,"parent_record_offset":31952,"ancestor_record_offset":31108,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":608,"record_offset":37144,"parent_record_offset":37116,"ancestor_record_offset":31108,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":608,"record_offset":33144,"parent_record_offset":33108,"ancestor_record_offset":31108,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":608,"record_offset":37116,"parent_record_offset":37088,"ancestor_record_offset":31108,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":608,"record_offset":42908,"parent_record_offset":42884,"ancestor_record_offset":31108,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":608,"record_offset":33264,"parent_record_offset":33232,"ancestor_record_offset":31108,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":608,"record_offset":41488,"parent_record_offset":41444,"ancestor_record_offset":31108,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":608,"record_offset":35864,"parent_record_offset":35780,"ancestor_record_offset":31108,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":608,"record_offset":49372,"parent_record_offset":49344,"ancestor_record_offset":31108,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":608,"record_offset":53944,"parent_record_offset":53508,"ancestor_record_offset":31108,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":608,"record_offset":38792,"parent_record_offset":38768,"ancestor_record_offset":31108,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":31800,"parent_record_offset":31776,"ancestor_record_offset":31108,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=172
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":33776,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":31356,"parent_record_offset":31332,"ancestor_record_offset":31108,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":36320,"parent_record_offset":36296,"ancestor_record_offset":31108,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":38768,"parent_record_offset":38744,"ancestor_record_offset":31108,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":33352,"parent_record_offset":33232,"ancestor_record_offset":31108,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":32124,"parent_record_offset":32100,"ancestor_record_offset":31108,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":42728,"parent_record_offset":42704,"ancestor_record_offset":31108,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":31448,"parent_record_offset":31388,"ancestor_record_offset":31108,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":46200,"parent_record_offset":46168,"ancestor_record_offset":31108,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":38744,"parent_record_offset":38720,"ancestor_record_offset":31108,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":31476,"parent_record_offset":31388,"ancestor_record_offset":31108,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":31420,"parent_record_offset":31388,"ancestor_record_offset":31108,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":31500,"parent_record_offset":31476,"ancestor_record_offset":31108,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":33376,"parent_record_offset":33352,"ancestor_record_offset":31108,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":42752,"parent_record_offset":42728,"ancestor_record_offset":31108,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":608,"record_offset":33736,"parent_record_offset":33712,"ancestor_record_offset":31108,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":608,"record_offset":37172,"parent_record_offset":37144,"ancestor_record_offset":31108,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":608,"record_offset":32300,"parent_record_offset":32272,"ancestor_record_offset":31108,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":608,"record_offset":32328,"parent_record_offset":32300,"ancestor_record_offset":31108,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":42824,"parent_record_offset":42800,"ancestor_record_offset":31108,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":42848,"parent_record_offset":42824,"ancestor_record_offset":31108,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":46228,"parent_record_offset":46168,"ancestor_record_offset":31108,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":46260,"parent_record_offset":46132,"ancestor_record_offset":31108,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\tuple.rs
//     evidence={"module":608,"record_offset":41380,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":27934,"source_line":34,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::bake_mod_textures::mod_new_texture_opts
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/bake_mod_textures.rs
//     evidence={"module":608,"record_offset":32668,"parent_record_offset":32616,"ancestor_record_offset":31108,"inlinee":38711,"source_line":239,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::bake_mod_textures::plan_bakes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/bake_mod_textures.rs
//     evidence={"module":608,"record_offset":32616,"parent_record_offset":32556,"ancestor_record_offset":31108,"inlinee":38710,"source_line":115,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":608,"record_offset":31920,"parent_record_offset":31892,"ancestor_record_offset":31108,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":608,"record_offset":31952,"parent_record_offset":31920,"ancestor_record_offset":31108,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":608,"record_offset":31892,"parent_record_offset":31668,"ancestor_record_offset":31108,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":31752,"parent_record_offset":31668,"ancestor_record_offset":31108,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":35808,"parent_record_offset":35780,"ancestor_record_offset":31108,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":32412,"parent_record_offset":31668,"ancestor_record_offset":31108,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":42124,"parent_record_offset":42084,"ancestor_record_offset":31108,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":31836,"parent_record_offset":31668,"ancestor_record_offset":31108,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":608,"record_offset":31724,"parent_record_offset":31668,"ancestor_record_offset":31108,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":608,"record_offset":42676,"parent_record_offset":41848,"ancestor_record_offset":31108,"inlinee":7447,"source_line":28,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":608,"record_offset":32188,"parent_record_offset":32160,"ancestor_record_offset":31108,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":43732,"parent_record_offset":43708,"ancestor_record_offset":31108,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":31560,"parent_record_offset":31536,"ancestor_record_offset":31108,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":32160,"parent_record_offset":32052,"ancestor_record_offset":31108,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":49612,"parent_record_offset":49588,"ancestor_record_offset":31108,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":31592,"parent_record_offset":31560,"ancestor_record_offset":31108,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":43012,"parent_record_offset":42988,"ancestor_record_offset":31108,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42988,"parent_record_offset":42964,"ancestor_record_offset":31108,"inlinee":8503,"source_line":573,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36296,"parent_record_offset":35692,"ancestor_record_offset":31108,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":32100,"parent_record_offset":32076,"ancestor_record_offset":31108,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36148,"parent_record_offset":35928,"ancestor_record_offset":31108,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42704,"parent_record_offset":41848,"ancestor_record_offset":31108,"inlinee":7448,"source_line":561,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":32504,"parent_record_offset":31668,"ancestor_record_offset":31108,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35780,"parent_record_offset":35724,"ancestor_record_offset":31108,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35928,"parent_record_offset":35900,"ancestor_record_offset":31108,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":32076,"parent_record_offset":32052,"ancestor_record_offset":31108,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":31644,"parent_record_offset":31620,"ancestor_record_offset":31108,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":31620,"parent_record_offset":31560,"ancestor_record_offset":31108,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":41848,"parent_record_offset":41816,"ancestor_record_offset":31108,"inlinee":7438,"source_line":1114,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36700,"parent_record_offset":36668,"ancestor_record_offset":31108,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42592,"parent_record_offset":41848,"ancestor_record_offset":31108,"inlinee":7446,"source_line":2382,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35752,"parent_record_offset":35724,"ancestor_record_offset":31108,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35692,"parent_record_offset":35620,"ancestor_record_offset":31108,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35620,"parent_record_offset":35588,"ancestor_record_offset":31108,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":31668,"parent_record_offset":31644,"ancestor_record_offset":31108,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":41940,"parent_record_offset":41896,"ancestor_record_offset":31108,"inlinee":7440,"source_line":1992,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_index_in_group
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42084,"parent_record_offset":41940,"ancestor_record_offset":31108,"inlinee":7441,"source_line":1787,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42272,"parent_record_offset":41940,"ancestor_record_offset":31108,"inlinee":7443,"source_line":1750,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36668,"parent_record_offset":35620,"ancestor_record_offset":31108,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42308,"parent_record_offset":42272,"ancestor_record_offset":31108,"inlinee":7444,"source_line":2685,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35660,"parent_record_offset":35620,"ancestor_record_offset":31108,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35724,"parent_record_offset":35692,"ancestor_record_offset":31108,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":41896,"parent_record_offset":41848,"ancestor_record_offset":31108,"inlinee":7439,"source_line":1950,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":43924,"parent_record_offset":43840,"ancestor_record_offset":31108,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42492,"parent_record_offset":41896,"ancestor_record_offset":31108,"inlinee":7445,"source_line":2540,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36732,"parent_record_offset":36700,"ancestor_record_offset":31108,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35588,"parent_record_offset":35560,"ancestor_record_offset":31108,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35900,"parent_record_offset":35692,"ancestor_record_offset":31108,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":32052,"parent_record_offset":31668,"ancestor_record_offset":31108,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\rustc_entry.rs
//     evidence={"module":608,"record_offset":42964,"parent_record_offset":42940,"ancestor_record_offset":31108,"inlinee":38728,"source_line":307,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\rustc_entry.rs
//     evidence={"module":608,"record_offset":41816,"parent_record_offset":41784,"ancestor_record_offset":31108,"inlinee":7470,"source_line":512,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":43128,"parent_record_offset":43104,"ancestor_record_offset":31108,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":608,"record_offset":43080,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":608,"record_offset":43104,"parent_record_offset":43080,"ancestor_record_offset":31108,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":43152,"parent_record_offset":43128,"ancestor_record_offset":31108,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":42800,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":43708,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":21897,"source_line":1231,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":41312,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":8388,"source_line":975,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":39724,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":32952,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":6553,"source_line":271,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":49560,"parent_record_offset":49208,"ancestor_record_offset":31108,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":42940,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":38727,"source_line":2667,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":41784,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":8397,"source_line":2881,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$78::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":49208,"parent_record_offset":49172,"ancestor_record_offset":31108,"inlinee":12269,"source_line":2919,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$79::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":49588,"parent_record_offset":49208,"ancestor_record_offset":31108,"inlinee":12226,"source_line":2936,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":32980,"parent_record_offset":32952,"ancestor_record_offset":31108,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":41340,"parent_record_offset":41312,"ancestor_record_offset":31108,"inlinee":19384,"source_line":2976,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":608,"record_offset":33712,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":608,"record_offset":33052,"parent_record_offset":33008,"ancestor_record_offset":31108,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":608,"record_offset":33312,"parent_record_offset":33288,"ancestor_record_offset":31108,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":608,"record_offset":33172,"parent_record_offset":33144,"ancestor_record_offset":31108,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":608,"record_offset":33508,"parent_record_offset":33108,"ancestor_record_offset":31108,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":608,"record_offset":33008,"parent_record_offset":32980,"ancestor_record_offset":31108,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":608,"record_offset":37260,"parent_record_offset":32668,"ancestor_record_offset":31108,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":608,"record_offset":33200,"parent_record_offset":33172,"ancestor_record_offset":31108,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":608,"record_offset":33232,"parent_record_offset":33200,"ancestor_record_offset":31108,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":608,"record_offset":33108,"parent_record_offset":33080,"ancestor_record_offset":31108,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":608,"record_offset":33080,"parent_record_offset":33052,"ancestor_record_offset":31108,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::bake_mod_textures::plan_bakes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9736
// 首条位置: {"module":608,"inlinee":38710}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::startup::bake_mod_textures::mod_new_texture_opts
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/bake_mod_textures.rs
//     evidence={"module":608,"record_offset":32668,"parent_record_offset":32616,"ancestor_record_offset":32616,"inlinee":38711,"source_line":239,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34836,"parent_record_offset":34812,"ancestor_record_offset":32616,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34860,"parent_record_offset":34836,"ancestor_record_offset":32616,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34884,"parent_record_offset":34860,"ancestor_record_offset":32616,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34812,"parent_record_offset":34748,"ancestor_record_offset":32616,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":43424,"parent_record_offset":43392,"ancestor_record_offset":32616,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":34780,"parent_record_offset":34748,"ancestor_record_offset":32616,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":34748,"parent_record_offset":34720,"ancestor_record_offset":32616,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":43472,"parent_record_offset":43448,"ancestor_record_offset":32616,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":43448,"parent_record_offset":43424,"ancestor_record_offset":32616,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":34720,"parent_record_offset":34692,"ancestor_record_offset":32616,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":608,"record_offset":40432,"parent_record_offset":40404,"ancestor_record_offset":32616,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":608,"record_offset":46168,"parent_record_offset":46132,"ancestor_record_offset":32616,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":608,"record_offset":46132,"parent_record_offset":46108,"ancestor_record_offset":32616,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":43392,"parent_record_offset":43344,"ancestor_record_offset":32616,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":33684,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":43308,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":43344,"parent_record_offset":43308,"ancestor_record_offset":32616,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":37656,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":34560,"parent_record_offset":34536,"ancestor_record_offset":32616,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":38648,"parent_record_offset":37956,"ancestor_record_offset":32616,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":37956,"parent_record_offset":37928,"ancestor_record_offset":32616,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":38868,"parent_record_offset":38844,"ancestor_record_offset":32616,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":37772,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":38720,"parent_record_offset":38696,"ancestor_record_offset":32616,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":608,"record_offset":40460,"parent_record_offset":40432,"ancestor_record_offset":32616,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::blob::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":608,"record_offset":41748,"parent_record_offset":41724,"ancestor_record_offset":32616,"inlinee":38726,"source_line":197,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41544,"parent_record_offset":41444,"ancestor_record_offset":32616,"inlinee":38718,"source_line":34,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41632,"parent_record_offset":41444,"ancestor_record_offset":32616,"inlinee":38720,"source_line":55,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$44::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41688,"parent_record_offset":41444,"ancestor_record_offset":32616,"inlinee":38724,"source_line":82,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$56::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41660,"parent_record_offset":41444,"ancestor_record_offset":32616,"inlinee":38722,"source_line":103,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41516,"parent_record_offset":41444,"ancestor_record_offset":32616,"inlinee":38716,"source_line":9,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$67::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41444,"parent_record_offset":41412,"ancestor_record_offset":32616,"inlinee":38714,"source_line":122,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":33536,"parent_record_offset":33508,"ancestor_record_offset":32616,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":33436,"parent_record_offset":33232,"ancestor_record_offset":32616,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":49456,"parent_record_offset":49400,"ancestor_record_offset":32616,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":41412,"parent_record_offset":41380,"ancestor_record_offset":32616,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":42884,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":41596,"parent_record_offset":41572,"ancestor_record_offset":32616,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":40404,"parent_record_offset":40376,"ancestor_record_offset":32616,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":608,"record_offset":35832,"parent_record_offset":35808,"ancestor_record_offset":32616,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":608,"record_offset":39964,"parent_record_offset":39940,"ancestor_record_offset":32616,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":608,"record_offset":40048,"parent_record_offset":40024,"ancestor_record_offset":32616,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":608,"record_offset":40572,"parent_record_offset":39856,"ancestor_record_offset":32616,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":608,"record_offset":49140,"parent_record_offset":49116,"ancestor_record_offset":32616,"inlinee":16013,"source_line":23,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":608,"record_offset":49660,"parent_record_offset":49636,"ancestor_record_offset":32616,"inlinee":17047,"source_line":142,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":608,"record_offset":37008,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":608,"record_offset":38696,"parent_record_offset":38672,"ancestor_record_offset":32616,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":608,"record_offset":38672,"parent_record_offset":38648,"ancestor_record_offset":32616,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":49172,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":49116,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":16011,"source_line":957,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":37032,"parent_record_offset":37008,"ancestor_record_offset":32616,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":37088,"parent_record_offset":37056,"ancestor_record_offset":32616,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":49636,"parent_record_offset":49612,"ancestor_record_offset":32616,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":37056,"parent_record_offset":37032,"ancestor_record_offset":32616,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":608,"record_offset":37684,"parent_record_offset":37656,"ancestor_record_offset":32616,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":608,"record_offset":37708,"parent_record_offset":37684,"ancestor_record_offset":32616,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":608,"record_offset":33400,"parent_record_offset":33376,"ancestor_record_offset":32616,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":36760,"parent_record_offset":36732,"ancestor_record_offset":32616,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":36788,"parent_record_offset":36760,"ancestor_record_offset":32616,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":35344,"parent_record_offset":35316,"ancestor_record_offset":32616,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":42552,"parent_record_offset":42516,"ancestor_record_offset":32616,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":33564,"parent_record_offset":33508,"ancestor_record_offset":32616,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":608,"record_offset":40196,"parent_record_offset":40168,"ancestor_record_offset":32616,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":608,"record_offset":36052,"parent_record_offset":36028,"ancestor_record_offset":32616,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":608,"record_offset":37144,"parent_record_offset":37116,"ancestor_record_offset":32616,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":608,"record_offset":33144,"parent_record_offset":33108,"ancestor_record_offset":32616,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":608,"record_offset":37116,"parent_record_offset":37088,"ancestor_record_offset":32616,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":608,"record_offset":42908,"parent_record_offset":42884,"ancestor_record_offset":32616,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":608,"record_offset":33264,"parent_record_offset":33232,"ancestor_record_offset":32616,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":608,"record_offset":41488,"parent_record_offset":41444,"ancestor_record_offset":32616,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":608,"record_offset":35864,"parent_record_offset":35780,"ancestor_record_offset":32616,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":608,"record_offset":49372,"parent_record_offset":49344,"ancestor_record_offset":32616,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":608,"record_offset":38792,"parent_record_offset":38768,"ancestor_record_offset":32616,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":39988,"parent_record_offset":39964,"ancestor_record_offset":32616,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=135
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":33776,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":37740,"parent_record_offset":37656,"ancestor_record_offset":32616,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":36320,"parent_record_offset":36296,"ancestor_record_offset":32616,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":38768,"parent_record_offset":38744,"ancestor_record_offset":32616,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":33352,"parent_record_offset":33232,"ancestor_record_offset":32616,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":36172,"parent_record_offset":36148,"ancestor_record_offset":32616,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":42728,"parent_record_offset":42704,"ancestor_record_offset":32616,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":37840,"parent_record_offset":37772,"ancestor_record_offset":32616,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":46200,"parent_record_offset":46168,"ancestor_record_offset":32616,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":38744,"parent_record_offset":38720,"ancestor_record_offset":32616,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":37868,"parent_record_offset":37772,"ancestor_record_offset":32616,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":37808,"parent_record_offset":37772,"ancestor_record_offset":32616,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":37892,"parent_record_offset":37868,"ancestor_record_offset":32616,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":33376,"parent_record_offset":33352,"ancestor_record_offset":32616,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":42752,"parent_record_offset":42728,"ancestor_record_offset":32616,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":608,"record_offset":33736,"parent_record_offset":33712,"ancestor_record_offset":32616,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":608,"record_offset":37172,"parent_record_offset":37144,"ancestor_record_offset":32616,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":608,"record_offset":40488,"parent_record_offset":40460,"ancestor_record_offset":32616,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":608,"record_offset":40516,"parent_record_offset":40488,"ancestor_record_offset":32616,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":42824,"parent_record_offset":42800,"ancestor_record_offset":32616,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":42848,"parent_record_offset":42824,"ancestor_record_offset":32616,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":46228,"parent_record_offset":46168,"ancestor_record_offset":32616,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":46260,"parent_record_offset":46132,"ancestor_record_offset":32616,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\tuple.rs
//     evidence={"module":608,"record_offset":41380,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":27934,"source_line":34,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":608,"record_offset":35996,"parent_record_offset":35968,"ancestor_record_offset":32616,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":608,"record_offset":36028,"parent_record_offset":35996,"ancestor_record_offset":32616,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":608,"record_offset":35968,"parent_record_offset":35928,"ancestor_record_offset":32616,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":39940,"parent_record_offset":39856,"ancestor_record_offset":32616,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":35808,"parent_record_offset":35780,"ancestor_record_offset":32616,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":40600,"parent_record_offset":39856,"ancestor_record_offset":32616,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":42124,"parent_record_offset":42084,"ancestor_record_offset":32616,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":40024,"parent_record_offset":39856,"ancestor_record_offset":32616,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":608,"record_offset":39912,"parent_record_offset":39856,"ancestor_record_offset":32616,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":608,"record_offset":42676,"parent_record_offset":41848,"ancestor_record_offset":32616,"inlinee":7447,"source_line":28,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":608,"record_offset":40376,"parent_record_offset":40348,"ancestor_record_offset":32616,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":43732,"parent_record_offset":43708,"ancestor_record_offset":32616,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":39748,"parent_record_offset":39724,"ancestor_record_offset":32616,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":40348,"parent_record_offset":40240,"ancestor_record_offset":32616,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":49612,"parent_record_offset":49588,"ancestor_record_offset":32616,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":39780,"parent_record_offset":39748,"ancestor_record_offset":32616,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":43012,"parent_record_offset":42988,"ancestor_record_offset":32616,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42988,"parent_record_offset":42964,"ancestor_record_offset":32616,"inlinee":8503,"source_line":573,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36296,"parent_record_offset":35692,"ancestor_record_offset":32616,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":40288,"parent_record_offset":40264,"ancestor_record_offset":32616,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36148,"parent_record_offset":35928,"ancestor_record_offset":32616,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42704,"parent_record_offset":41848,"ancestor_record_offset":32616,"inlinee":7448,"source_line":561,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":40692,"parent_record_offset":39856,"ancestor_record_offset":32616,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35780,"parent_record_offset":35724,"ancestor_record_offset":32616,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35928,"parent_record_offset":35900,"ancestor_record_offset":32616,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":40264,"parent_record_offset":40240,"ancestor_record_offset":32616,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":39832,"parent_record_offset":39808,"ancestor_record_offset":32616,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":39808,"parent_record_offset":39748,"ancestor_record_offset":32616,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":41848,"parent_record_offset":41816,"ancestor_record_offset":32616,"inlinee":7438,"source_line":1114,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36700,"parent_record_offset":36668,"ancestor_record_offset":32616,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42592,"parent_record_offset":41848,"ancestor_record_offset":32616,"inlinee":7446,"source_line":2382,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35752,"parent_record_offset":35724,"ancestor_record_offset":32616,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35692,"parent_record_offset":35620,"ancestor_record_offset":32616,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35620,"parent_record_offset":35588,"ancestor_record_offset":32616,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":39856,"parent_record_offset":39832,"ancestor_record_offset":32616,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":41940,"parent_record_offset":41896,"ancestor_record_offset":32616,"inlinee":7440,"source_line":1992,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_index_in_group
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42084,"parent_record_offset":41940,"ancestor_record_offset":32616,"inlinee":7441,"source_line":1787,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42272,"parent_record_offset":41940,"ancestor_record_offset":32616,"inlinee":7443,"source_line":1750,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36668,"parent_record_offset":35620,"ancestor_record_offset":32616,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42308,"parent_record_offset":42272,"ancestor_record_offset":32616,"inlinee":7444,"source_line":2685,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35660,"parent_record_offset":35620,"ancestor_record_offset":32616,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35724,"parent_record_offset":35692,"ancestor_record_offset":32616,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":41896,"parent_record_offset":41848,"ancestor_record_offset":32616,"inlinee":7439,"source_line":1950,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":43924,"parent_record_offset":43840,"ancestor_record_offset":32616,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42492,"parent_record_offset":41896,"ancestor_record_offset":32616,"inlinee":7445,"source_line":2540,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36732,"parent_record_offset":36700,"ancestor_record_offset":32616,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35588,"parent_record_offset":35560,"ancestor_record_offset":32616,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35900,"parent_record_offset":35692,"ancestor_record_offset":32616,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":40240,"parent_record_offset":39856,"ancestor_record_offset":32616,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\rustc_entry.rs
//     evidence={"module":608,"record_offset":42964,"parent_record_offset":42940,"ancestor_record_offset":32616,"inlinee":38728,"source_line":307,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\rustc_entry.rs
//     evidence={"module":608,"record_offset":41816,"parent_record_offset":41784,"ancestor_record_offset":32616,"inlinee":7470,"source_line":512,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":43128,"parent_record_offset":43104,"ancestor_record_offset":32616,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":608,"record_offset":43080,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":608,"record_offset":43104,"parent_record_offset":43080,"ancestor_record_offset":32616,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":43152,"parent_record_offset":43128,"ancestor_record_offset":32616,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":42800,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":43708,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":21897,"source_line":1231,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":41312,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":8388,"source_line":975,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":39724,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":32952,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":6553,"source_line":271,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":49560,"parent_record_offset":49208,"ancestor_record_offset":32616,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":42940,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":38727,"source_line":2667,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":41784,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":8397,"source_line":2881,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$78::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":49208,"parent_record_offset":49172,"ancestor_record_offset":32616,"inlinee":12269,"source_line":2919,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$79::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":49588,"parent_record_offset":49208,"ancestor_record_offset":32616,"inlinee":12226,"source_line":2936,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":32980,"parent_record_offset":32952,"ancestor_record_offset":32616,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":41340,"parent_record_offset":41312,"ancestor_record_offset":32616,"inlinee":19384,"source_line":2976,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":608,"record_offset":33712,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":608,"record_offset":33052,"parent_record_offset":33008,"ancestor_record_offset":32616,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":608,"record_offset":33312,"parent_record_offset":33288,"ancestor_record_offset":32616,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":608,"record_offset":33172,"parent_record_offset":33144,"ancestor_record_offset":32616,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":608,"record_offset":33508,"parent_record_offset":33108,"ancestor_record_offset":32616,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":608,"record_offset":33008,"parent_record_offset":32980,"ancestor_record_offset":32616,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":608,"record_offset":37260,"parent_record_offset":32668,"ancestor_record_offset":32616,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":608,"record_offset":33200,"parent_record_offset":33172,"ancestor_record_offset":32616,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":608,"record_offset":33232,"parent_record_offset":33200,"ancestor_record_offset":32616,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":608,"record_offset":33108,"parent_record_offset":33080,"ancestor_record_offset":32616,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":608,"record_offset":33080,"parent_record_offset":33052,"ancestor_record_offset":32616,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::bake_mod_textures::bake_recipe
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9738
// 首条位置: {"module":608,"inlinee":38712}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":608,"record_offset":53364,"parent_record_offset":53264,"ancestor_record_offset":53264,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":54608,"parent_record_offset":53264,"ancestor_record_offset":53264,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":54664,"parent_record_offset":53264,"ancestor_record_offset":53264,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":54748,"parent_record_offset":53264,"ancestor_record_offset":53264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":53840,"parent_record_offset":53816,"ancestor_record_offset":53264,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":53864,"parent_record_offset":53840,"ancestor_record_offset":53264,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":54996,"parent_record_offset":54972,"ancestor_record_offset":53264,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":55020,"parent_record_offset":54996,"ancestor_record_offset":53264,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":53888,"parent_record_offset":53864,"ancestor_record_offset":53264,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":55044,"parent_record_offset":55020,"ancestor_record_offset":53264,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":53816,"parent_record_offset":53660,"ancestor_record_offset":53264,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":54972,"parent_record_offset":54916,"ancestor_record_offset":53264,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":53584,"parent_record_offset":53548,"ancestor_record_offset":53264,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":54944,"parent_record_offset":54916,"ancestor_record_offset":53264,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":54916,"parent_record_offset":54892,"ancestor_record_offset":53264,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":53660,"parent_record_offset":53620,"ancestor_record_offset":53264,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":53620,"parent_record_offset":53584,"ancestor_record_offset":53264,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":54892,"parent_record_offset":54868,"ancestor_record_offset":53264,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":53704,"parent_record_offset":53660,"ancestor_record_offset":53264,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":608,"record_offset":53436,"parent_record_offset":53400,"ancestor_record_offset":53264,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":608,"record_offset":53472,"parent_record_offset":53436,"ancestor_record_offset":53264,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":608,"record_offset":53508,"parent_record_offset":53472,"ancestor_record_offset":53264,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":608,"record_offset":53400,"parent_record_offset":53364,"ancestor_record_offset":53264,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":53548,"parent_record_offset":53508,"ancestor_record_offset":53264,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":54688,"parent_record_offset":54664,"ancestor_record_offset":53264,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":608,"record_offset":53728,"parent_record_offset":53704,"ancestor_record_offset":53264,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":608,"record_offset":53776,"parent_record_offset":53752,"ancestor_record_offset":53264,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":608,"record_offset":54712,"parent_record_offset":54688,"ancestor_record_offset":53264,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":53752,"parent_record_offset":53728,"ancestor_record_offset":53264,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":608,"record_offset":53944,"parent_record_offset":53508,"ancestor_record_offset":53264,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":53968,"parent_record_offset":53944,"ancestor_record_offset":53264,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":54772,"parent_record_offset":54748,"ancestor_record_offset":53264,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":54632,"parent_record_offset":54608,"ancestor_record_offset":53264,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::bake_mod_textures::bake_recipe::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15921
// 首条位置: {"module":1822,"inlinee":88353}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1822,"record_offset":24736,"parent_record_offset":24688,"ancestor_record_offset":24688,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1822,"record_offset":25208,"parent_record_offset":24688,"ancestor_record_offset":24688,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":25028,"parent_record_offset":25004,"ancestor_record_offset":24688,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":25052,"parent_record_offset":25028,"ancestor_record_offset":24688,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":25456,"parent_record_offset":25432,"ancestor_record_offset":24688,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":25480,"parent_record_offset":25456,"ancestor_record_offset":24688,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":25076,"parent_record_offset":25052,"ancestor_record_offset":24688,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":25504,"parent_record_offset":25480,"ancestor_record_offset":24688,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":25004,"parent_record_offset":24968,"ancestor_record_offset":24688,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":25432,"parent_record_offset":25376,"ancestor_record_offset":24688,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":24908,"parent_record_offset":24880,"ancestor_record_offset":24688,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":25404,"parent_record_offset":25376,"ancestor_record_offset":24688,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":25376,"parent_record_offset":25352,"ancestor_record_offset":24688,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":24968,"parent_record_offset":24936,"ancestor_record_offset":24688,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":24936,"parent_record_offset":24908,"ancestor_record_offset":24688,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":25352,"parent_record_offset":25328,"ancestor_record_offset":24688,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1822,"record_offset":24792,"parent_record_offset":24764,"ancestor_record_offset":24688,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1822,"record_offset":24820,"parent_record_offset":24792,"ancestor_record_offset":24688,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1822,"record_offset":24848,"parent_record_offset":24820,"ancestor_record_offset":24688,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1822,"record_offset":24764,"parent_record_offset":24736,"ancestor_record_offset":24688,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1822,"record_offset":24880,"parent_record_offset":24848,"ancestor_record_offset":24688,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1822,"record_offset":25132,"parent_record_offset":24848,"ancestor_record_offset":24688,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1822,"record_offset":25156,"parent_record_offset":25132,"ancestor_record_offset":24688,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1822,"record_offset":25232,"parent_record_offset":25208,"ancestor_record_offset":24688,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::bake_mod_textures::bake_all::async_fn$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13EA5
// 首条位置: {"module":1647,"inlinee":81573}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1647,"record_offset":23644,"parent_record_offset":23424,"ancestor_record_offset":23424,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":24772,"parent_record_offset":23424,"ancestor_record_offset":23424,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":24156,"parent_record_offset":23424,"ancestor_record_offset":23424,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":24560,"parent_record_offset":23424,"ancestor_record_offset":23424,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1647,"record_offset":24616,"parent_record_offset":23424,"ancestor_record_offset":23424,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::future::future::impl$1::poll
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\future\future.rs
//     evidence={"module":1647,"record_offset":26580,"parent_record_offset":23424,"ancestor_record_offset":23424,"inlinee":5392,"source_line":132,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1647,"record_offset":25652,"parent_record_offset":23424,"ancestor_record_offset":23424,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1647,"record_offset":27224,"parent_record_offset":23424,"ancestor_record_offset":23424,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1647,"record_offset":27252,"parent_record_offset":23424,"ancestor_record_offset":23424,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::BuildContext::with_spawner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":1647,"record_offset":23732,"parent_record_offset":23424,"ancestor_record_offset":23424,"inlinee":81575,"source_line":668,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":23896,"parent_record_offset":23872,"ancestor_record_offset":23424,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":23920,"parent_record_offset":23896,"ancestor_record_offset":23424,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":26132,"parent_record_offset":26108,"ancestor_record_offset":23424,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":26156,"parent_record_offset":26132,"ancestor_record_offset":23424,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":23944,"parent_record_offset":23920,"ancestor_record_offset":23424,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":26180,"parent_record_offset":26156,"ancestor_record_offset":23424,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":23872,"parent_record_offset":23840,"ancestor_record_offset":23424,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":26108,"parent_record_offset":26012,"ancestor_record_offset":23424,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1647,"record_offset":23840,"parent_record_offset":23804,"ancestor_record_offset":23424,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1647,"record_offset":23804,"parent_record_offset":23776,"ancestor_record_offset":23424,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1647,"record_offset":26720,"parent_record_offset":26692,"ancestor_record_offset":23424,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":24888,"parent_record_offset":24856,"ancestor_record_offset":23424,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":24220,"parent_record_offset":24188,"ancestor_record_offset":23424,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":26044,"parent_record_offset":26012,"ancestor_record_offset":23424,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":26012,"parent_record_offset":25984,"ancestor_record_offset":23424,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":24936,"parent_record_offset":24912,"ancestor_record_offset":23424,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":24912,"parent_record_offset":24888,"ancestor_record_offset":23424,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":24280,"parent_record_offset":24248,"ancestor_record_offset":23424,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":24248,"parent_record_offset":24220,"ancestor_record_offset":23424,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":25984,"parent_record_offset":25956,"ancestor_record_offset":23424,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":24316,"parent_record_offset":24280,"ancestor_record_offset":23424,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1647,"record_offset":23776,"parent_record_offset":23732,"ancestor_record_offset":23424,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1647,"record_offset":24024,"parent_record_offset":24000,"ancestor_record_offset":23424,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":24856,"parent_record_offset":24808,"ancestor_record_offset":23424,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":24808,"parent_record_offset":24772,"ancestor_record_offset":23424,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":24188,"parent_record_offset":24156,"ancestor_record_offset":23424,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1647,"record_offset":25712,"parent_record_offset":25680,"ancestor_record_offset":23424,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1647,"record_offset":25680,"parent_record_offset":25652,"ancestor_record_offset":23424,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1647,"record_offset":25932,"parent_record_offset":25908,"ancestor_record_offset":23424,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1647,"record_offset":25784,"parent_record_offset":25760,"ancestor_record_offset":23424,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1647,"record_offset":26752,"parent_record_offset":26720,"ancestor_record_offset":23424,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1647,"record_offset":24340,"parent_record_offset":24316,"ancestor_record_offset":23424,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1647,"record_offset":25760,"parent_record_offset":25736,"ancestor_record_offset":23424,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1647,"record_offset":25736,"parent_record_offset":25712,"ancestor_record_offset":23424,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1647,"record_offset":26816,"parent_record_offset":26780,"ancestor_record_offset":23424,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1647,"record_offset":26780,"parent_record_offset":26752,"ancestor_record_offset":23424,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1647,"record_offset":24364,"parent_record_offset":24340,"ancestor_record_offset":23424,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1647,"record_offset":24388,"parent_record_offset":24364,"ancestor_record_offset":23424,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1647,"record_offset":26076,"parent_record_offset":26044,"ancestor_record_offset":23424,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1647,"record_offset":27280,"parent_record_offset":27252,"ancestor_record_offset":23424,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":1647,"record_offset":26604,"parent_record_offset":26580,"ancestor_record_offset":23424,"inlinee":5375,"source_line":1428,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":1647,"record_offset":26628,"parent_record_offset":26604,"ancestor_record_offset":23424,"inlinee":5376,"source_line":1409,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1647,"record_offset":25856,"parent_record_offset":25832,"ancestor_record_offset":23424,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1647,"record_offset":24000,"parent_record_offset":23732,"ancestor_record_offset":23424,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1647,"record_offset":24584,"parent_record_offset":24560,"ancestor_record_offset":23424,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1647,"record_offset":25832,"parent_record_offset":25808,"ancestor_record_offset":23424,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1647,"record_offset":24684,"parent_record_offset":24616,"ancestor_record_offset":23424,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1647,"record_offset":25808,"parent_record_offset":25784,"ancestor_record_offset":23424,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1647,"record_offset":24712,"parent_record_offset":24616,"ancestor_record_offset":23424,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1647,"record_offset":24652,"parent_record_offset":24616,"ancestor_record_offset":23424,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1647,"record_offset":24736,"parent_record_offset":24712,"ancestor_record_offset":23424,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1647,"record_offset":24976,"parent_record_offset":24808,"ancestor_record_offset":23424,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":23672,"parent_record_offset":23644,"ancestor_record_offset":23424,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":24060,"parent_record_offset":24024,"ancestor_record_offset":23424,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":23696,"parent_record_offset":23672,"ancestor_record_offset":23424,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":24084,"parent_record_offset":24060,"ancestor_record_offset":23424,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":24116,"parent_record_offset":24024,"ancestor_record_offset":23424,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::bake_mod_textures::bake_all::async_fn$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F57
// 首条位置: {"module":1651,"inlinee":81751}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_tasks::task::Task::detach
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/task.rs
//     evidence={"module":1651,"record_offset":314632,"parent_record_offset":314600,"ancestor_record_offset":314600,"inlinee":35016,"source_line":63,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ptr::drop_in_place<enum2$<country_core::startup::bake_mod_textures::bake_all::async_fn_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 195
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1749,"record_offset":16036,"rva":27810064,"va":5396519184}
// 编译实例 RVAs: 0x1a85910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-20840590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396519184,"raw_hex":"e872ffc1fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396519289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140983c70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17833363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396519184,"raw_hex":"e86de2effe","size":5,"soft_fail":false,"target_va":5378686064,"va":5396519422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098b260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17803119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396519184,"raw_hex":"e89158f0fe","size":5,"soft_fail":false,"target_va":5378716256,"va":5396519370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098b760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17802058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396519184,"raw_hex":"e8b65cf0fe","size":5,"soft_fail":false,"target_va":5378717536,"va":5396519589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990020
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17783288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396519184,"raw_hex":"e808a6f0fe","size":5,"soft_fail":false,"target_va":5378736160,"va":5396519443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140972c40
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-17903094","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5396519184,"raw_hex":"e90ad2eefe","size":5,"soft_fail":false,"target_va":5378616384,"va":5396519473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5396519184,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5396519265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1749,"record_offset":16224,"parent_record_offset":16036,"ancestor_record_offset":16036,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1749,"record_offset":16428,"parent_record_offset":16404,"ancestor_record_offset":16036,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1749,"record_offset":16452,"parent_record_offset":16428,"ancestor_record_offset":16036,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1749,"record_offset":16476,"parent_record_offset":16452,"ancestor_record_offset":16036,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1749,"record_offset":16404,"parent_record_offset":16280,"ancestor_record_offset":16036,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1749,"record_offset":16280,"parent_record_offset":16252,"ancestor_record_offset":16036,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1749,"record_offset":16888,"parent_record_offset":16860,"ancestor_record_offset":16036,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1749,"record_offset":16860,"parent_record_offset":16836,"ancestor_record_offset":16036,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1749,"record_offset":16836,"parent_record_offset":16816,"ancestor_record_offset":16036,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1749,"record_offset":17108,"parent_record_offset":17076,"ancestor_record_offset":16036,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1749,"record_offset":17076,"parent_record_offset":17048,"ancestor_record_offset":16036,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1749,"record_offset":17328,"parent_record_offset":17304,"ancestor_record_offset":16036,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1749,"record_offset":17180,"parent_record_offset":17156,"ancestor_record_offset":16036,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1749,"record_offset":16308,"parent_record_offset":16280,"ancestor_record_offset":16036,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1749,"record_offset":17156,"parent_record_offset":17132,"ancestor_record_offset":16036,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1749,"record_offset":17132,"parent_record_offset":17108,"ancestor_record_offset":16036,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1749,"record_offset":16368,"parent_record_offset":16336,"ancestor_record_offset":16036,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1749,"record_offset":16336,"parent_record_offset":16308,"ancestor_record_offset":16036,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1749,"record_offset":17472,"parent_record_offset":17440,"ancestor_record_offset":16036,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1749,"record_offset":17252,"parent_record_offset":17228,"ancestor_record_offset":16036,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1749,"record_offset":16252,"parent_record_offset":16224,"ancestor_record_offset":16036,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1749,"record_offset":17228,"parent_record_offset":17204,"ancestor_record_offset":16036,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1749,"record_offset":17204,"parent_record_offset":17180,"ancestor_record_offset":16036,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::bake_mod_textures::upload_baked_texture
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x974B
// 首条位置: {"module":608,"inlinee":38731}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::bake_mod_textures::mod_new_texture_opts
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 239
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9737
// 首条位置: {"module":608,"inlinee":38711}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":33684,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":43308,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":37656,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":37772,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":42884,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":608,"record_offset":37008,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":6653,"source_line":63,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":49172,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::filter_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":49116,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":16011,"source_line":957,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":33776,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::tuple::impl$10::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\tuple.rs
//     evidence={"module":608,"record_offset":41380,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":27934,"source_line":34,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":608,"record_offset":43080,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":42800,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":43708,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":21897,"source_line":1231,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":41312,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":8388,"source_line":975,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":39724,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":32952,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":6553,"source_line":271,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::OccupiedEntry::key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":42940,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":38727,"source_line":2667,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":41784,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":8397,"source_line":2881,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_dir
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":608,"record_offset":33712,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":13252,"source_line":3311,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":608,"record_offset":37260,"parent_record_offset":32668,"ancestor_record_offset":32668,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34836,"parent_record_offset":34812,"ancestor_record_offset":32668,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34860,"parent_record_offset":34836,"ancestor_record_offset":32668,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34884,"parent_record_offset":34860,"ancestor_record_offset":32668,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":34812,"parent_record_offset":34748,"ancestor_record_offset":32668,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":43424,"parent_record_offset":43392,"ancestor_record_offset":32668,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":34780,"parent_record_offset":34748,"ancestor_record_offset":32668,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":34748,"parent_record_offset":34720,"ancestor_record_offset":32668,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":43472,"parent_record_offset":43448,"ancestor_record_offset":32668,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":43448,"parent_record_offset":43424,"ancestor_record_offset":32668,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":34720,"parent_record_offset":34692,"ancestor_record_offset":32668,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":608,"record_offset":40432,"parent_record_offset":40404,"ancestor_record_offset":32668,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":608,"record_offset":46168,"parent_record_offset":46132,"ancestor_record_offset":32668,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":608,"record_offset":46132,"parent_record_offset":46108,"ancestor_record_offset":32668,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":43392,"parent_record_offset":43344,"ancestor_record_offset":32668,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":43344,"parent_record_offset":43308,"ancestor_record_offset":32668,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":34560,"parent_record_offset":34536,"ancestor_record_offset":32668,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":38648,"parent_record_offset":37956,"ancestor_record_offset":32668,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":37956,"parent_record_offset":37928,"ancestor_record_offset":32668,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":38868,"parent_record_offset":38844,"ancestor_record_offset":32668,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":608,"record_offset":38720,"parent_record_offset":38696,"ancestor_record_offset":32668,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":608,"record_offset":40460,"parent_record_offset":40432,"ancestor_record_offset":32668,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::blob::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/blob.rs
//     evidence={"module":608,"record_offset":41748,"parent_record_offset":41724,"ancestor_record_offset":32668,"inlinee":38726,"source_line":197,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$20::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41544,"parent_record_offset":41444,"ancestor_record_offset":32668,"inlinee":38718,"source_line":34,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41632,"parent_record_offset":41444,"ancestor_record_offset":32668,"inlinee":38720,"source_line":55,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$44::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41688,"parent_record_offset":41444,"ancestor_record_offset":32668,"inlinee":38724,"source_line":82,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$56::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41660,"parent_record_offset":41444,"ancestor_record_offset":32668,"inlinee":38722,"source_line":103,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41516,"parent_record_offset":41444,"ancestor_record_offset":32668,"inlinee":38716,"source_line":9,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] asset::texture::desc::impl$67::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture/desc.rs
//     evidence={"module":608,"record_offset":41444,"parent_record_offset":41412,"ancestor_record_offset":32668,"inlinee":38714,"source_line":122,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":33536,"parent_record_offset":33508,"ancestor_record_offset":32668,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":33436,"parent_record_offset":33232,"ancestor_record_offset":32668,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":608,"record_offset":49456,"parent_record_offset":49400,"ancestor_record_offset":32668,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":41412,"parent_record_offset":41380,"ancestor_record_offset":32668,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":41596,"parent_record_offset":41572,"ancestor_record_offset":32668,"inlinee":8139,"source_line":1878,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":40404,"parent_record_offset":40376,"ancestor_record_offset":32668,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":608,"record_offset":35832,"parent_record_offset":35808,"ancestor_record_offset":32668,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":608,"record_offset":39964,"parent_record_offset":39940,"ancestor_record_offset":32668,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":608,"record_offset":40048,"parent_record_offset":40024,"ancestor_record_offset":32668,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":608,"record_offset":40572,"parent_record_offset":39856,"ancestor_record_offset":32668,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::FilterMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":608,"record_offset":49140,"parent_record_offset":49116,"ancestor_record_offset":32668,"inlinee":16013,"source_line":23,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":608,"record_offset":49660,"parent_record_offset":49636,"ancestor_record_offset":32668,"inlinee":17047,"source_line":142,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":608,"record_offset":38696,"parent_record_offset":38672,"ancestor_record_offset":32668,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":608,"record_offset":38672,"parent_record_offset":38648,"ancestor_record_offset":32668,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":37032,"parent_record_offset":37008,"ancestor_record_offset":32668,"inlinee":6654,"source_line":2969,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":37088,"parent_record_offset":37056,"ancestor_record_offset":32668,"inlinee":9022,"source_line":2976,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":49636,"parent_record_offset":49612,"ancestor_record_offset":32668,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":37056,"parent_record_offset":37032,"ancestor_record_offset":32668,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":608,"record_offset":37684,"parent_record_offset":37656,"ancestor_record_offset":32668,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":608,"record_offset":37708,"parent_record_offset":37684,"ancestor_record_offset":32668,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":608,"record_offset":33400,"parent_record_offset":33376,"ancestor_record_offset":32668,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":36760,"parent_record_offset":36732,"ancestor_record_offset":32668,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":36788,"parent_record_offset":36760,"ancestor_record_offset":32668,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":35344,"parent_record_offset":35316,"ancestor_record_offset":32668,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":42552,"parent_record_offset":42516,"ancestor_record_offset":32668,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":33564,"parent_record_offset":33508,"ancestor_record_offset":32668,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":608,"record_offset":40196,"parent_record_offset":40168,"ancestor_record_offset":32668,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":608,"record_offset":36052,"parent_record_offset":36028,"ancestor_record_offset":32668,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":608,"record_offset":37144,"parent_record_offset":37116,"ancestor_record_offset":32668,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":608,"record_offset":33144,"parent_record_offset":33108,"ancestor_record_offset":32668,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":608,"record_offset":37116,"parent_record_offset":37088,"ancestor_record_offset":32668,"inlinee":4214,"source_line":297,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":608,"record_offset":42908,"parent_record_offset":42884,"ancestor_record_offset":32668,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":608,"record_offset":33264,"parent_record_offset":33232,"ancestor_record_offset":32668,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":608,"record_offset":41488,"parent_record_offset":41444,"ancestor_record_offset":32668,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":608,"record_offset":35864,"parent_record_offset":35780,"ancestor_record_offset":32668,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":608,"record_offset":49372,"parent_record_offset":49344,"ancestor_record_offset":32668,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":608,"record_offset":38792,"parent_record_offset":38768,"ancestor_record_offset":32668,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":39988,"parent_record_offset":39964,"ancestor_record_offset":32668,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=108
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":34584,"parent_record_offset":34560,"ancestor_record_offset":32668,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":37740,"parent_record_offset":37656,"ancestor_record_offset":32668,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":36320,"parent_record_offset":36296,"ancestor_record_offset":32668,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":38768,"parent_record_offset":38744,"ancestor_record_offset":32668,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":33352,"parent_record_offset":33232,"ancestor_record_offset":32668,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":36172,"parent_record_offset":36148,"ancestor_record_offset":32668,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":608,"record_offset":42728,"parent_record_offset":42704,"ancestor_record_offset":32668,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":37840,"parent_record_offset":37772,"ancestor_record_offset":32668,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":46200,"parent_record_offset":46168,"ancestor_record_offset":32668,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":38744,"parent_record_offset":38720,"ancestor_record_offset":32668,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":37868,"parent_record_offset":37772,"ancestor_record_offset":32668,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":608,"record_offset":37808,"parent_record_offset":37772,"ancestor_record_offset":32668,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":37892,"parent_record_offset":37868,"ancestor_record_offset":32668,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":33376,"parent_record_offset":33352,"ancestor_record_offset":32668,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":42752,"parent_record_offset":42728,"ancestor_record_offset":32668,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":608,"record_offset":33736,"parent_record_offset":33712,"ancestor_record_offset":32668,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":608,"record_offset":37172,"parent_record_offset":37144,"ancestor_record_offset":32668,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":608,"record_offset":40488,"parent_record_offset":40460,"ancestor_record_offset":32668,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":608,"record_offset":40516,"parent_record_offset":40488,"ancestor_record_offset":32668,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":42824,"parent_record_offset":42800,"ancestor_record_offset":32668,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":42848,"parent_record_offset":42824,"ancestor_record_offset":32668,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":46228,"parent_record_offset":46168,"ancestor_record_offset":32668,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":46260,"parent_record_offset":46132,"ancestor_record_offset":32668,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":608,"record_offset":35996,"parent_record_offset":35968,"ancestor_record_offset":32668,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":608,"record_offset":36028,"parent_record_offset":35996,"ancestor_record_offset":32668,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":608,"record_offset":35968,"parent_record_offset":35928,"ancestor_record_offset":32668,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":39940,"parent_record_offset":39856,"ancestor_record_offset":32668,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":35808,"parent_record_offset":35780,"ancestor_record_offset":32668,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":40600,"parent_record_offset":39856,"ancestor_record_offset":32668,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":42124,"parent_record_offset":42084,"ancestor_record_offset":32668,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":608,"record_offset":40024,"parent_record_offset":39856,"ancestor_record_offset":32668,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":608,"record_offset":39912,"parent_record_offset":39856,"ancestor_record_offset":32668,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":608,"record_offset":42676,"parent_record_offset":41848,"ancestor_record_offset":32668,"inlinee":7447,"source_line":28,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":608,"record_offset":40376,"parent_record_offset":40348,"ancestor_record_offset":32668,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":43732,"parent_record_offset":43708,"ancestor_record_offset":32668,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":39748,"parent_record_offset":39724,"ancestor_record_offset":32668,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":40348,"parent_record_offset":40240,"ancestor_record_offset":32668,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":49612,"parent_record_offset":49588,"ancestor_record_offset":32668,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":608,"record_offset":39780,"parent_record_offset":39748,"ancestor_record_offset":32668,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":43012,"parent_record_offset":42988,"ancestor_record_offset":32668,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42988,"parent_record_offset":42964,"ancestor_record_offset":32668,"inlinee":8503,"source_line":573,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36296,"parent_record_offset":35692,"ancestor_record_offset":32668,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":40288,"parent_record_offset":40264,"ancestor_record_offset":32668,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36148,"parent_record_offset":35928,"ancestor_record_offset":32668,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42704,"parent_record_offset":41848,"ancestor_record_offset":32668,"inlinee":7448,"source_line":561,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":40692,"parent_record_offset":39856,"ancestor_record_offset":32668,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35780,"parent_record_offset":35724,"ancestor_record_offset":32668,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35928,"parent_record_offset":35900,"ancestor_record_offset":32668,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":40264,"parent_record_offset":40240,"ancestor_record_offset":32668,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":39832,"parent_record_offset":39808,"ancestor_record_offset":32668,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":39808,"parent_record_offset":39748,"ancestor_record_offset":32668,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":41848,"parent_record_offset":41816,"ancestor_record_offset":32668,"inlinee":7438,"source_line":1114,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36700,"parent_record_offset":36668,"ancestor_record_offset":32668,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42592,"parent_record_offset":41848,"ancestor_record_offset":32668,"inlinee":7446,"source_line":2382,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35752,"parent_record_offset":35724,"ancestor_record_offset":32668,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35692,"parent_record_offset":35620,"ancestor_record_offset":32668,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35620,"parent_record_offset":35588,"ancestor_record_offset":32668,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":39856,"parent_record_offset":39832,"ancestor_record_offset":32668,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":41940,"parent_record_offset":41896,"ancestor_record_offset":32668,"inlinee":7440,"source_line":1992,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_index_in_group
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42084,"parent_record_offset":41940,"ancestor_record_offset":32668,"inlinee":7441,"source_line":1787,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42272,"parent_record_offset":41940,"ancestor_record_offset":32668,"inlinee":7443,"source_line":1750,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36668,"parent_record_offset":35620,"ancestor_record_offset":32668,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42308,"parent_record_offset":42272,"ancestor_record_offset":32668,"inlinee":7444,"source_line":2685,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35660,"parent_record_offset":35620,"ancestor_record_offset":32668,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35724,"parent_record_offset":35692,"ancestor_record_offset":32668,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":41896,"parent_record_offset":41848,"ancestor_record_offset":32668,"inlinee":7439,"source_line":1950,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":43924,"parent_record_offset":43840,"ancestor_record_offset":32668,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":42492,"parent_record_offset":41896,"ancestor_record_offset":32668,"inlinee":7445,"source_line":2540,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":36732,"parent_record_offset":36700,"ancestor_record_offset":32668,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35588,"parent_record_offset":35560,"ancestor_record_offset":32668,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":35900,"parent_record_offset":35692,"ancestor_record_offset":32668,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":608,"record_offset":40240,"parent_record_offset":39856,"ancestor_record_offset":32668,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\rustc_entry.rs
//     evidence={"module":608,"record_offset":42964,"parent_record_offset":42940,"ancestor_record_offset":32668,"inlinee":38728,"source_line":307,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\rustc_entry.rs
//     evidence={"module":608,"record_offset":41816,"parent_record_offset":41784,"ancestor_record_offset":32668,"inlinee":7470,"source_line":512,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":43128,"parent_record_offset":43104,"ancestor_record_offset":32668,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":608,"record_offset":43104,"parent_record_offset":43080,"ancestor_record_offset":32668,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":43152,"parent_record_offset":43128,"ancestor_record_offset":32668,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":49560,"parent_record_offset":49208,"ancestor_record_offset":32668,"inlinee":12224,"source_line":362,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$78::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":49208,"parent_record_offset":49172,"ancestor_record_offset":32668,"inlinee":12269,"source_line":2919,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$79::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":49588,"parent_record_offset":49208,"ancestor_record_offset":32668,"inlinee":12226,"source_line":2936,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":32980,"parent_record_offset":32952,"ancestor_record_offset":32668,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":608,"record_offset":41340,"parent_record_offset":41312,"ancestor_record_offset":32668,"inlinee":19384,"source_line":2976,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":608,"record_offset":33052,"parent_record_offset":33008,"ancestor_record_offset":32668,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":608,"record_offset":33312,"parent_record_offset":33288,"ancestor_record_offset":32668,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":608,"record_offset":33172,"parent_record_offset":33144,"ancestor_record_offset":32668,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":608,"record_offset":33508,"parent_record_offset":33108,"ancestor_record_offset":32668,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":608,"record_offset":33008,"parent_record_offset":32980,"ancestor_record_offset":32668,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":608,"record_offset":33200,"parent_record_offset":33172,"ancestor_record_offset":32668,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":608,"record_offset":33232,"parent_record_offset":33200,"ancestor_record_offset":32668,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":608,"record_offset":33108,"parent_record_offset":33080,"ancestor_record_offset":32668,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":608,"record_offset":33080,"parent_record_offset":33052,"ancestor_record_offset":32668,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::bake_mod_textures::mod_new_texture_opts::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 292
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1591D
// 首条位置: {"module":1822,"inlinee":88349}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1822,"record_offset":11560,"parent_record_offset":10512,"ancestor_record_offset":10512,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1822,"record_offset":10544,"parent_record_offset":10512,"ancestor_record_offset":10512,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":11736,"parent_record_offset":11712,"ancestor_record_offset":10512,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":11760,"parent_record_offset":11736,"ancestor_record_offset":10512,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":11784,"parent_record_offset":11760,"ancestor_record_offset":10512,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1822,"record_offset":11712,"parent_record_offset":11656,"ancestor_record_offset":10512,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":11684,"parent_record_offset":11656,"ancestor_record_offset":10512,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":11656,"parent_record_offset":11632,"ancestor_record_offset":10512,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1822,"record_offset":11632,"parent_record_offset":11608,"ancestor_record_offset":10512,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1822,"record_offset":11248,"parent_record_offset":11220,"ancestor_record_offset":10512,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":1822,"record_offset":11276,"parent_record_offset":11248,"ancestor_record_offset":10512,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1822,"record_offset":11220,"parent_record_offset":11192,"ancestor_record_offset":10512,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1822,"record_offset":10780,"parent_record_offset":10756,"ancestor_record_offset":10512,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1822,"record_offset":10864,"parent_record_offset":10840,"ancestor_record_offset":10512,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1822,"record_offset":11388,"parent_record_offset":10676,"ancestor_record_offset":10512,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1822,"record_offset":11012,"parent_record_offset":10984,"ancestor_record_offset":10512,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1822,"record_offset":10984,"parent_record_offset":10956,"ancestor_record_offset":10512,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1822,"record_offset":10804,"parent_record_offset":10780,"ancestor_record_offset":10512,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1822,"record_offset":11584,"parent_record_offset":11560,"ancestor_record_offset":10512,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1822,"record_offset":11128,"parent_record_offset":11104,"ancestor_record_offset":10512,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1822,"record_offset":11304,"parent_record_offset":11276,"ancestor_record_offset":10512,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1822,"record_offset":11332,"parent_record_offset":11304,"ancestor_record_offset":10512,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1822,"record_offset":10924,"parent_record_offset":10896,"ancestor_record_offset":10512,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1822,"record_offset":10956,"parent_record_offset":10924,"ancestor_record_offset":10512,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1822,"record_offset":10896,"parent_record_offset":10676,"ancestor_record_offset":10512,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1822,"record_offset":10756,"parent_record_offset":10676,"ancestor_record_offset":10512,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1822,"record_offset":11416,"parent_record_offset":10676,"ancestor_record_offset":10512,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1822,"record_offset":10840,"parent_record_offset":10676,"ancestor_record_offset":10512,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1822,"record_offset":10728,"parent_record_offset":10676,"ancestor_record_offset":10512,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1822,"record_offset":11192,"parent_record_offset":11164,"ancestor_record_offset":10512,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1822,"record_offset":10568,"parent_record_offset":10544,"ancestor_record_offset":10512,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1822,"record_offset":11164,"parent_record_offset":11056,"ancestor_record_offset":10512,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1822,"record_offset":10600,"parent_record_offset":10568,"ancestor_record_offset":10512,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":11104,"parent_record_offset":11080,"ancestor_record_offset":10512,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":11508,"parent_record_offset":10676,"ancestor_record_offset":10512,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":11080,"parent_record_offset":11056,"ancestor_record_offset":10512,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":10652,"parent_record_offset":10628,"ancestor_record_offset":10512,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":10628,"parent_record_offset":10568,"ancestor_record_offset":10512,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":10676,"parent_record_offset":10652,"ancestor_record_offset":10512,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1822,"record_offset":11056,"parent_record_offset":10676,"ancestor_record_offset":10512,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01822.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::bake_mod_textures::texture_mappings
// 记录形式: compiled_procedure; instances=1
// 源码行: 298
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":608,"record_offset":26736,"rva":13099360,"va":5381808480}
// 编译实例 RVAs: 0xc7e160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2800595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381808480,"raw_hex":"e8d3bb2a00","size":5,"soft_fail":false,"target_va":5384609408,"va":5381808808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-6130208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381808480,"raw_hex":"e8e075a2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381808907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12469701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381808480,"raw_hex":"e83bba41ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5381808544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12465915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381808480,"raw_hex":"e805c941ff","size":5,"soft_fail":false,"target_va":5369343504,"va":5381809414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3071505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381808480,"raw_hex":"e8ef21d1ff","size":5,"soft_fail":false,"target_va":5378737680,"va":5381809180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a9960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2968105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381808480,"raw_hex":"e8d7b5d2ff","size":5,"soft_fail":false,"target_va":5378840928,"va":5381809028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409de280
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2753115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381808480,"raw_hex":"e8a5fdd5ff","size":5,"soft_fail":false,"target_va":5379056256,"va":5381809366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141616920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10061217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381808480,"raw_hex":"e8a1859900","size":5,"soft_fail":false,"target_va":5391870240,"va":5381809018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141651ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bake_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10304747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381808480,"raw_hex":"e8eb3c9d00","size":5,"soft_fail":false,"target_va":5392113360,"va":5381808608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2942736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381808480,"raw_hex":"e810e72c00","size":5,"soft_fail":false,"target_va":5384751264,"va":5381808523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":26964,"parent_record_offset":26736,"ancestor_record_offset":26736,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":608,"record_offset":27076,"parent_record_offset":26736,"ancestor_record_offset":26736,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":608,"record_offset":27972,"parent_record_offset":26736,"ancestor_record_offset":26736,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":26940,"parent_record_offset":26736,"ancestor_record_offset":26736,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":608,"record_offset":27132,"parent_record_offset":26736,"ancestor_record_offset":26736,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":26992,"parent_record_offset":26736,"ancestor_record_offset":26736,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] serde_json::map::Map::iter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/map.rs
//     evidence={"module":608,"record_offset":27916,"parent_record_offset":26736,"ancestor_record_offset":26736,"inlinee":38706,"source_line":303,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_object
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":608,"record_offset":27888,"parent_record_offset":26736,"ancestor_record_offset":26736,"inlinee":36646,"source_line":366,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":608,"record_offset":26912,"parent_record_offset":26736,"ancestor_record_offset":26736,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":27448,"parent_record_offset":27424,"ancestor_record_offset":26736,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":27472,"parent_record_offset":27448,"ancestor_record_offset":26736,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":27496,"parent_record_offset":27472,"ancestor_record_offset":26736,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":608,"record_offset":27424,"parent_record_offset":27396,"ancestor_record_offset":26736,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":608,"record_offset":27396,"parent_record_offset":27368,"ancestor_record_offset":26736,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":608,"record_offset":27940,"parent_record_offset":27916,"ancestor_record_offset":26736,"inlinee":8809,"source_line":2698,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":28496,"parent_record_offset":28472,"ancestor_record_offset":26736,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":28264,"parent_record_offset":28236,"ancestor_record_offset":26736,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":28236,"parent_record_offset":28212,"ancestor_record_offset":26736,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":28544,"parent_record_offset":28520,"ancestor_record_offset":26736,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":28520,"parent_record_offset":28496,"ancestor_record_offset":26736,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":608,"record_offset":28212,"parent_record_offset":28188,"ancestor_record_offset":26736,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":28472,"parent_record_offset":28448,"ancestor_record_offset":26736,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":27996,"parent_record_offset":27972,"ancestor_record_offset":26736,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":608,"record_offset":28448,"parent_record_offset":28424,"ancestor_record_offset":26736,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":608,"record_offset":28020,"parent_record_offset":27996,"ancestor_record_offset":26736,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":608,"record_offset":28688,"parent_record_offset":28660,"ancestor_record_offset":26736,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":608,"record_offset":27100,"parent_record_offset":27076,"ancestor_record_offset":26736,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":608,"record_offset":27280,"parent_record_offset":27244,"ancestor_record_offset":26736,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":27016,"parent_record_offset":26992,"ancestor_record_offset":26736,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":608,"record_offset":27040,"parent_record_offset":27016,"ancestor_record_offset":26736,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":27180,"parent_record_offset":27156,"ancestor_record_offset":26736,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":608,"record_offset":27156,"parent_record_offset":27132,"ancestor_record_offset":26736,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":608,"record_offset":27204,"parent_record_offset":27180,"ancestor_record_offset":26736,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00608.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::bake_mod_textures::texture_mappings::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 314
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B31
// 首条位置: {"module":528,"inlinee":31537}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":528,"record_offset":28540,"parent_record_offset":28484,"ancestor_record_offset":28484,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":528,"record_offset":29124,"parent_record_offset":28484,"ancestor_record_offset":28484,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":528,"record_offset":29152,"parent_record_offset":28484,"ancestor_record_offset":28484,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::as_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":528,"record_offset":28512,"parent_record_offset":28484,"ancestor_record_offset":28484,"inlinee":31538,"source_line":492,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":528,"record_offset":28944,"parent_record_offset":28920,"ancestor_record_offset":28484,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":528,"record_offset":28968,"parent_record_offset":28944,"ancestor_record_offset":28484,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":528,"record_offset":29312,"parent_record_offset":29288,"ancestor_record_offset":28484,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":528,"record_offset":29336,"parent_record_offset":29312,"ancestor_record_offset":28484,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":528,"record_offset":28992,"parent_record_offset":28968,"ancestor_record_offset":28484,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":528,"record_offset":29360,"parent_record_offset":29336,"ancestor_record_offset":28484,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":528,"record_offset":28920,"parent_record_offset":28772,"ancestor_record_offset":28484,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":528,"record_offset":29288,"parent_record_offset":29236,"ancestor_record_offset":28484,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":528,"record_offset":28712,"parent_record_offset":28684,"ancestor_record_offset":28484,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":528,"record_offset":29260,"parent_record_offset":29236,"ancestor_record_offset":28484,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":528,"record_offset":29236,"parent_record_offset":29212,"ancestor_record_offset":28484,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":528,"record_offset":28772,"parent_record_offset":28740,"ancestor_record_offset":28484,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":528,"record_offset":28740,"parent_record_offset":28712,"ancestor_record_offset":28484,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":528,"record_offset":29212,"parent_record_offset":29192,"ancestor_record_offset":28484,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":528,"record_offset":28808,"parent_record_offset":28772,"ancestor_record_offset":28484,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":528,"record_offset":28596,"parent_record_offset":28568,"ancestor_record_offset":28484,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":528,"record_offset":28624,"parent_record_offset":28596,"ancestor_record_offset":28484,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":528,"record_offset":28652,"parent_record_offset":28624,"ancestor_record_offset":28484,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":528,"record_offset":28568,"parent_record_offset":28540,"ancestor_record_offset":28484,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":528,"record_offset":28684,"parent_record_offset":28652,"ancestor_record_offset":28484,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":528,"record_offset":28832,"parent_record_offset":28808,"ancestor_record_offset":28484,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":528,"record_offset":28880,"parent_record_offset":28856,"ancestor_record_offset":28484,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":528,"record_offset":28856,"parent_record_offset":28832,"ancestor_record_offset":28484,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":528,"record_offset":29048,"parent_record_offset":28652,"ancestor_record_offset":28484,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":528,"record_offset":29072,"parent_record_offset":29048,"ancestor_record_offset":28484,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":528,"record_offset":29172,"parent_record_offset":29152,"ancestor_record_offset":28484,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00528.jsonl.gz"}
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
// 完整结构及原始类型记录: bake_mod_textures.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: bake_mod_textures.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: bake_mod_textures.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 24; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::clutter_registry::ClutterId>,ref$<country_core::resources::clutter_registry::ClutterId>)
// 候选参数类型: ["ref$<country_core::resources::clutter_registry::ClutterId>", "ref$<country_core::resources::clutter_registry::ClutterId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u64>,ref$<u64>)
// 候选参数类型: ["ref$<u64>", "ref$<u64>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<alloc::string::String> >,ref$<ref$<alloc::string::String> >)
// 候选参数类型: ["ref$<ref$<alloc::string::String> >", "ref$<ref$<alloc::string::String> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tuple$<alloc::string::String,std::path::PathBuf> >,ref$<tuple$<alloc::string::String,std::path::PathBuf> >)
// 候选参数类型: ["ref$<tuple$<alloc::string::String,std::path::PathBuf> >", "ref$<tuple$<alloc::string::String,std::path::PathBuf> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<mesh_to_sdf::generate::grid::State>,ref$<mesh_to_sdf::generate::grid::State>)
// 候选参数类型: ["ref$<mesh_to_sdf::generate::grid::State>", "ref$<mesh_to_sdf::generate::grid::State>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u32>,ref$<u32>)
// 候选参数类型: ["ref$<u32>", "ref$<u32>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<str$> >,ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >", "ref$<ref$<str$> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: core::ptr::drop_in_place<enum2$<country_core::startup::bake_mod_textures::bake_all::async_fn_env$0> >
// 信息 ID: 76112335d5371c47cec353a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["country_core::startup::bake_mod_textures::bake_all::async_fn_env$0"]; 字段布局 = unknown。
// [类型关系] core::ptr::drop_in_place -> ["enum2$<country_core::startup::bake_mod_textures::bake_all::async_fn_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources @ 0xc804fe -> 0x1a85910 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::bake_mod_textures::bake_all::async_fn$0
// 信息 ID: 303601fb46bcaa3b48cfca1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<enum2$<core::result::Result<alloc::vec::Vec<tuple$<alloc::string::String,enum2$<core::result::Result<alloc::sync::Arc<asset::texture::blob::TextureAssetBlob,alloc::alloc::Global>,anyhow::Error> > >,alloc::alloc::Global>,anyhow::Error> >,enum2$<country_core::startup::bake_mod_textures::bake_all::async_fn_env$0> >,enum2$<core::result::Result<alloc::vec::Vec<tuple$<alloc::string::String,enum2$<core::result::Result<alloc::sync::Arc<asset::texture::blob::TextureAssetBlob,alloc::alloc::Global>,anyhow::Error> > >,alloc::alloc::Global>,anyhow::Error> > >; module=1647, inline_record=23424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::bake_mod_textures::bake_all::async_fn$0::closure$0
// 信息 ID: 855d2de9b879789569656ce1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::bake_mod_textures::bake_all::async_fn$0", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::bake_mod_textures::bake_all::async_fn$0::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::startup::bake_mod_textures::bake_all::async_fn$0::closure_env$0,tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::startup::bake_mod_textures::bake_all::async_fn$0::closure_env$0,tuple$<core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> > > >; module=1651, inline_record=314600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources
// 信息 ID: 47951720d807edf8e9eb7f7b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mod_textures::reload_mod_textures @ 0x9e4872 -> 0xc7e560 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources::closure$0
// 信息 ID: 90389643214f89a68c112d6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources; module=608, inline_record=30900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources::closure$1
// 信息 ID: c632d6c002a180f0cf8ceabf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources; module=608, inline_record=64952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::bake_mod_textures::bake_recipe
// 信息 ID: 152c120078c8b0cebe6fe00b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources; module=608, inline_record=53264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::bake_mod_textures::bake_recipe::closure$0
// 信息 ID: 1cd8d302f17e63c1c62167f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::bake_mod_textures::bake_recipe", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::bake_mod_textures::bake_recipe::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::bake_mod_textures::bake_recipe::closure_env$0 <- alloc::vec::in_place_collect::impl$1::from_iter<asset_pipe::texture::TextureLayerAssetDesc,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::bake_mod_textures::bake_recipe::closure_env$0 <- alloc::vec::into_iter::impl$6::fold<std::path::PathBuf,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<std::path::PathBuf,asset_pipe::texture::TextureLayerAssetDesc,tuple$<>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::vec::impl$21::extend_trusted::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::bake_mod_textures::bake_recipe::closure_env$0 <- alloc::vec::into_iter::impl$6::fold<std::path::PathBuf,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<std::path::PathBuf,asset_pipe::texture::TextureLayerAssetDesc,tuple$<>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::vec::impl$21::extend_trusted::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::fold<std::path::PathBuf,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<std::path::PathBuf,asset_pipe::texture::TextureLayerAssetDesc,tuple$<>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::vec::impl$21::extend_trusted::closure_env$0<asset_pipe::texture::TextureLayerAssetDesc,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,country_core::startup::bake_mod_textures::bake_recipe::closure_env$0> > > > >; module=1822, inline_record=24688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::bake_mod_textures::mod_new_texture_opts
// 信息 ID: 81e21f9917e77190c07f5074
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources; module=608, inline_record=32668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::bake_mod_textures::mod_new_texture_opts::closure$0
// 信息 ID: 45ab9046c8f058de7a73e966
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::bake_mod_textures::mod_new_texture_opts", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::bake_mod_textures::mod_new_texture_opts::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,country_core::startup::bake_mod_textures::mod_new_texture_opts::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::bake_mod_textures::mod_new_texture_opts::closure_env$0 <- alloc::vec::into_iter::impl$6::fold<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<alloc::string::String,alloc::string::String>,tuple$<alloc::string::String,tuple$<asset::texture::desc::TextureOpts,asset::texture::blob::TextureAssetType> >,tuple$<>,country_core::startup::bake_mod_textures::mod_new_texture_opts::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<asset::texture::desc::TextureOpts,asset::texture::blob::TextureAssetType> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,tuple$<asset::texture::desc::TextureOpts,asset::texture::blob::TextureAssetType>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,country_core::startup::bake_mod_textures::mod_new_texture_opts::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::bake_mod_textures::mod_new_texture_opts::closure_env$0 <- alloc::vec::into_iter::impl$6::fold<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<alloc::string::String,alloc::string::String>,tuple$<alloc::string::String,tuple$<asset::texture::desc::TextureOpts,asset::texture::blob::TextureAssetType> >,tuple$<>,country_core::startup::bake_mod_textures::mod_new_texture_opts::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<asset::texture::desc::TextureOpts,asset::texture::blob::TextureAssetType> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,tuple$<asset::texture::desc::TextureOpts,asset::texture::blob::TextureAssetType>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,country_core::startup::bake_mod_textures::mod_new_texture_opts::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] alloc::vec::into_iter::impl$6::fold<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global,tuple$<>,core::iter::adapters::filter_map::filter_map_fold::closure_env$0<tuple$<alloc::string::String,alloc::string::String>,tuple$<alloc::string::String,tuple$<asset::texture::desc::TextureOpts,asset::texture::blob::TextureAssetType> >,tuple$<>,country_core::startup::bake_mod_textures::mod_new_texture_opts::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,tuple$<asset::texture::desc::TextureOpts,asset::texture::blob::TextureAssetType> >,hashbrown::map::impl$82::extend::closure_env$0<alloc::string::String,tuple$<asset::texture::desc::TextureOpts,asset::texture::blob::TextureAssetType>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<alloc::vec::into_iter::IntoIter<tuple$<alloc::string::String,alloc::string::String>,alloc::alloc::Global>,country_core::startup::bake_mod_textures::mod_new_texture_opts::closure_env$0> > > > >; module=1822, inline_record=10512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::bake_mod_textures::plan_bakes
// 信息 ID: 8985c4acb8f4e2133a113a4f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources; module=608, inline_record=31108; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::bake_mod_textures::plan_bakes::closure$0
// 信息 ID: 75d940877d47f096150d530b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::bake_mod_textures::plan_bakes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources; module=608, inline_record=32616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::bake_mod_textures::texture_mappings
// 信息 ID: e017fbddb2ae23f3c8b7a617
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources @ 0xc7f1e1 -> 0xc7e160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources @ 0xc7f809 -> 0xc7e160 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::bake_mod_textures::texture_mappings::closure$0
// 信息 ID: f2407a0e935aa2d922814b1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::bake_mod_textures::texture_mappings", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::bake_mod_textures::texture_mappings::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<alloc::string::String>,ref$<enum2$<serde_json::value::Value> > > >,country_core::startup::bake_mod_textures::texture_mappings::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::bake_mod_textures::texture_mappings::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<alloc::string::String,alloc::string::String>,core::iter::adapters::filter_map::FilterMap<serde_json::map::Iter,country_core::startup::bake_mod_textures::texture_mappings::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<alloc::string::String>,ref$<enum2$<serde_json::value::Value> > > >,country_core::startup::bake_mod_textures::texture_mappings::closure_env$0>; module=528, inline_record=28484; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::bake_mod_textures::upload_baked_texture
// 信息 ID: 7776637477ba972af117e58e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources; module=608, inline_record=64980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
