// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/game_loop.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: aab190536d11488d54a51b698268e5cbb8d188f6a9a26e7362b8e13502a164b9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: game_loop.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 194, 245
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use arboard; // namespace candidate
//   package identities: [{"name":"arboard","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.6.0"}]
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_input; // namespace candidate
//   package identities: [{"name":"bevy_input","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_platform; // namespace candidate
//   package identities: [{"name":"bevy_platform","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use egui; // namespace candidate
//   package identities: [{"name":"egui","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.2"}]
// use equivalent; // namespace candidate
//   package identities: [{"name":"equivalent","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.1"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.21.3"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use smol_str; // namespace candidate
//   package identities: [{"name":"smol_str","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.2.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use winit; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"2.9.0"},{"name":"winit","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e","version":"0.29.15"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":194,"record_offset":6172,"rva":5079296,"va":5373788416}
// 编译实例 RVAs: 0x4d8100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38058640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788416,"raw_hex":"e890ba4402","size":5,"soft_fail":false,"target_va":5411847168,"va":5373788523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::run
// 记录形式: compiled_procedure; instances=1
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":194,"record_offset":6284,"rva":5079632,"va":5373788752}
// 编译实例 RVAs: 0x4d8250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8853697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e8c1188700","size":5,"soft_fail":false,"target_va":5382644528,"va":5373790826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$winit..platform_impl..platform..monitor..VideoMode$u20$as$u20$core..cmp..PartialEq$GT$::eq
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1832063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e87ff41b00","size":5,"soft_fail":false,"target_va":5375621488,"va":5373789420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4808766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e8c29fb6ff","size":5,"soft_fail":false,"target_va":5368983552,"va":5373792313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=2
//     target_aliases=19 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4173924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e89c4fc0ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5373790031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e0530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2065133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e8137de0ff","size":5,"soft_fail":false,"target_va":5371725104,"va":5373790232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9a30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t921757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e89d100e00","size":5,"soft_fail":false,"target_va":5374712368,"va":5373790606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1022100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e894980f00","size":5,"soft_fail":false,"target_va":5374814320,"va":5373792215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e6cd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1106462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e81ee21000","size":5,"soft_fail":false,"target_va":5374897360,"va":5373790893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1112331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e80bf91000","size":5,"soft_fail":false,"target_va":5374904480,"va":5373792144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e8b60
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1114237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e87d001100","size":5,"soft_fail":false,"target_va":5374905184,"va":5373790942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f12b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1148807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e887871100","size":5,"soft_fail":false,"target_va":5374939824,"va":5373791012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f37f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1158261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e875ac1100","size":5,"soft_fail":false,"target_va":5374949360,"va":5373791094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f7630
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1174298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e81aeb1100","size":5,"soft_fail":false,"target_va":5374965296,"va":5373790993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140602c80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1221058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e8c2a11200","size":5,"soft_fail":false,"target_va":5375011968,"va":5373790905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140606870
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1236333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e86ddd1200","size":5,"soft_fail":false,"target_va":5375027312,"va":5373790974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14060a2c0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1251277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e8cd171300","size":5,"soft_fail":false,"target_va":5375042240,"va":5373790958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1292457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e8a9b81300","size":5,"soft_fail":false,"target_va":5375081488,"va":5373789026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1292279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e8f7b71300","size":5,"soft_fail":false,"target_va":5375081504,"va":5373789220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1725335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e897531a00","size":5,"soft_fail":false,"target_va":5375516032,"va":5373790692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140987ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4911893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e815f34a00","size":5,"soft_fail":false,"target_va":5378702496,"va":5373790598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5be90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6829064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e808346800","size":5,"soft_fail":false,"target_va":5380619920,"va":5373790851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7379316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e874997000","size":5,"soft_fail":false,"target_va":5381170352,"va":5373791031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9037432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e878e68900","size":5,"soft_fail":false,"target_va":5382827728,"va":5373790291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38058108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e87cb84402","size":5,"soft_fail":false,"target_va":5411847168,"va":5373789055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37749439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e8bf024002","size":5,"soft_fail":false,"target_va":5411540992,"va":5373791548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37749528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e818034002","size":5,"soft_fail":false,"target_va":5411540963,"va":5373791430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t1832015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e84ff41b00","size":5,"soft_fail":false,"target_va":5375621200,"va":5373789180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8866189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e88d498700","size":5,"soft_fail":false,"target_va":5382655024,"va":5373788830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8547739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e89b6d8200","size":5,"soft_fail":false,"target_va":5382338320,"va":5373790576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37750385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e871064002","size":5,"soft_fail":false,"target_va":5411541776,"va":5373791386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37751628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e84c0b4002","size":5,"soft_fail":false,"target_va":5411542880,"va":5373791247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8915910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e8c60b8800","size":5,"soft_fail":false,"target_va":5382706224,"va":5373790309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t37747431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e8e7fa3f02","size":5,"soft_fail":false,"target_va":5411538608,"va":5373791172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37747608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e898fb3f02","size":5,"soft_fail":false,"target_va":5411538768,"va":5373791155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37747677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e8ddfb3f02","size":5,"soft_fail":false,"target_va":5411538960,"va":5373791278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::game_loop::GameLoop::main_loop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e8f7120000","size":5,"soft_fail":false,"target_va":5373793664,"va":5373788804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t418691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e883630600","size":5,"soft_fail":false,"target_va":5374208688,"va":5373789992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::current_monitor
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1832511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373788752,"raw_hex":"e83ff61b00","size":5,"soft_fail":false,"target_va":5375621888,"va":5373789372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5373788752,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5373790407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":6408,"parent_record_offset":6284,"ancestor_record_offset":6284,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::shut_down_everything
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":6480,"parent_record_offset":6284,"ancestor_record_offset":6284,"inlinee":14478,"source_line":1020,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":6284,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":7884,"parent_record_offset":7860,"ancestor_record_offset":6284,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":9008,"parent_record_offset":8984,"ancestor_record_offset":6284,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":7836,"parent_record_offset":7812,"ancestor_record_offset":6284,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":8984,"parent_record_offset":8960,"ancestor_record_offset":6284,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":7744,"parent_record_offset":7688,"ancestor_record_offset":6284,"inlinee":6152,"source_line":1742,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":7780,"parent_record_offset":7744,"ancestor_record_offset":6284,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":7812,"parent_record_offset":7780,"ancestor_record_offset":6284,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":8960,"parent_record_offset":8936,"ancestor_record_offset":6284,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":6284,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":11964,"parent_record_offset":11936,"ancestor_record_offset":6284,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":12372,"parent_record_offset":12344,"ancestor_record_offset":6284,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":6284,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":11492,"parent_record_offset":11468,"ancestor_record_offset":6284,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":6284,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":12024,"parent_record_offset":11992,"ancestor_record_offset":6284,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":11992,"parent_record_offset":11964,"ancestor_record_offset":6284,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":6284,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_inner_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":7060,"parent_record_offset":6936,"ancestor_record_offset":6284,"inlinee":6797,"source_line":281,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":6968,"parent_record_offset":6936,"ancestor_record_offset":6284,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":6936,"parent_record_offset":6756,"ancestor_record_offset":6284,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":7092,"parent_record_offset":6756,"ancestor_record_offset":6284,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":6284,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":6284,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":14580,"parent_record_offset":14540,"ancestor_record_offset":6284,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":6284,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":11396,"parent_record_offset":11356,"ancestor_record_offset":6284,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":11908,"parent_record_offset":11848,"ancestor_record_offset":6284,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":11936,"parent_record_offset":11908,"ancestor_record_offset":6284,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":11356,"parent_record_offset":11332,"ancestor_record_offset":6284,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":14540,"parent_record_offset":14516,"ancestor_record_offset":6284,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":11744,"parent_record_offset":11720,"ancestor_record_offset":6284,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":11720,"parent_record_offset":11696,"ancestor_record_offset":6284,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":11668,"parent_record_offset":11636,"ancestor_record_offset":6284,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":11612,"parent_record_offset":11076,"ancestor_record_offset":6284,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":11636,"parent_record_offset":11612,"ancestor_record_offset":6284,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":14796,"parent_record_offset":14228,"ancestor_record_offset":6284,"inlinee":9406,"source_line":258,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":11104,"parent_record_offset":11076,"ancestor_record_offset":6284,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":14288,"parent_record_offset":14228,"ancestor_record_offset":6284,"inlinee":9260,"source_line":353,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":11696,"parent_record_offset":11636,"ancestor_record_offset":6284,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":6284,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":11128,"parent_record_offset":11104,"ancestor_record_offset":6284,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":14312,"parent_record_offset":14288,"ancestor_record_offset":6284,"inlinee":9261,"source_line":405,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":11332,"parent_record_offset":11308,"ancestor_record_offset":6284,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":14516,"parent_record_offset":14492,"ancestor_record_offset":6284,"inlinee":9266,"source_line":407,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":10984,"parent_record_offset":6480,"ancestor_record_offset":6284,"inlinee":8096,"source_line":2015,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::remove_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":14228,"parent_record_offset":6480,"ancestor_record_offset":6284,"inlinee":9405,"source_line":1793,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":11036,"parent_record_offset":11008,"ancestor_record_offset":6284,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":11008,"parent_record_offset":10984,"ancestor_record_offset":6284,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":11076,"parent_record_offset":11008,"ancestor_record_offset":6284,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::OwningPtr::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":194,"record_offset":14888,"parent_record_offset":14228,"ancestor_record_offset":6284,"inlinee":9407,"source_line":455,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":194,"record_offset":13768,"parent_record_offset":13736,"ancestor_record_offset":6284,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impl$0::clone_to_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":194,"record_offset":7928,"parent_record_offset":7744,"ancestor_record_offset":6284,"inlinee":6154,"source_line":275,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::uninit::impl$1::clone_one
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone\uninit.rs
//     evidence={"module":194,"record_offset":7952,"parent_record_offset":7928,"ancestor_record_offset":6284,"inlinee":6836,"source_line":56,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":15060,"parent_record_offset":6480,"ancestor_record_offset":6284,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":8164,"parent_record_offset":8128,"ancestor_record_offset":6284,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":194,"record_offset":12716,"parent_record_offset":12680,"ancestor_record_offset":6284,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":194,"record_offset":7976,"parent_record_offset":7952,"ancestor_record_offset":6284,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":194,"record_offset":9932,"parent_record_offset":6756,"ancestor_record_offset":6284,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":194,"record_offset":13792,"parent_record_offset":13768,"ancestor_record_offset":6284,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":194,"record_offset":11244,"parent_record_offset":11180,"ancestor_record_offset":6284,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":11268,"parent_record_offset":11244,"ancestor_record_offset":6284,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":11308,"parent_record_offset":11128,"ancestor_record_offset":6284,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":15672,"parent_record_offset":6480,"ancestor_record_offset":6284,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":8088,"parent_record_offset":6756,"ancestor_record_offset":6284,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":14260,"parent_record_offset":14228,"ancestor_record_offset":6284,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":7620,"parent_record_offset":6756,"ancestor_record_offset":6284,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":194,"record_offset":15148,"parent_record_offset":15116,"ancestor_record_offset":6284,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":194,"record_offset":15176,"parent_record_offset":15116,"ancestor_record_offset":6284,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=61
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":6428,"parent_record_offset":6408,"ancestor_record_offset":6284,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":14912,"parent_record_offset":14888,"ancestor_record_offset":6284,"inlinee":4834,"source_line":1354,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":13536,"parent_record_offset":13512,"ancestor_record_offset":6284,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":13584,"parent_record_offset":13560,"ancestor_record_offset":6284,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":13560,"parent_record_offset":13536,"ancestor_record_offset":6284,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":194,"record_offset":7000,"parent_record_offset":6968,"ancestor_record_offset":6284,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":14936,"parent_record_offset":14912,"ancestor_record_offset":6284,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":194,"record_offset":7580,"parent_record_offset":6756,"ancestor_record_offset":6284,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":194,"record_offset":11180,"parent_record_offset":11152,"ancestor_record_offset":6284,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":14744,"parent_record_offset":14720,"ancestor_record_offset":6284,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":11560,"parent_record_offset":11536,"ancestor_record_offset":6284,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":11216,"parent_record_offset":11180,"ancestor_record_offset":6284,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":11536,"parent_record_offset":11356,"ancestor_record_offset":6284,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":14720,"parent_record_offset":14540,"ancestor_record_offset":6284,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":7500,"parent_record_offset":7472,"ancestor_record_offset":6284,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":8516,"parent_record_offset":8328,"ancestor_record_offset":6284,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":7188,"parent_record_offset":7156,"ancestor_record_offset":6284,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":8608,"parent_record_offset":8576,"ancestor_record_offset":6284,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":16404,"parent_record_offset":16368,"ancestor_record_offset":6284,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":7404,"parent_record_offset":7372,"ancestor_record_offset":6284,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":7028,"parent_record_offset":6968,"ancestor_record_offset":6284,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":7212,"parent_record_offset":7188,"ancestor_record_offset":6284,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":7428,"parent_record_offset":7404,"ancestor_record_offset":6284,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":8540,"parent_record_offset":8516,"ancestor_record_offset":6284,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":8796,"parent_record_offset":8736,"ancestor_record_offset":6284,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":8632,"parent_record_offset":8608,"ancestor_record_offset":6284,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":8828,"parent_record_offset":8700,"ancestor_record_offset":6284,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":15204,"parent_record_offset":15116,"ancestor_record_offset":6284,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":194,"record_offset":15092,"parent_record_offset":6480,"ancestor_record_offset":6284,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":194,"record_offset":15116,"parent_record_offset":15092,"ancestor_record_offset":6284,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":15228,"parent_record_offset":15204,"ancestor_record_offset":6284,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":14976,"parent_record_offset":6480,"ancestor_record_offset":6284,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":194,"record_offset":12240,"parent_record_offset":12156,"ancestor_record_offset":6284,"inlinee":8878,"source_line":499,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":194,"record_offset":12184,"parent_record_offset":12156,"ancestor_record_offset":6284,"inlinee":8160,"source_line":25,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":194,"record_offset":12208,"parent_record_offset":12184,"ancestor_record_offset":6284,"inlinee":8161,"source_line":49,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":194,"record_offset":11800,"parent_record_offset":6480,"ancestor_record_offset":6284,"inlinee":10610,"source_line":2217,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":194,"record_offset":11848,"parent_record_offset":11800,"ancestor_record_offset":6284,"inlinee":8157,"source_line":2185,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":194,"record_offset":12156,"parent_record_offset":11848,"ancestor_record_offset":6284,"inlinee":8158,"source_line":2149,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":194,"record_offset":12828,"parent_record_offset":12800,"ancestor_record_offset":6284,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":194,"record_offset":12884,"parent_record_offset":12856,"ancestor_record_offset":6284,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":194,"record_offset":12856,"parent_record_offset":12828,"ancestor_record_offset":6284,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":194,"record_offset":12680,"parent_record_offset":6480,"ancestor_record_offset":6284,"inlinee":9318,"source_line":347,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":194,"record_offset":13512,"parent_record_offset":13484,"ancestor_record_offset":6284,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":194,"record_offset":13484,"parent_record_offset":13456,"ancestor_record_offset":6284,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":194,"record_offset":7372,"parent_record_offset":7344,"ancestor_record_offset":6284,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":194,"record_offset":7344,"parent_record_offset":7316,"ancestor_record_offset":6284,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":194,"record_offset":12800,"parent_record_offset":12772,"ancestor_record_offset":6284,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":194,"record_offset":12772,"parent_record_offset":12744,"ancestor_record_offset":6284,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":194,"record_offset":12744,"parent_record_offset":12716,"ancestor_record_offset":6284,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":8328,"parent_record_offset":8288,"ancestor_record_offset":6284,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":7472,"parent_record_offset":7276,"ancestor_record_offset":6284,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":7276,"parent_record_offset":7248,"ancestor_record_offset":6284,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":7120,"parent_record_offset":6756,"ancestor_record_offset":6284,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":7248,"parent_record_offset":7120,"ancestor_record_offset":6284,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":8288,"parent_record_offset":8256,"ancestor_record_offset":6284,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":194,"record_offset":7156,"parent_record_offset":7120,"ancestor_record_offset":6284,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":194,"record_offset":8576,"parent_record_offset":8288,"ancestor_record_offset":6284,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":194,"record_offset":7316,"parent_record_offset":7276,"ancestor_record_offset":6284,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$2::clone
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/mod.rs
//     evidence={"module":194,"record_offset":7656,"parent_record_offset":7620,"ancestor_record_offset":6284,"inlinee":6875,"source_line":29,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$5::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/mod.rs
//     evidence={"module":194,"record_offset":8128,"parent_record_offset":8088,"ancestor_record_offset":6284,"inlinee":14484,"source_line":29,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$10::clone
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":194,"record_offset":7716,"parent_record_offset":7688,"ancestor_record_offset":6284,"inlinee":6834,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$13::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":194,"record_offset":9060,"parent_record_offset":6756,"ancestor_record_offset":6284,"inlinee":14486,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$8::clone
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":194,"record_offset":7688,"parent_record_offset":7656,"ancestor_record_offset":6284,"inlinee":6832,"source_line":28,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":194,"record_offset":6652,"parent_record_offset":6600,"ancestor_record_offset":6284,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_fullscreen
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":194,"record_offset":6756,"parent_record_offset":6704,"ancestor_record_offset":6284,"inlinee":14482,"source_line":686,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::set_fullscreen
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":6600,"parent_record_offset":6480,"ancestor_record_offset":6284,"inlinee":14479,"source_line":1085,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_fullscreen::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":6704,"parent_record_offset":6652,"ancestor_record_offset":6284,"inlinee":14481,"source_line":1087,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::receive_app_events
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x389C
// 首条位置: {"module":194,"inlinee":14492}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":35492,"parent_record_offset":35448,"ancestor_record_offset":35448,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":194,"record_offset":35604,"parent_record_offset":35448,"ancestor_record_offset":35448,"inlinee":7838,"source_line":785,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":35544,"parent_record_offset":35492,"ancestor_record_offset":35448,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":35572,"parent_record_offset":35492,"ancestor_record_offset":35448,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":194,"record_offset":35628,"parent_record_offset":35604,"ancestor_record_offset":35448,"inlinee":7804,"source_line":915,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::send_window_events
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x389D
// 首条位置: {"module":194,"inlinee":14493}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":38276,"parent_record_offset":35676,"ancestor_record_offset":35676,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":194,"record_offset":38244,"parent_record_offset":35676,"ancestor_record_offset":35676,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":35796,"parent_record_offset":35676,"ancestor_record_offset":35676,"inlinee":8096,"source_line":2015,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":37820,"parent_record_offset":35676,"ancestor_record_offset":35676,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":194,"record_offset":37792,"parent_record_offset":35676,"ancestor_record_offset":35676,"inlinee":7835,"source_line":607,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::CursorRequestClient::next_request_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":37764,"parent_record_offset":35676,"ancestor_record_offset":35676,"inlinee":14496,"source_line":1078,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::on_cursor_moved_absolute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":36452,"parent_record_offset":35676,"ancestor_record_offset":35676,"inlinee":14494,"source_line":1014,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":38164,"parent_record_offset":38140,"ancestor_record_offset":35676,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":38140,"parent_record_offset":38084,"ancestor_record_offset":35676,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":36236,"parent_record_offset":36212,"ancestor_record_offset":35676,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":38112,"parent_record_offset":38084,"ancestor_record_offset":35676,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":38084,"parent_record_offset":38060,"ancestor_record_offset":35676,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":36284,"parent_record_offset":36260,"ancestor_record_offset":35676,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":36260,"parent_record_offset":36236,"ancestor_record_offset":35676,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":38060,"parent_record_offset":38036,"ancestor_record_offset":35676,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":37568,"parent_record_offset":37524,"ancestor_record_offset":35676,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":36212,"parent_record_offset":36188,"ancestor_record_offset":35676,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":36188,"parent_record_offset":36156,"ancestor_record_offset":35676,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":37524,"parent_record_offset":37488,"ancestor_record_offset":35676,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":39072,"parent_record_offset":39048,"ancestor_record_offset":35676,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":36156,"parent_record_offset":36132,"ancestor_record_offset":35676,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":37432,"parent_record_offset":37408,"ancestor_record_offset":35676,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":37408,"parent_record_offset":36452,"ancestor_record_offset":35676,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":194,"record_offset":37464,"parent_record_offset":36452,"ancestor_record_offset":35676,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":194,"record_offset":37488,"parent_record_offset":37464,"ancestor_record_offset":35676,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":37240,"parent_record_offset":37216,"ancestor_record_offset":35676,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":37216,"parent_record_offset":37192,"ancestor_record_offset":35676,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":36404,"parent_record_offset":36372,"ancestor_record_offset":35676,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":36348,"parent_record_offset":35876,"ancestor_record_offset":35676,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":37108,"parent_record_offset":36568,"ancestor_record_offset":35676,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":36372,"parent_record_offset":36348,"ancestor_record_offset":35676,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":35904,"parent_record_offset":35876,"ancestor_record_offset":35676,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":37192,"parent_record_offset":37132,"ancestor_record_offset":35676,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":35952,"parent_record_offset":35928,"ancestor_record_offset":35676,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":35928,"parent_record_offset":35904,"ancestor_record_offset":35676,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":36132,"parent_record_offset":36108,"ancestor_record_offset":35676,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":36488,"parent_record_offset":36452,"ancestor_record_offset":35676,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":37308,"parent_record_offset":37284,"ancestor_record_offset":35676,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":37284,"parent_record_offset":36568,"ancestor_record_offset":35676,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":35820,"parent_record_offset":35796,"ancestor_record_offset":35676,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":35876,"parent_record_offset":35820,"ancestor_record_offset":35676,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":36512,"parent_record_offset":36488,"ancestor_record_offset":35676,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":36568,"parent_record_offset":36512,"ancestor_record_offset":35676,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":40068,"parent_record_offset":40040,"ancestor_record_offset":35676,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":194,"record_offset":36044,"parent_record_offset":35980,"ancestor_record_offset":35676,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":36068,"parent_record_offset":36044,"ancestor_record_offset":35676,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":36108,"parent_record_offset":35928,"ancestor_record_offset":35676,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":37728,"parent_record_offset":36452,"ancestor_record_offset":35676,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":35848,"parent_record_offset":35820,"ancestor_record_offset":35676,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=71
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":37844,"parent_record_offset":37820,"ancestor_record_offset":35676,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":38324,"parent_record_offset":38300,"ancestor_record_offset":35676,"inlinee":14498,"source_line":1244,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":38300,"parent_record_offset":38276,"ancestor_record_offset":35676,"inlinee":14497,"source_line":1264,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":38348,"parent_record_offset":38324,"ancestor_record_offset":35676,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":37688,"parent_record_offset":37524,"ancestor_record_offset":35676,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":194,"record_offset":40600,"parent_record_offset":40544,"ancestor_record_offset":35676,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":194,"record_offset":35980,"parent_record_offset":35952,"ancestor_record_offset":35676,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":37056,"parent_record_offset":37032,"ancestor_record_offset":35676,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":36016,"parent_record_offset":35980,"ancestor_record_offset":35676,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":37032,"parent_record_offset":36852,"ancestor_record_offset":35676,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":37332,"parent_record_offset":37308,"ancestor_record_offset":35676,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":37356,"parent_record_offset":37332,"ancestor_record_offset":35676,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":194,"record_offset":40544,"parent_record_offset":40504,"ancestor_record_offset":35676,"inlinee":13835,"source_line":392,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::impl$4::send::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":194,"record_offset":40640,"parent_record_offset":40600,"ancestor_record_offset":35676,"inlinee":13832,"source_line":398,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::main_loop
// 记录形式: compiled_procedure; instances=1
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":194,"record_offset":17372,"rva":5084544,"va":5373793664}
// 编译实例 RVAs: 0x4d9580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9032412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8dcd28900","size":5,"soft_fail":false,"target_va":5382847888,"va":5373815471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$egui..data..input..RawInput$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7661602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e822e87400","size":5,"soft_fail":false,"target_va":5381474160,"va":5373812553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t8849813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e895098700","size":5,"soft_fail":false,"target_va":5382644528,"va":5373794710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..time..Instant$u20$as$u20$core..ops..arith..Add$LT$core..time..Duration$GT$$GT$::add
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t8940420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8846b8800","size":5,"soft_fail":false,"target_va":5382748704,"va":5373808279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$u32$u20$as$u20$winit..dpi..Pixel$GT$::from_f64
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1821167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8efc91b00","size":5,"soft_fail":false,"target_va":5375621104,"va":5373799932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4839452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8e427b6ff","size":5,"soft_fail":false,"target_va":5368983552,"va":5373822999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004c400
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4799184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e830c5b6ff","size":5,"soft_fail":false,"target_va":5369021440,"va":5373820619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004ceb0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4786626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e83ef6b6ff","size":5,"soft_fail":false,"target_va":5369024176,"va":5373810797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2580
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4445421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8132bbcff","size":5,"soft_fail":false,"target_va":5369374080,"va":5373819496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=56
//     target_aliases=19 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4178172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8043fc0ff","size":5,"soft_fail":false,"target_va":5369616112,"va":5373794279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e6f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4158825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8978ac0ff","size":5,"soft_fail":false,"target_va":5369655184,"va":5373814004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3665933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8f30fc8ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5373824056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140175450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3563841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8bf9ec9ff","size":5,"soft_fail":false,"target_va":5370238032,"va":5373801868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140268ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2592669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e86370d8ff","size":5,"soft_fail":false,"target_va":5371235488,"va":5373828152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b47f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2252403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e88da1ddff","size":5,"soft_fail":false,"target_va":5371545584,"va":5373797982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b4910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2253100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8d49eddff","size":5,"soft_fail":false,"target_va":5371545872,"va":5373798967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b4aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2251264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e800a6ddff","size":5,"soft_fail":false,"target_va":5371546272,"va":5373797531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b4c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2252076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8d4a2ddff","size":5,"soft_fail":false,"target_va":5371546688,"va":5373798759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403282f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1777079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e849e2e4ff","size":5,"soft_fail":false,"target_va":5372019440,"va":5373796514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140328840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1775515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e865e8e4ff","size":5,"soft_fail":false,"target_va":5372020800,"va":5373796310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140328d80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1773411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e89df0e4ff","size":5,"soft_fail":false,"target_va":5372022144,"va":5373795550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140329350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1772126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8a2f5e4ff","size":5,"soft_fail":false,"target_va":5372023632,"va":5373795753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140329890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1772162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e87ef5e4ff","size":5,"soft_fail":false,"target_va":5372024976,"va":5373797133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140329e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1769431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e82900e5ff","size":5,"soft_fail":false,"target_va":5372026480,"va":5373795906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14032a3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1769133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e85301e5ff","size":5,"soft_fail":false,"target_va":5372027840,"va":5373796968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14032a9f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1766675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8ed0ae5ff","size":5,"soft_fail":false,"target_va":5372029424,"va":5373796094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=2
//     target_aliases=102 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t989491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e833190f00","size":5,"soft_fail":false,"target_va":5374814320,"va":5373824824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e8000
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1072943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e82f5f1000","size":5,"soft_fail":false,"target_va":5374902272,"va":5373829324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e8b60
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1110429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e89df11000","size":5,"soft_fail":false,"target_va":5374905184,"va":5373794750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e9e10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1081578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8ea801000","size":5,"soft_fail":false,"target_va":5374909968,"va":5373828385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f0d10
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1140467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8f3661100","size":5,"soft_fail":false,"target_va":5374938384,"va":5373797912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f1730
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1117695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8ff0d1100","size":5,"soft_fail":false,"target_va":5374940976,"va":5373823276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fa110
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1170109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8bdda1100","size":5,"soft_fail":false,"target_va":5374976272,"va":5373806158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fdb80
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1188819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8d3231200","size":5,"soft_fail":false,"target_va":5374991232,"va":5373802408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406035f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1198129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e831481200","size":5,"soft_fail":false,"target_va":5375014384,"va":5373816250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140609210
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1231284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8b4c91200","size":5,"soft_fail":false,"target_va":5375037968,"va":5373806679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1281200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8b08c1300","size":5,"soft_fail":false,"target_va":5375081488,"va":5373800283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=24
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1283540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8d4951300","size":5,"soft_fail":false,"target_va":5375081504,"va":5373797959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=6
//     target_aliases=69 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1721497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e899441a00","size":5,"soft_fail":false,"target_va":5375516032,"va":5373794530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093fc60
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4592925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e81d154600","size":5,"soft_fail":false,"target_va":5378407520,"va":5373814590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140987ca0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4895387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e89bb24a00","size":5,"soft_fail":false,"target_va":5378702496,"va":5373807104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14289aa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37491882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8aa143c02","size":5,"soft_fail":false,"target_va":5411285568,"va":5373793681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a1640
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37501176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8f8383c02","size":5,"soft_fail":false,"target_va":5411313216,"va":5373812035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a4550
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37513066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e86a673c02","size":5,"soft_fail":false,"target_va":5411325264,"va":5373812193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=17
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38052923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e83ba44402","size":5,"soft_fail":false,"target_va":5411847168,"va":5373794240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38049952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8a0984402","size":5,"soft_fail":false,"target_va":5411847184,"va":5373797227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37722774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8969a3f02","size":5,"soft_fail":false,"target_va":5411540992,"va":5373818213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t558885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e825870800","size":5,"soft_fail":false,"target_va":5374355568,"va":5373796678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t1516847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e82f251700","size":5,"soft_fail":false,"target_va":5375322928,"va":5373806076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arboard::Clipboard::get_text
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arboard","version":"3.6.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1387442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8b22b1500","size":5,"soft_fail":false,"target_va":5375191856,"va":5373804409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8189266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e852f57c00","size":5,"soft_fail":false,"target_va":5382002848,"va":5373813577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::storage::blob_vec::BlobVec::swap_remove_and_forget_unchecked
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8542834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8725a8200","size":5,"soft_fail":false,"target_va":5382338320,"va":5373795481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_input::keyboard::impl$32::clone
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_input","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8cdb5ffff","size":5,"soft_fail":false,"target_va":5373787248,"va":5373806238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t37725047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e877a33f02","size":5,"soft_fail":false,"target_va":5411541584,"va":5373816532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37725191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e807a43f02","size":5,"soft_fail":false,"target_va":5411541808,"va":5373816612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=46
//     evidence={"assembly":"\tcallq\t37725037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e86da33f02","size":5,"soft_fail":false,"target_va":5411541776,"va":5373816734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37725363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8b3a43f02","size":5,"soft_fail":false,"target_va":5411542064,"va":5373816696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37723972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8449f3f02","size":5,"soft_fail":false,"target_va":5411541904,"va":5373817927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$std..sync..mpmc..error..SendTimeoutError$LT$tiny_glade..event_loop..GameEvent$GT$$GT$::h5e6f64744f958e10 (.llvm.17929901220240424442)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1161383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e85947eeff","size":5,"soft_fail":false,"target_va":5372666112,"va":5373827490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<winit::keyboard::Key<smol_str::SmolStr> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-39988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8cc63ffff","size":5,"soft_fail":false,"target_va":5373787184,"va":5373827167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37724774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e866a23f02","size":5,"soft_fail":false,"target_va":5411542880,"va":5373818101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t37725031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e867a33f02","size":5,"soft_fail":false,"target_va":5411543360,"va":5373818324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::meta_commander::MetaCommander::is_profiling
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4458817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e841094400","size":5,"soft_fail":false,"target_va":5378273520,"va":5373814698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::render_config::UpscalingPreset::tgsr_resolution_scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4766104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e898b94800","size":5,"soft_fail":false,"target_va":5378575280,"va":5373809171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::render_config::impl$38::clone
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-14758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e85ac6ffff","size":5,"soft_fail":false,"target_va":5373787472,"va":5373802225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::EscMenuState::is_open
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4592781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e88d144600","size":5,"soft_fail":false,"target_va":5378407360,"va":5373814574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::logo_animation::LogoAnimation::finish_wait
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6554317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8cd026400","size":5,"soft_fail":false,"target_va":5380349232,"va":5373794910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::logo_animation::LogoAnimation::is_finished
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6554180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e844026400","size":5,"soft_fail":false,"target_va":5380349216,"va":5373795031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::logo_animation::LogoAnimation::is_waiting
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6554841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8d9046400","size":5,"soft_fail":false,"target_va":5380349200,"va":5373794354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::WindowSize::set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4648771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e843ef4600","size":5,"soft_fail":false,"target_va":5378448384,"va":5373799608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::VermeerState::into_disable_hold
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3989386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e88adf3c00","size":5,"soft_fail":false,"target_va":5377786336,"va":5373796945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] egui::context::Context::begin_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"egui","version":"0.27.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7877094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8e6317800","size":5,"soft_fail":false,"target_va":5381690112,"va":5373813013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t37678592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e800ee3e02","size":5,"soft_fail":false,"target_va":5411494800,"va":5373816203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8011719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8c73f7a00","size":5,"soft_fail":false,"target_va":5381824016,"va":5373812292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::GlobalProfiler::lock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7575033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8f9957300","size":5,"soft_fail":false,"target_va":5381387296,"va":5373812258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::GlobalProfiler::new_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7575110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e846967300","size":5,"soft_fail":false,"target_va":5381387392,"va":5373812277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7573412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8a48f7300","size":5,"soft_fail":false,"target_va":5381385536,"va":5373812119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7578633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e809a47300","size":5,"soft_fail":false,"target_va":5381390640,"va":5373812002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7573352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8688f7300","size":5,"soft_fail":false,"target_va":5381386480,"va":5373813123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7572932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8c48d7300","size":5,"soft_fail":false,"target_va":5381384704,"va":5373811767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::refresh_memory_budget
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7456115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e873c57100","size":5,"soft_fail":false,"target_va":5381263232,"va":5373807112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7451335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8c7b27100","size":5,"soft_fail":false,"target_va":5381262944,"va":5373811604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::any_present_successful
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7538443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e80b077300","size":5,"soft_fail":false,"target_va":5381353328,"va":5373814880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::extent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7544791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8d71f7300","size":5,"soft_fail":false,"target_va":5381352080,"va":5373807284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::mark_stale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7550708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8f4367300","size":5,"soft_fail":false,"target_va":5381351760,"va":5373801047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::on_focus
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7548184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8182d7300","size":5,"soft_fail":false,"target_va":5381351088,"va":5373802899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::resize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7544074","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e80a1d7300","size":5,"soft_fail":false,"target_va":5381351408,"va":5373807329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::set_vsync
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7536206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e84efe7200","size":5,"soft_fail":false,"target_va":5381351584,"va":5373815373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_pool::ResourcePool::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7380980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8f49f7000","size":5,"soft_fail":false,"target_va":5381191408,"va":5373810423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8847423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e83f008700","size":5,"soft_fail":false,"target_va":5382657856,"va":5373810428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t37732259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8a3bf3f02","size":5,"soft_fail":false,"target_va":5411538608,"va":5373806344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::Sender$LT$T$GT$::send
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-1133509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e83bb4eeff","size":5,"soft_fail":false,"target_va":5372664336,"va":5373797840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t89796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8c45e0100","size":5,"soft_fail":false,"target_va":5373888016,"va":5373798215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t100375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e817880100","size":5,"soft_fail":false,"target_va":5373894800,"va":5373794420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2573330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8eebbd8ff","size":5,"soft_fail":false,"target_va":5371224784,"va":5373798109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2564618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8f6ddd8ff","size":5,"soft_fail":false,"target_va":5371229792,"va":5373794405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::send
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t251300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8a4d50300","size":5,"soft_fail":false,"target_va":5374049472,"va":5373798167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t290087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8276d0400","size":5,"soft_fail":false,"target_va":5374084528,"va":5373794436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8943329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8e1768800","size":5,"soft_fail":false,"target_va":5382753792,"va":5373810458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37732450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e862c03f02","size":5,"soft_fail":false,"target_va":5411538768,"va":5373806313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t37735176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e808cb3f02","size":5,"soft_fail":false,"target_va":5411538960,"va":5373803779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::read_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37733303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8b7c33f02","size":5,"soft_fail":false,"target_va":5411539504,"va":5373806196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37733786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e89ac53f02","size":5,"soft_fail":false,"target_va":5411540288,"va":5373806497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t37718869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8558b3f02","size":5,"soft_fail":false,"target_va":5411537104,"va":5373818230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8884239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e80f908700","size":5,"soft_fail":false,"target_va":5382691200,"va":5373806956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::elapsed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8936183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8f75a8800","size":5,"soft_fail":false,"target_va":5382748432,"va":5373812244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8939349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e855678800","size":5,"soft_fail":false,"target_va":5382747616,"va":5373808262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::egui_backend::EguiBackend::end_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4197063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e839f5bfff","size":5,"soft_fail":false,"target_va":5369616656,"va":5373813714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::event_loop::EguiEventIo::take_egui_input
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2559348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e88cf2d8ff","size":5,"soft_fail":false,"target_va":5371256784,"va":5373816127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::fps_limiter::FpsLimiter::frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2547599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e87120d9ff","size":5,"soft_fail":false,"target_va":5371267152,"va":5373814746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::game_loop::GameLoop::extract_available_monitors_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t36419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8438e0000","size":5,"soft_fail":false,"target_va":5373830256,"va":5373793832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::log_time_to_first_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1061957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8bbcbefff","size":5,"soft_fail":false,"target_va":5372752384,"va":5373814336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::render_loop::render
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t39503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e84f9a0000","size":5,"soft_fail":false,"target_va":5373853712,"va":5373814204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::vram::calculate_max_resolution_scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t111738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e87ab40100","size":5,"soft_fail":false,"target_va":5373919008,"va":5373807265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::winit_bevy_glue::winit_to_bevy_keycode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4062140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e84404c2ff","size":5,"soft_fail":false,"target_va":5369739344,"va":5373801479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::on_clipboard_paste
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1744287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e89f9d1a00","size":5,"soft_fail":false,"target_va":5375549648,"va":5373805356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::on_key_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1743702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8569b1a00","size":5,"soft_fail":false,"target_va":5375549376,"va":5373805669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::keyboard::Key::to_text
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1880614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e826b21c00","size":5,"soft_fail":false,"target_va":5375682032,"va":5373801413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::event_loop::EventLoopThreadExecutor::execute_in_thread
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t390901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e8f5f60500","size":5,"soft_fail":false,"target_va":5374205872,"va":5373814966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::current_monitor
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1820258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e862c61b00","size":5,"soft_fail":false,"target_va":5375621888,"va":5373801625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::window::Window::window_state_lock
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1845078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373793664,"raw_hex":"e856271c00","size":5,"soft_fail":false,"target_va":5375645312,"va":5373800229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":42080,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":17592,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":194,"record_offset":76604,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":26644,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::collections::Events::send_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":194,"record_offset":33640,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":9990,"source_line":164,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":24372,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":8096,"source_line":2015,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":32664,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::remove_resource
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":27968,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":9405,"source_line":1793,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":25620,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":9173,"source_line":2000,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":25384,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":194,"record_offset":42372,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":25176,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":9187,"source_line":672,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":27560,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":28728,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":25592,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":194,"record_offset":25416,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":25300,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":194,"record_offset":25232,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":7838,"source_line":785,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::apply_setting_changes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":17620,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":14488,"source_line":960,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::process_app_event
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":42400,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":14499,"source_line":329,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::receive_app_events
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":35448,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":14492,"source_line":131,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::send_window_events
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":35676,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":14493,"source_line":148,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::simulate_and_render_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":76896,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":14531,"source_line":534,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":113488,"parent_record_offset":17372,"ancestor_record_offset":17372,"inlinee":8437,"source_line":940,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":51568,"parent_record_offset":51544,"ancestor_record_offset":17372,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":51592,"parent_record_offset":51568,"ancestor_record_offset":17372,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":21592,"parent_record_offset":21564,"ancestor_record_offset":17372,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":51544,"parent_record_offset":51520,"ancestor_record_offset":17372,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":21564,"parent_record_offset":21504,"ancestor_record_offset":17372,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":51452,"parent_record_offset":51424,"ancestor_record_offset":17372,"inlinee":6152,"source_line":1742,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":51488,"parent_record_offset":51452,"ancestor_record_offset":17372,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":51520,"parent_record_offset":51488,"ancestor_record_offset":17372,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":52428,"parent_record_offset":52404,"ancestor_record_offset":17372,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=70
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":19444,"parent_record_offset":19416,"ancestor_record_offset":17372,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":21532,"parent_record_offset":21504,"ancestor_record_offset":17372,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":21504,"parent_record_offset":21476,"ancestor_record_offset":17372,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=70
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":19500,"parent_record_offset":19472,"ancestor_record_offset":17372,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=70
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":19472,"parent_record_offset":19444,"ancestor_record_offset":17372,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":21476,"parent_record_offset":21452,"ancestor_record_offset":17372,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":57256,"parent_record_offset":57228,"ancestor_record_offset":17372,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":57228,"parent_record_offset":57192,"ancestor_record_offset":17372,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":51108,"parent_record_offset":42400,"ancestor_record_offset":17372,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":73488,"parent_record_offset":73464,"ancestor_record_offset":17372,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":22948,"parent_record_offset":22904,"ancestor_record_offset":17372,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":28324,"parent_record_offset":28284,"ancestor_record_offset":17372,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":19416,"parent_record_offset":19388,"ancestor_record_offset":17372,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":19388,"parent_record_offset":19344,"ancestor_record_offset":17372,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":42228,"parent_record_offset":42080,"ancestor_record_offset":17372,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":22904,"parent_record_offset":22864,"ancestor_record_offset":17372,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":42256,"parent_record_offset":42080,"ancestor_record_offset":17372,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":194,"record_offset":76824,"parent_record_offset":76796,"ancestor_record_offset":17372,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":38276,"parent_record_offset":35676,"ancestor_record_offset":17372,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":39072,"parent_record_offset":39048,"ancestor_record_offset":17372,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":19344,"parent_record_offset":19316,"ancestor_record_offset":17372,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":28284,"parent_record_offset":28260,"ancestor_record_offset":17372,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":194,"record_offset":38244,"parent_record_offset":35676,"ancestor_record_offset":17372,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=31
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":22772,"parent_record_offset":22744,"ancestor_record_offset":17372,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":22744,"parent_record_offset":17620,"ancestor_record_offset":17372,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":194,"record_offset":46620,"parent_record_offset":46564,"ancestor_record_offset":17372,"inlinee":8098,"source_line":2472,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_resource_id::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":194,"record_offset":47660,"parent_record_offset":47620,"ancestor_record_offset":17372,"inlinee":8100,"source_line":2473,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_resource_id::closure$0::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":194,"record_offset":48996,"parent_record_offset":48972,"ancestor_record_offset":17372,"inlinee":8103,"source_line":2479,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$22::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":194,"record_offset":71084,"parent_record_offset":70440,"ancestor_record_offset":17372,"inlinee":14530,"source_line":174,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":194,"record_offset":22836,"parent_record_offset":22808,"ancestor_record_offset":17372,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_default
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":194,"record_offset":22808,"parent_record_offset":17620,"ancestor_record_offset":17372,"inlinee":9990,"source_line":164,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":194,"record_offset":22864,"parent_record_offset":22836,"ancestor_record_offset":17372,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=46
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":19780,"parent_record_offset":19752,"ancestor_record_offset":17372,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=46
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":19752,"parent_record_offset":19724,"ancestor_record_offset":17372,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=56
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":19692,"parent_record_offset":19660,"ancestor_record_offset":17372,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":20708,"parent_record_offset":20104,"ancestor_record_offset":17372,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":19632,"parent_record_offset":19028,"ancestor_record_offset":17372,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=56
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":19660,"parent_record_offset":19632,"ancestor_record_offset":17372,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::remove
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":28540,"parent_record_offset":27968,"ancestor_record_offset":17372,"inlinee":9406,"source_line":258,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=49
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":19064,"parent_record_offset":19028,"ancestor_record_offset":17372,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":28032,"parent_record_offset":27968,"ancestor_record_offset":17372,"inlinee":9260,"source_line":353,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":20808,"parent_record_offset":20740,"ancestor_record_offset":17372,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":19724,"parent_record_offset":19660,"ancestor_record_offset":17372,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=57
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":19120,"parent_record_offset":19092,"ancestor_record_offset":17372,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=49
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":19092,"parent_record_offset":19064,"ancestor_record_offset":17372,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":28056,"parent_record_offset":28032,"ancestor_record_offset":17372,"inlinee":9261,"source_line":405,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=49
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":19316,"parent_record_offset":19288,"ancestor_record_offset":17372,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get_mut::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":28260,"parent_record_offset":28236,"ancestor_record_offset":17372,"inlinee":9266,"source_line":407,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":107008,"parent_record_offset":76896,"ancestor_record_offset":17372,"inlinee":9218,"source_line":1825,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":35796,"parent_record_offset":35676,"ancestor_record_offset":17372,"inlinee":8096,"source_line":2015,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":25656,"parent_record_offset":25620,"ancestor_record_offset":17372,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":19856,"parent_record_offset":19828,"ancestor_record_offset":17372,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":107072,"parent_record_offset":107044,"ancestor_record_offset":17372,"inlinee":9220,"source_line":1828,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":19828,"parent_record_offset":19028,"ancestor_record_offset":17372,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":20036,"parent_record_offset":20000,"ancestor_record_offset":17372,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":20104,"parent_record_offset":20036,"ancestor_record_offset":17372,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":18964,"parent_record_offset":18932,"ancestor_record_offset":17372,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":19028,"parent_record_offset":18964,"ancestor_record_offset":17372,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::impl$11::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.16.0/src/lib.rs
//     evidence={"module":194,"record_offset":71056,"parent_record_offset":70440,"ancestor_record_offset":17372,"inlinee":14528,"source_line":163,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$2::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.16.0/src/keyboard.rs
//     evidence={"module":194,"record_offset":70440,"parent_record_offset":42400,"ancestor_record_offset":17372,"inlinee":14524,"source_line":97,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$46::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.16.0/src/keyboard.rs
//     evidence={"module":194,"record_offset":70512,"parent_record_offset":70440,"ancestor_record_offset":17372,"inlinee":14526,"source_line":763,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":194,"record_offset":46676,"parent_record_offset":46620,"ancestor_record_offset":17372,"inlinee":5614,"source_line":933,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::OwningPtr::read
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":194,"record_offset":28628,"parent_record_offset":27968,"ancestor_record_offset":17372,"inlinee":9407,"source_line":455,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":194,"record_offset":47500,"parent_record_offset":47476,"ancestor_record_offset":17372,"inlinee":5068,"source_line":719,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::any::type_name
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":194,"record_offset":26576,"parent_record_offset":25620,"ancestor_record_offset":17372,"inlinee":4206,"source_line":812,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":194,"record_offset":80540,"parent_record_offset":76896,"ancestor_record_offset":17372,"inlinee":14532,"source_line":13,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":194,"record_offset":80596,"parent_record_offset":80568,"ancestor_record_offset":17372,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":194,"record_offset":80568,"parent_record_offset":80540,"ancestor_record_offset":17372,"inlinee":9488,"source_line":152,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":96456,"parent_record_offset":96424,"ancestor_record_offset":17372,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":88684,"parent_record_offset":88652,"ancestor_record_offset":17372,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":88752,"parent_record_offset":88716,"ancestor_record_offset":17372,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":96484,"parent_record_offset":96456,"ancestor_record_offset":17372,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":96392,"parent_record_offset":96356,"ancestor_record_offset":17372,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":96424,"parent_record_offset":96392,"ancestor_record_offset":17372,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":96616,"parent_record_offset":96424,"ancestor_record_offset":17372,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":96736,"parent_record_offset":96712,"ancestor_record_offset":17372,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impl$0::clone_to_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":194,"record_offset":51636,"parent_record_offset":51452,"ancestor_record_offset":17372,"inlinee":6154,"source_line":275,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::uninit::impl$1::clone_one
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone\uninit.rs
//     evidence={"module":194,"record_offset":51660,"parent_record_offset":51636,"ancestor_record_offset":17372,"inlinee":6836,"source_line":56,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":84928,"parent_record_offset":80636,"ancestor_record_offset":17372,"inlinee":11953,"source_line":1063,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":50100,"parent_record_offset":50072,"ancestor_record_offset":17372,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":88012,"parent_record_offset":76896,"ancestor_record_offset":17372,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":65772,"parent_record_offset":65748,"ancestor_record_offset":17372,"inlinee":11109,"source_line":1425,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":83468,"parent_record_offset":80636,"ancestor_record_offset":17372,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":84784,"parent_record_offset":84760,"ancestor_record_offset":17372,"inlinee":5087,"source_line":1371,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":47548,"parent_record_offset":47524,"ancestor_record_offset":17372,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":65820,"parent_record_offset":65796,"ancestor_record_offset":17372,"inlinee":14516,"source_line":1820,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":83836,"parent_record_offset":83812,"ancestor_record_offset":17372,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":56920,"parent_record_offset":56892,"ancestor_record_offset":17372,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":194,"record_offset":46188,"parent_record_offset":42400,"ancestor_record_offset":17372,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$108::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":194,"record_offset":46300,"parent_record_offset":46276,"ancestor_record_offset":17372,"inlinee":5955,"source_line":77,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":194,"record_offset":46948,"parent_record_offset":46924,"ancestor_record_offset":17372,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":194,"record_offset":47032,"parent_record_offset":47008,"ancestor_record_offset":17372,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":194,"record_offset":111552,"parent_record_offset":76896,"ancestor_record_offset":17372,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":194,"record_offset":26608,"parent_record_offset":25620,"ancestor_record_offset":17372,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":194,"record_offset":46972,"parent_record_offset":46948,"ancestor_record_offset":17372,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":194,"record_offset":84000,"parent_record_offset":83944,"ancestor_record_offset":17372,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":194,"record_offset":18348,"parent_record_offset":18320,"ancestor_record_offset":17372,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":194,"record_offset":18320,"parent_record_offset":17620,"ancestor_record_offset":17372,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":90224,"parent_record_offset":90196,"ancestor_record_offset":17372,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":40068,"parent_record_offset":40040,"ancestor_record_offset":17372,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":194,"record_offset":95888,"parent_record_offset":95852,"ancestor_record_offset":17372,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":83944,"parent_record_offset":83908,"ancestor_record_offset":17372,"inlinee":9728,"source_line":870,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":83972,"parent_record_offset":83944,"ancestor_record_offset":17372,"inlinee":9729,"source_line":2579,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":88716,"parent_record_offset":88652,"ancestor_record_offset":17372,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":194,"record_offset":47184,"parent_record_offset":47156,"ancestor_record_offset":17372,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":194,"record_offset":47156,"parent_record_offset":47128,"ancestor_record_offset":17372,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=72
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":194,"record_offset":19220,"parent_record_offset":19152,"ancestor_record_offset":17372,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":68520,"parent_record_offset":42400,"ancestor_record_offset":17372,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=58
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":19244,"parent_record_offset":19220,"ancestor_record_offset":17372,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":84036,"parent_record_offset":83880,"ancestor_record_offset":17372,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":107600,"parent_record_offset":107008,"ancestor_record_offset":17372,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=61
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":19288,"parent_record_offset":19092,"ancestor_record_offset":17372,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":47620,"parent_record_offset":46620,"ancestor_record_offset":17372,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=37
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":19964,"parent_record_offset":18932,"ancestor_record_offset":17372,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":88384,"parent_record_offset":88356,"ancestor_record_offset":17372,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":56892,"parent_record_offset":42400,"ancestor_record_offset":17372,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=56
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":18996,"parent_record_offset":18964,"ancestor_record_offset":17372,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":21084,"parent_record_offset":20988,"ancestor_record_offset":17372,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":194,"record_offset":25472,"parent_record_offset":25440,"ancestor_record_offset":17372,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":194,"record_offset":25500,"parent_record_offset":25440,"ancestor_record_offset":17372,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":194,"record_offset":95980,"parent_record_offset":95952,"ancestor_record_offset":17372,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":194,"record_offset":96320,"parent_record_offset":96172,"ancestor_record_offset":17372,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=253
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":18448,"parent_record_offset":17620,"ancestor_record_offset":17372,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":35544,"parent_record_offset":35492,"ancestor_record_offset":17372,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":28656,"parent_record_offset":28628,"ancestor_record_offset":17372,"inlinee":4834,"source_line":1354,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":88476,"parent_record_offset":88356,"ancestor_record_offset":17372,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":47416,"parent_record_offset":47392,"ancestor_record_offset":17372,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":38324,"parent_record_offset":38300,"ancestor_record_offset":17372,"inlinee":14498,"source_line":1244,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":38300,"parent_record_offset":38276,"ancestor_record_offset":17372,"inlinee":14497,"source_line":1264,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":38348,"parent_record_offset":38324,"ancestor_record_offset":17372,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":194,"record_offset":76756,"parent_record_offset":76732,"ancestor_record_offset":17372,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":194,"record_offset":57288,"parent_record_offset":57256,"ancestor_record_offset":17372,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":194,"record_offset":95412,"parent_record_offset":95384,"ancestor_record_offset":17372,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":194,"record_offset":95384,"parent_record_offset":95356,"ancestor_record_offset":17372,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":194,"record_offset":76676,"parent_record_offset":76632,"ancestor_record_offset":17372,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":18376,"parent_record_offset":18348,"ancestor_record_offset":17372,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":88504,"parent_record_offset":88476,"ancestor_record_offset":17372,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":18408,"parent_record_offset":18348,"ancestor_record_offset":17372,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":194,"record_offset":40600,"parent_record_offset":40544,"ancestor_record_offset":17372,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":194,"record_offset":68572,"parent_record_offset":68548,"ancestor_record_offset":17372,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":194,"record_offset":18272,"parent_record_offset":17620,"ancestor_record_offset":17372,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":194,"record_offset":56992,"parent_record_offset":56968,"ancestor_record_offset":17372,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":194,"record_offset":57016,"parent_record_offset":56992,"ancestor_record_offset":17372,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=57
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":194,"record_offset":19152,"parent_record_offset":19120,"ancestor_record_offset":17372,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":194,"record_offset":42284,"parent_record_offset":42080,"ancestor_record_offset":17372,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":28488,"parent_record_offset":28464,"ancestor_record_offset":17372,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":19576,"parent_record_offset":19548,"ancestor_record_offset":17372,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=57
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":19188,"parent_record_offset":19152,"ancestor_record_offset":17372,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":19548,"parent_record_offset":19344,"ancestor_record_offset":17372,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":28464,"parent_record_offset":28284,"ancestor_record_offset":17372,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":194,"record_offset":42308,"parent_record_offset":42284,"ancestor_record_offset":17372,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":194,"record_offset":95328,"parent_record_offset":95300,"ancestor_record_offset":17372,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":194,"record_offset":76732,"parent_record_offset":76708,"ancestor_record_offset":17372,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":194,"record_offset":95356,"parent_record_offset":95328,"ancestor_record_offset":17372,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":194,"record_offset":95700,"parent_record_offset":95668,"ancestor_record_offset":17372,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":194,"record_offset":76632,"parent_record_offset":76604,"ancestor_record_offset":17372,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":194,"record_offset":95300,"parent_record_offset":95176,"ancestor_record_offset":17372,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":194,"record_offset":76708,"parent_record_offset":76632,"ancestor_record_offset":17372,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":194,"record_offset":95852,"parent_record_offset":95820,"ancestor_record_offset":17372,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":194,"record_offset":94980,"parent_record_offset":94944,"ancestor_record_offset":17372,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":194,"record_offset":95140,"parent_record_offset":95104,"ancestor_record_offset":17372,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":194,"record_offset":95104,"parent_record_offset":95064,"ancestor_record_offset":17372,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":194,"record_offset":95036,"parent_record_offset":95008,"ancestor_record_offset":17372,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":194,"record_offset":95064,"parent_record_offset":95036,"ancestor_record_offset":17372,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":194,"record_offset":95008,"parent_record_offset":94980,"ancestor_record_offset":17372,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":194,"record_offset":95668,"parent_record_offset":95064,"ancestor_record_offset":17372,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":194,"record_offset":95820,"parent_record_offset":95788,"ancestor_record_offset":17372,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":194,"record_offset":95924,"parent_record_offset":95820,"ancestor_record_offset":17372,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":194,"record_offset":95788,"parent_record_offset":95756,"ancestor_record_offset":17372,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":194,"record_offset":56944,"parent_record_offset":56920,"ancestor_record_offset":17372,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":194,"record_offset":95756,"parent_record_offset":94944,"ancestor_record_offset":17372,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":194,"record_offset":95952,"parent_record_offset":95924,"ancestor_record_offset":17372,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":194,"record_offset":95176,"parent_record_offset":95140,"ancestor_record_offset":17372,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":194,"record_offset":95492,"parent_record_offset":95176,"ancestor_record_offset":17372,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":194,"record_offset":95460,"parent_record_offset":95176,"ancestor_record_offset":17372,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":194,"record_offset":95236,"parent_record_offset":95176,"ancestor_record_offset":17372,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":18184,"parent_record_offset":18152,"ancestor_record_offset":17372,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":18724,"parent_record_offset":18520,"ancestor_record_offset":17372,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":97344,"parent_record_offset":97316,"ancestor_record_offset":17372,"inlinee":6050,"source_line":1566,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":47868,"parent_record_offset":47748,"ancestor_record_offset":17372,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":49120,"parent_record_offset":49084,"ancestor_record_offset":17372,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":19884,"parent_record_offset":19856,"ancestor_record_offset":17372,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":17840,"parent_record_offset":17808,"ancestor_record_offset":17372,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":18824,"parent_record_offset":18792,"ancestor_record_offset":17372,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":18076,"parent_record_offset":18044,"ancestor_record_offset":17372,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":57316,"parent_record_offset":57256,"ancestor_record_offset":17372,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":17868,"parent_record_offset":17840,"ancestor_record_offset":17372,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":47892,"parent_record_offset":47868,"ancestor_record_offset":17372,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":18104,"parent_record_offset":18076,"ancestor_record_offset":17372,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":18752,"parent_record_offset":18724,"ancestor_record_offset":17372,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":49144,"parent_record_offset":49120,"ancestor_record_offset":17372,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":18852,"parent_record_offset":18824,"ancestor_record_offset":17372,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":73616,"parent_record_offset":73488,"ancestor_record_offset":17372,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":194,"record_offset":83908,"parent_record_offset":83880,"ancestor_record_offset":17372,"inlinee":14538,"source_line":749,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":194,"record_offset":83812,"parent_record_offset":83788,"ancestor_record_offset":17372,"inlinee":5095,"source_line":79,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$5::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":194,"record_offset":83880,"parent_record_offset":80636,"ancestor_record_offset":17372,"inlinee":14537,"source_line":1143,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":194,"record_offset":47524,"parent_record_offset":47500,"ancestor_record_offset":17372,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::adaptive_render_state::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/adaptive_render_state.rs
//     evidence={"module":194,"record_offset":81632,"parent_record_offset":80636,"ancestor_record_offset":17372,"inlinee":14535,"source_line":5,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$13::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":194,"record_offset":21024,"parent_record_offset":20988,"ancestor_record_offset":17372,"inlinee":10688,"source_line":16,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":194,"record_offset":21120,"parent_record_offset":21084,"ancestor_record_offset":17372,"inlinee":10692,"source_line":33,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$22::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":194,"record_offset":20988,"parent_record_offset":17620,"ancestor_record_offset":17372,"inlinee":10686,"source_line":48,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":194,"record_offset":21052,"parent_record_offset":21024,"ancestor_record_offset":17372,"inlinee":10690,"source_line":7,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":194,"record_offset":20000,"parent_record_offset":17620,"ancestor_record_offset":17372,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":194,"record_offset":18932,"parent_record_offset":17620,"ancestor_record_offset":17372,"inlinee":11668,"source_line":41,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":194,"record_offset":47476,"parent_record_offset":47452,"ancestor_record_offset":17372,"inlinee":4660,"source_line":88,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":194,"record_offset":47092,"parent_record_offset":47064,"ancestor_record_offset":17372,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":194,"record_offset":47128,"parent_record_offset":47092,"ancestor_record_offset":17372,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":194,"record_offset":47064,"parent_record_offset":46844,"ancestor_record_offset":17372,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":194,"record_offset":46924,"parent_record_offset":46844,"ancestor_record_offset":17372,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":194,"record_offset":47228,"parent_record_offset":46844,"ancestor_record_offset":17372,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":194,"record_offset":47008,"parent_record_offset":46844,"ancestor_record_offset":17372,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":194,"record_offset":46720,"parent_record_offset":46676,"ancestor_record_offset":17372,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":194,"record_offset":46756,"parent_record_offset":46720,"ancestor_record_offset":17372,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":194,"record_offset":47452,"parent_record_offset":47344,"ancestor_record_offset":17372,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":47392,"parent_record_offset":47368,"ancestor_record_offset":17372,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":47312,"parent_record_offset":46844,"ancestor_record_offset":17372,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":47368,"parent_record_offset":47344,"ancestor_record_offset":17372,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":46820,"parent_record_offset":46796,"ancestor_record_offset":17372,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":46796,"parent_record_offset":46756,"ancestor_record_offset":17372,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":194,"record_offset":90164,"parent_record_offset":90128,"ancestor_record_offset":17372,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":194,"record_offset":90064,"parent_record_offset":90036,"ancestor_record_offset":17372,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":46844,"parent_record_offset":46820,"ancestor_record_offset":17372,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":194,"record_offset":90128,"parent_record_offset":90064,"ancestor_record_offset":17372,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":194,"record_offset":90096,"parent_record_offset":90064,"ancestor_record_offset":17372,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":46896,"parent_record_offset":46844,"ancestor_record_offset":17372,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":194,"record_offset":90196,"parent_record_offset":90164,"ancestor_record_offset":17372,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":194,"record_offset":89740,"parent_record_offset":89716,"ancestor_record_offset":17372,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":47344,"parent_record_offset":46844,"ancestor_record_offset":17372,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":194,"record_offset":97664,"parent_record_offset":76896,"ancestor_record_offset":17372,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":194,"record_offset":97756,"parent_record_offset":97728,"ancestor_record_offset":17372,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":25528,"parent_record_offset":25440,"ancestor_record_offset":17372,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":194,"record_offset":83500,"parent_record_offset":80636,"ancestor_record_offset":17372,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":194,"record_offset":25440,"parent_record_offset":25416,"ancestor_record_offset":17372,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":25552,"parent_record_offset":25528,"ancestor_record_offset":17372,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":83384,"parent_record_offset":80636,"ancestor_record_offset":17372,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":194,"record_offset":97316,"parent_record_offset":97288,"ancestor_record_offset":17372,"inlinee":6048,"source_line":49,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":194,"record_offset":97200,"parent_record_offset":97168,"ancestor_record_offset":17372,"inlinee":6042,"source_line":1308,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":194,"record_offset":97288,"parent_record_offset":97256,"ancestor_record_offset":17372,"inlinee":6046,"source_line":963,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":194,"record_offset":97228,"parent_record_offset":97200,"ancestor_record_offset":17372,"inlinee":6044,"source_line":1115,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":194,"record_offset":97256,"parent_record_offset":97228,"ancestor_record_offset":17372,"inlinee":6045,"source_line":1152,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":194,"record_offset":97168,"parent_record_offset":97140,"ancestor_record_offset":17372,"inlinee":6040,"source_line":1375,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":194,"record_offset":97788,"parent_record_offset":97756,"ancestor_record_offset":17372,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":194,"record_offset":97824,"parent_record_offset":97788,"ancestor_record_offset":17372,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":96036,"parent_record_offset":76896,"ancestor_record_offset":17372,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":96080,"parent_record_offset":96036,"ancestor_record_offset":17372,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":102012,"parent_record_offset":101972,"ancestor_record_offset":17372,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":102632,"parent_record_offset":102332,"ancestor_record_offset":17372,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":194,"record_offset":96240,"parent_record_offset":96212,"ancestor_record_offset":17372,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":96356,"parent_record_offset":96172,"ancestor_record_offset":17372,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":96656,"parent_record_offset":96356,"ancestor_record_offset":17372,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] puffin::now_ns
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":97140,"parent_record_offset":76896,"ancestor_record_offset":17372,"inlinee":9050,"source_line":538,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":94944,"parent_record_offset":76896,"ancestor_record_offset":17372,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::temporal::impl$18::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":194,"record_offset":88076,"parent_record_offset":76896,"ancestor_record_offset":17372,"inlinee":14540,"source_line":79,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$33::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":194,"record_offset":57164,"parent_record_offset":57128,"ancestor_record_offset":17372,"inlinee":9881,"source_line":37,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$37::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":194,"record_offset":57192,"parent_record_offset":57164,"ancestor_record_offset":17372,"inlinee":9883,"source_line":410,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":194,"record_offset":88872,"parent_record_offset":88116,"ancestor_record_offset":17372,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":194,"record_offset":88116,"parent_record_offset":88076,"ancestor_record_offset":17372,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":194,"record_offset":88172,"parent_record_offset":88144,"ancestor_record_offset":17372,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":194,"record_offset":88436,"parent_record_offset":88412,"ancestor_record_offset":17372,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":194,"record_offset":88292,"parent_record_offset":88264,"ancestor_record_offset":17372,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":194,"record_offset":88652,"parent_record_offset":88228,"ancestor_record_offset":17372,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":194,"record_offset":88144,"parent_record_offset":88116,"ancestor_record_offset":17372,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":194,"record_offset":18044,"parent_record_offset":18012,"ancestor_record_offset":17372,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":194,"record_offset":18012,"parent_record_offset":17980,"ancestor_record_offset":17372,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::try_recv
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":194,"record_offset":25256,"parent_record_offset":25232,"ancestor_record_offset":17372,"inlinee":7804,"source_line":915,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":194,"record_offset":40544,"parent_record_offset":40504,"ancestor_record_offset":17372,"inlinee":13835,"source_line":392,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::impl$4::send::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":194,"record_offset":40640,"parent_record_offset":40600,"ancestor_record_offset":17372,"inlinee":13832,"source_line":398,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":194,"record_offset":35604,"parent_record_offset":35448,"ancestor_record_offset":17372,"inlinee":7838,"source_line":785,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":194,"record_offset":21204,"parent_record_offset":17620,"ancestor_record_offset":17372,"inlinee":7835,"source_line":607,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":47952,"parent_record_offset":47928,"ancestor_record_offset":17372,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":18520,"parent_record_offset":18480,"ancestor_record_offset":17372,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":18152,"parent_record_offset":17940,"ancestor_record_offset":17372,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":17940,"parent_record_offset":17908,"ancestor_record_offset":17372,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":17768,"parent_record_offset":17620,"ancestor_record_offset":17372,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":17908,"parent_record_offset":17768,"ancestor_record_offset":17372,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":18480,"parent_record_offset":18448,"ancestor_record_offset":17372,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":194,"record_offset":47712,"parent_record_offset":47660,"ancestor_record_offset":17372,"inlinee":8101,"source_line":367,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":194,"record_offset":47928,"parent_record_offset":47712,"ancestor_record_offset":17372,"inlinee":7022,"source_line":682,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$31::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":194,"record_offset":49056,"parent_record_offset":49028,"ancestor_record_offset":17372,"inlinee":7028,"source_line":816,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$20::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":194,"record_offset":83788,"parent_record_offset":83764,"ancestor_record_offset":17372,"inlinee":5093,"source_line":14,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":194,"record_offset":17808,"parent_record_offset":17768,"ancestor_record_offset":17372,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":194,"record_offset":18792,"parent_record_offset":18480,"ancestor_record_offset":17372,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":194,"record_offset":47748,"parent_record_offset":47712,"ancestor_record_offset":17372,"inlinee":7017,"source_line":94,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":194,"record_offset":49084,"parent_record_offset":49056,"ancestor_record_offset":17372,"inlinee":7029,"source_line":110,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":194,"record_offset":47840,"parent_record_offset":47748,"ancestor_record_offset":17372,"inlinee":7019,"source_line":47,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":194,"record_offset":88324,"parent_record_offset":88292,"ancestor_record_offset":17372,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":194,"record_offset":88356,"parent_record_offset":88324,"ancestor_record_offset":17372,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":194,"record_offset":88228,"parent_record_offset":88200,"ancestor_record_offset":17372,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":194,"record_offset":88200,"parent_record_offset":88172,"ancestor_record_offset":17372,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":194,"record_offset":17980,"parent_record_offset":17940,"ancestor_record_offset":17372,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$23::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":194,"record_offset":83764,"parent_record_offset":83732,"ancestor_record_offset":17372,"inlinee":5091,"source_line":149,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::app_config::PendingVideoSettingChanges::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":194,"record_offset":20956,"parent_record_offset":17620,"ancestor_record_offset":17372,"inlinee":14545,"source_line":603,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::app_config::PendingVideoSettingChanges::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":194,"record_offset":18900,"parent_record_offset":17620,"ancestor_record_offset":17372,"inlinee":14544,"source_line":585,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::app_config::impl$24::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":194,"record_offset":67688,"parent_record_offset":42400,"ancestor_record_offset":17372,"inlinee":14518,"source_line":739,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::egui_backend::EguiBackend::take_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/egui_backend.rs
//     evidence={"module":194,"record_offset":112772,"parent_record_offset":76896,"ancestor_record_offset":17372,"inlinee":8298,"source_line":45,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::event_loop::impl$5::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/event_loop.rs
//     evidence={"module":194,"record_offset":65796,"parent_record_offset":65772,"ancestor_record_offset":17372,"inlinee":14515,"source_line":83,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::CursorRequestClient::is_fully_confirmed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":65748,"parent_record_offset":42400,"ancestor_record_offset":17372,"inlinee":14513,"source_line":1087,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::CursorRequestClient::next_request_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":37764,"parent_record_offset":35676,"ancestor_record_offset":17372,"inlinee":14496,"source_line":1078,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::CursorRequestClient::on_confirmation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":50072,"parent_record_offset":42400,"ancestor_record_offset":17372,"inlinee":14500,"source_line":1083,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::GameLoop::maintain_resolution_scale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":80636,"parent_record_offset":76896,"ancestor_record_offset":17372,"inlinee":14533,"source_line":781,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::main_loop::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":25200,"parent_record_offset":25176,"ancestor_record_offset":17372,"inlinee":14490,"source_line":207,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::main_loop::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":27528,"parent_record_offset":27504,"ancestor_record_offset":17372,"inlinee":14491,"source_line":227,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::process_app_event::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":68548,"parent_record_offset":68520,"ancestor_record_offset":17372,"inlinee":14522,"source_line":422,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":110560,"parent_record_offset":110532,"ancestor_record_offset":17372,"inlinee":14542,"source_line":759,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":111520,"parent_record_offset":111492,"ancestor_record_offset":17372,"inlinee":14543,"source_line":767,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::on_cursor_moved_absolute
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":36452,"parent_record_offset":35676,"ancestor_record_offset":17372,"inlinee":14494,"source_line":1014,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":194,"record_offset":46248,"parent_record_offset":46216,"ancestor_record_offset":17372,"inlinee":5953,"source_line":109,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":194,"record_offset":46216,"parent_record_offset":46188,"ancestor_record_offset":17372,"inlinee":5972,"source_line":466,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::keyboard::ModifiersState::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":194,"record_offset":56820,"parent_record_offset":42400,"ancestor_record_offset":17372,"inlinee":14510,"source_line":703,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::keyboard::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":194,"record_offset":56852,"parent_record_offset":56820,"ancestor_record_offset":17372,"inlinee":14512,"source_line":703,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::keyboard::impl$13::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/keyboard.rs
//     evidence={"module":194,"record_offset":68492,"parent_record_offset":42400,"ancestor_record_offset":17372,"inlinee":14520,"source_line":1504,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$1::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/mod.rs
//     evidence={"module":194,"record_offset":52256,"parent_record_offset":52232,"ancestor_record_offset":17372,"inlinee":14508,"source_line":46,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$2::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/mod.rs
//     evidence={"module":194,"record_offset":51396,"parent_record_offset":51364,"ancestor_record_offset":17372,"inlinee":6875,"source_line":29,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$8::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":194,"record_offset":51424,"parent_record_offset":51396,"ancestor_record_offset":17372,"inlinee":6832,"source_line":28,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":194,"record_offset":57436,"parent_record_offset":57412,"ancestor_record_offset":17372,"inlinee":11205,"source_line":1023,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::fullscreen
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":194,"record_offset":51300,"parent_record_offset":51248,"ancestor_record_offset":17372,"inlinee":14504,"source_line":680,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":194,"record_offset":113516,"parent_record_offset":113488,"ancestor_record_offset":17372,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":194,"record_offset":51192,"parent_record_offset":51136,"ancestor_record_offset":17372,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_visible
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":194,"record_offset":113568,"parent_record_offset":113544,"ancestor_record_offset":17372,"inlinee":6838,"source_line":147,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":57364,"parent_record_offset":42400,"ancestor_record_offset":17372,"inlinee":11202,"source_line":1488,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::Window::fullscreen
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":51136,"parent_record_offset":42400,"ancestor_record_offset":17372,"inlinee":14501,"source_line":1107,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::fullscreen::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":51248,"parent_record_offset":51192,"ancestor_record_offset":17372,"inlinee":14503,"source_line":1109,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::fullscreen::closure$0::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":52208,"parent_record_offset":52180,"ancestor_record_offset":17372,"inlinee":14506,"source_line":1109,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_visible::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":113544,"parent_record_offset":113516,"ancestor_record_offset":17372,"inlinee":8439,"source_line":942,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":57412,"parent_record_offset":57388,"ancestor_record_offset":17372,"inlinee":11204,"source_line":1490,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::main_loop::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 207
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x389A
// 首条位置: {"module":194,"inlinee":14490}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::main_loop::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x389B
// 首条位置: {"module":194,"inlinee":14491}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::process_app_event
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 329
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x38A3
// 首条位置: {"module":194,"inlinee":14499}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":51108,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":44224,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":194,"record_offset":43040,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::collections::Events::send_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":194,"record_offset":56548,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":9990,"source_line":164,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":54740,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":8096,"source_line":2015,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":43304,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_input::keyboard::impl$2::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.16.0/src/keyboard.rs
//     evidence={"module":194,"record_offset":70440,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":14524,"source_line":97,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":194,"record_offset":46188,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":68520,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":58396,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":73964,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":56892,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":57128,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":52308,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":194,"record_offset":67640,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":67176,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::impl$24::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":194,"record_offset":67688,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":14518,"source_line":739,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::CursorRequestClient::is_fully_confirmed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":65748,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":14513,"source_line":1087,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::CursorRequestClient::on_confirmation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":50072,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":14500,"source_line":1083,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::on_cursor_moved_absolute
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":65860,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":14494,"source_line":1014,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] winit::keyboard::ModifiersState::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":194,"record_offset":56820,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":14510,"source_line":703,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] winit::keyboard::impl$13::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/keyboard.rs
//     evidence={"module":194,"record_offset":68492,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":14520,"source_line":1504,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":57364,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":11202,"source_line":1488,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::fullscreen
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":51136,"parent_record_offset":42400,"ancestor_record_offset":42400,"inlinee":14501,"source_line":1107,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":51568,"parent_record_offset":51544,"ancestor_record_offset":42400,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":51592,"parent_record_offset":51568,"ancestor_record_offset":42400,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":52476,"parent_record_offset":52452,"ancestor_record_offset":42400,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":51544,"parent_record_offset":51520,"ancestor_record_offset":42400,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":52452,"parent_record_offset":52428,"ancestor_record_offset":42400,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":51452,"parent_record_offset":51424,"ancestor_record_offset":42400,"inlinee":6152,"source_line":1742,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":51488,"parent_record_offset":51452,"ancestor_record_offset":42400,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":51520,"parent_record_offset":51488,"ancestor_record_offset":42400,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":52428,"parent_record_offset":52404,"ancestor_record_offset":42400,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":43176,"parent_record_offset":43144,"ancestor_record_offset":42400,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":59656,"parent_record_offset":59628,"ancestor_record_offset":42400,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":59628,"parent_record_offset":59604,"ancestor_record_offset":42400,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":43224,"parent_record_offset":43200,"ancestor_record_offset":42400,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":43200,"parent_record_offset":43176,"ancestor_record_offset":42400,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":59604,"parent_record_offset":59580,"ancestor_record_offset":42400,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":57256,"parent_record_offset":57228,"ancestor_record_offset":42400,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":57228,"parent_record_offset":57192,"ancestor_record_offset":42400,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":73488,"parent_record_offset":73464,"ancestor_record_offset":42400,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":43144,"parent_record_offset":43104,"ancestor_record_offset":42400,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":43732,"parent_record_offset":43708,"ancestor_record_offset":42400,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":43708,"parent_record_offset":43668,"ancestor_record_offset":42400,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":43104,"parent_record_offset":43068,"ancestor_record_offset":42400,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":43668,"parent_record_offset":43644,"ancestor_record_offset":42400,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":44248,"parent_record_offset":44224,"ancestor_record_offset":42400,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":66820,"parent_record_offset":65860,"ancestor_record_offset":42400,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":194,"record_offset":46620,"parent_record_offset":46564,"ancestor_record_offset":42400,"inlinee":8098,"source_line":2472,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_resource_id::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":194,"record_offset":47660,"parent_record_offset":47620,"ancestor_record_offset":42400,"inlinee":8100,"source_line":2473,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::impl$19::get_resource_id::closure$0::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs
//     evidence={"module":194,"record_offset":48996,"parent_record_offset":48972,"ancestor_record_offset":42400,"inlinee":8103,"source_line":2479,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$22::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":194,"record_offset":71084,"parent_record_offset":70440,"ancestor_record_offset":42400,"inlinee":14530,"source_line":174,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":194,"record_offset":56572,"parent_record_offset":56548,"ancestor_record_offset":42400,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":194,"record_offset":43068,"parent_record_offset":43040,"ancestor_record_offset":42400,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":44056,"parent_record_offset":44032,"ancestor_record_offset":42400,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":44032,"parent_record_offset":44008,"ancestor_record_offset":42400,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":43980,"parent_record_offset":43948,"ancestor_record_offset":42400,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":55356,"parent_record_offset":54820,"ancestor_record_offset":42400,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":43924,"parent_record_offset":43384,"ancestor_record_offset":42400,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":43948,"parent_record_offset":43924,"ancestor_record_offset":42400,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":43416,"parent_record_offset":43384,"ancestor_record_offset":42400,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":55440,"parent_record_offset":55380,"ancestor_record_offset":42400,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":44008,"parent_record_offset":43948,"ancestor_record_offset":42400,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":43464,"parent_record_offset":43440,"ancestor_record_offset":42400,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":43440,"parent_record_offset":43416,"ancestor_record_offset":42400,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":43644,"parent_record_offset":43620,"ancestor_record_offset":42400,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":65900,"parent_record_offset":65860,"ancestor_record_offset":42400,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":44124,"parent_record_offset":44100,"ancestor_record_offset":42400,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":44100,"parent_record_offset":43384,"ancestor_record_offset":42400,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":54764,"parent_record_offset":54740,"ancestor_record_offset":42400,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":54820,"parent_record_offset":54764,"ancestor_record_offset":42400,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":43328,"parent_record_offset":43304,"ancestor_record_offset":42400,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":43384,"parent_record_offset":43328,"ancestor_record_offset":42400,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::impl$11::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.16.0/src/lib.rs
//     evidence={"module":194,"record_offset":71056,"parent_record_offset":70440,"ancestor_record_offset":42400,"inlinee":14528,"source_line":163,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_input::keyboard::impl$46::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_input","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_input-0.16.0/src/keyboard.rs
//     evidence={"module":194,"record_offset":70512,"parent_record_offset":70440,"ancestor_record_offset":42400,"inlinee":14526,"source_line":763,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_platform::collections::hash_map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_platform","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_platform-0.16.0/src/collections/hash_map.rs
//     evidence={"module":194,"record_offset":46676,"parent_record_offset":46620,"ancestor_record_offset":42400,"inlinee":5614,"source_line":933,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\any.rs
//     evidence={"module":194,"record_offset":47500,"parent_record_offset":47476,"ancestor_record_offset":42400,"inlinee":5068,"source_line":719,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impl$0::clone_to_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":194,"record_offset":51636,"parent_record_offset":51452,"ancestor_record_offset":42400,"inlinee":6154,"source_line":275,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::uninit::impl$1::clone_one
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone\uninit.rs
//     evidence={"module":194,"record_offset":51660,"parent_record_offset":51636,"ancestor_record_offset":42400,"inlinee":6836,"source_line":56,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":50100,"parent_record_offset":50072,"ancestor_record_offset":42400,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":65772,"parent_record_offset":65748,"ancestor_record_offset":42400,"inlinee":11109,"source_line":1425,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":47548,"parent_record_offset":47524,"ancestor_record_offset":42400,"inlinee":5070,"source_line":1740,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":65820,"parent_record_offset":65796,"ancestor_record_offset":42400,"inlinee":14516,"source_line":1820,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":56920,"parent_record_offset":56892,"ancestor_record_offset":42400,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":194,"record_offset":46276,"parent_record_offset":46248,"ancestor_record_offset":42400,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$108::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":194,"record_offset":46300,"parent_record_offset":46276,"ancestor_record_offset":42400,"inlinee":5955,"source_line":77,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":194,"record_offset":46948,"parent_record_offset":46924,"ancestor_record_offset":42400,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":194,"record_offset":47032,"parent_record_offset":47008,"ancestor_record_offset":42400,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":194,"record_offset":46972,"parent_record_offset":46948,"ancestor_record_offset":42400,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":194,"record_offset":47184,"parent_record_offset":47156,"ancestor_record_offset":42400,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":194,"record_offset":47156,"parent_record_offset":47128,"ancestor_record_offset":42400,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":194,"record_offset":43556,"parent_record_offset":43492,"ancestor_record_offset":42400,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":43580,"parent_record_offset":43556,"ancestor_record_offset":42400,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":43620,"parent_record_offset":43440,"ancestor_record_offset":42400,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":47620,"parent_record_offset":46620,"ancestor_record_offset":42400,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":67140,"parent_record_offset":65860,"ancestor_record_offset":42400,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":43356,"parent_record_offset":43328,"ancestor_record_offset":42400,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":51364,"parent_record_offset":51300,"ancestor_record_offset":42400,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=71
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":49028,"parent_record_offset":47660,"ancestor_record_offset":42400,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":47416,"parent_record_offset":47392,"ancestor_record_offset":42400,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":194,"record_offset":57288,"parent_record_offset":57256,"ancestor_record_offset":42400,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":43264,"parent_record_offset":43104,"ancestor_record_offset":42400,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":194,"record_offset":68572,"parent_record_offset":68548,"ancestor_record_offset":42400,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":194,"record_offset":56992,"parent_record_offset":56968,"ancestor_record_offset":42400,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":194,"record_offset":57016,"parent_record_offset":56992,"ancestor_record_offset":42400,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":194,"record_offset":43492,"parent_record_offset":43464,"ancestor_record_offset":42400,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":43872,"parent_record_offset":43848,"ancestor_record_offset":42400,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":43528,"parent_record_offset":43492,"ancestor_record_offset":42400,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":43848,"parent_record_offset":43668,"ancestor_record_offset":42400,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":194,"record_offset":56944,"parent_record_offset":56920,"ancestor_record_offset":42400,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":48000,"parent_record_offset":47976,"ancestor_record_offset":42400,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":52016,"parent_record_offset":51820,"ancestor_record_offset":42400,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":47868,"parent_record_offset":47748,"ancestor_record_offset":42400,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":49120,"parent_record_offset":49084,"ancestor_record_offset":42400,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":44148,"parent_record_offset":44124,"ancestor_record_offset":42400,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":67248,"parent_record_offset":67216,"ancestor_record_offset":42400,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":52108,"parent_record_offset":52076,"ancestor_record_offset":42400,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":51948,"parent_record_offset":51916,"ancestor_record_offset":42400,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":57316,"parent_record_offset":57256,"ancestor_record_offset":42400,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":67272,"parent_record_offset":67248,"ancestor_record_offset":42400,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":47892,"parent_record_offset":47868,"ancestor_record_offset":42400,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":44172,"parent_record_offset":44148,"ancestor_record_offset":42400,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":52040,"parent_record_offset":52016,"ancestor_record_offset":42400,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":49144,"parent_record_offset":49120,"ancestor_record_offset":42400,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":52132,"parent_record_offset":52108,"ancestor_record_offset":42400,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":73616,"parent_record_offset":73488,"ancestor_record_offset":42400,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":194,"record_offset":47524,"parent_record_offset":47500,"ancestor_record_offset":42400,"inlinee":4662,"source_line":30,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] equivalent::impl$0::equivalent
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"equivalent","version":"1.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/equivalent-1.0.1/src/lib.rs
//     evidence={"module":194,"record_offset":47476,"parent_record_offset":47452,"ancestor_record_offset":42400,"inlinee":4660,"source_line":88,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":194,"record_offset":47092,"parent_record_offset":47064,"ancestor_record_offset":42400,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":194,"record_offset":47128,"parent_record_offset":47092,"ancestor_record_offset":42400,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":194,"record_offset":47064,"parent_record_offset":46844,"ancestor_record_offset":42400,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":194,"record_offset":46924,"parent_record_offset":46844,"ancestor_record_offset":42400,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":194,"record_offset":47228,"parent_record_offset":46844,"ancestor_record_offset":42400,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":194,"record_offset":47008,"parent_record_offset":46844,"ancestor_record_offset":42400,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":194,"record_offset":46720,"parent_record_offset":46676,"ancestor_record_offset":42400,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":194,"record_offset":46756,"parent_record_offset":46720,"ancestor_record_offset":42400,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":194,"record_offset":47452,"parent_record_offset":47344,"ancestor_record_offset":42400,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":47392,"parent_record_offset":47368,"ancestor_record_offset":42400,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":47312,"parent_record_offset":46844,"ancestor_record_offset":42400,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":47368,"parent_record_offset":47344,"ancestor_record_offset":42400,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":46820,"parent_record_offset":46796,"ancestor_record_offset":42400,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":46796,"parent_record_offset":46756,"ancestor_record_offset":42400,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":46844,"parent_record_offset":46820,"ancestor_record_offset":42400,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":46896,"parent_record_offset":46844,"ancestor_record_offset":42400,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":194,"record_offset":47344,"parent_record_offset":46844,"ancestor_record_offset":42400,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$33::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":194,"record_offset":57164,"parent_record_offset":57128,"ancestor_record_offset":42400,"inlinee":9881,"source_line":37,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] smol_str::impl$37::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smol_str","version":"0.2.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smol_str-0.2.0/src/lib.rs
//     evidence={"module":194,"record_offset":57192,"parent_record_offset":57164,"ancestor_record_offset":42400,"inlinee":9883,"source_line":410,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":194,"record_offset":51916,"parent_record_offset":51888,"ancestor_record_offset":42400,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":194,"record_offset":51888,"parent_record_offset":51860,"ancestor_record_offset":42400,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":47952,"parent_record_offset":47928,"ancestor_record_offset":42400,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":51820,"parent_record_offset":51772,"ancestor_record_offset":42400,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":47976,"parent_record_offset":47952,"ancestor_record_offset":42400,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":67336,"parent_record_offset":67308,"ancestor_record_offset":42400,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":67308,"parent_record_offset":67176,"ancestor_record_offset":42400,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":51772,"parent_record_offset":51736,"ancestor_record_offset":42400,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":194,"record_offset":47712,"parent_record_offset":47660,"ancestor_record_offset":42400,"inlinee":8101,"source_line":367,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":194,"record_offset":47928,"parent_record_offset":47712,"ancestor_record_offset":42400,"inlinee":7022,"source_line":682,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$31::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":194,"record_offset":49056,"parent_record_offset":49028,"ancestor_record_offset":42400,"inlinee":7028,"source_line":816,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":194,"record_offset":67216,"parent_record_offset":67176,"ancestor_record_offset":42400,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":194,"record_offset":52076,"parent_record_offset":51772,"ancestor_record_offset":42400,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":194,"record_offset":47748,"parent_record_offset":47712,"ancestor_record_offset":42400,"inlinee":7017,"source_line":94,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":194,"record_offset":49084,"parent_record_offset":49056,"ancestor_record_offset":42400,"inlinee":7029,"source_line":110,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":194,"record_offset":47840,"parent_record_offset":47748,"ancestor_record_offset":42400,"inlinee":7019,"source_line":47,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":194,"record_offset":51860,"parent_record_offset":51820,"ancestor_record_offset":42400,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::event_loop::impl$5::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/event_loop.rs
//     evidence={"module":194,"record_offset":65796,"parent_record_offset":65772,"ancestor_record_offset":42400,"inlinee":14515,"source_line":83,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::process_app_event::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":68548,"parent_record_offset":68520,"ancestor_record_offset":42400,"inlinee":14522,"source_line":422,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::Pixel::cast
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":194,"record_offset":46248,"parent_record_offset":46216,"ancestor_record_offset":42400,"inlinee":5953,"source_line":109,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::dpi::impl$31::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/dpi.rs
//     evidence={"module":194,"record_offset":46216,"parent_record_offset":46188,"ancestor_record_offset":42400,"inlinee":5972,"source_line":466,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::keyboard::_::InternalBitFlags::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"2.9.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-2.9.0/src/lib.rs
//     evidence={"module":194,"record_offset":56852,"parent_record_offset":56820,"ancestor_record_offset":42400,"inlinee":14512,"source_line":703,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$1::from
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/mod.rs
//     evidence={"module":194,"record_offset":52256,"parent_record_offset":52232,"ancestor_record_offset":42400,"inlinee":14508,"source_line":46,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$2::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/mod.rs
//     evidence={"module":194,"record_offset":51396,"parent_record_offset":51364,"ancestor_record_offset":42400,"inlinee":6875,"source_line":29,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$8::clone
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":194,"record_offset":51424,"parent_record_offset":51396,"ancestor_record_offset":42400,"inlinee":6832,"source_line":28,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::current_monitor
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":194,"record_offset":57436,"parent_record_offset":57412,"ancestor_record_offset":42400,"inlinee":11205,"source_line":1023,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::fullscreen
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":194,"record_offset":51300,"parent_record_offset":51248,"ancestor_record_offset":42400,"inlinee":14504,"source_line":680,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":194,"record_offset":51192,"parent_record_offset":51136,"ancestor_record_offset":42400,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::fullscreen::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":51248,"parent_record_offset":51192,"ancestor_record_offset":42400,"inlinee":14503,"source_line":1109,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::fullscreen::closure$0::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":52208,"parent_record_offset":52180,"ancestor_record_offset":42400,"inlinee":14506,"source_line":1109,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::current_monitor::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":57412,"parent_record_offset":57388,"ancestor_record_offset":42400,"inlinee":11204,"source_line":1490,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::process_app_event::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 422
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x38BA
// 首条位置: {"module":194,"inlinee":14522}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":194,"record_offset":68572,"parent_record_offset":68548,"ancestor_record_offset":68548,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":68800,"parent_record_offset":68776,"ancestor_record_offset":68548,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":68776,"parent_record_offset":68720,"ancestor_record_offset":68548,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":68748,"parent_record_offset":68720,"ancestor_record_offset":68548,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":68720,"parent_record_offset":68696,"ancestor_record_offset":68548,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":68696,"parent_record_offset":68672,"ancestor_record_offset":68548,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":68600,"parent_record_offset":68572,"ancestor_record_offset":68548,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::simulate_and_render_frame
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 534
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x38C3
// 首条位置: {"module":194,"inlinee":14531}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":87948,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::contains_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":107008,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":9218,"source_line":1825,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":77408,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":8096,"source_line":2015,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":86944,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::array::equality::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":194,"record_offset":80540,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":14532,"source_line":13,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":88012,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":194,"record_offset":111552,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":106976,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":110532,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":112804,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":89788,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":194,"record_offset":97664,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":96036,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] puffin::now_ns
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":97140,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":9050,"source_line":538,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":94944,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::temporal::impl$18::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":194,"record_offset":88076,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":14540,"source_line":79,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":194,"record_offset":106504,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":7835,"source_line":607,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::egui_backend::EguiBackend::take_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/egui_backend.rs
//     evidence={"module":194,"record_offset":112772,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":8298,"source_line":45,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::game_loop::GameLoop::maintain_resolution_scale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":80636,"parent_record_offset":76896,"ancestor_record_offset":76896,"inlinee":14533,"source_line":781,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":90328,"parent_record_offset":90296,"ancestor_record_offset":76896,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":90296,"parent_record_offset":90128,"ancestor_record_offset":76896,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":77856,"parent_record_offset":77832,"ancestor_record_offset":76896,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":106832,"parent_record_offset":106804,"ancestor_record_offset":76896,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":106804,"parent_record_offset":106776,"ancestor_record_offset":76896,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":77904,"parent_record_offset":77880,"ancestor_record_offset":76896,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":77880,"parent_record_offset":77856,"ancestor_record_offset":76896,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":106776,"parent_record_offset":106752,"ancestor_record_offset":76896,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":77832,"parent_record_offset":77808,"ancestor_record_offset":76896,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":77808,"parent_record_offset":77768,"ancestor_record_offset":76896,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":77768,"parent_record_offset":77744,"ancestor_record_offset":76896,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":83700,"parent_record_offset":83676,"ancestor_record_offset":76896,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":83676,"parent_record_offset":80636,"ancestor_record_offset":76896,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":78156,"parent_record_offset":78132,"ancestor_record_offset":76896,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":78132,"parent_record_offset":78108,"ancestor_record_offset":76896,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":78080,"parent_record_offset":78048,"ancestor_record_offset":76896,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":78024,"parent_record_offset":77488,"ancestor_record_offset":76896,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":83084,"parent_record_offset":82544,"ancestor_record_offset":76896,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":78048,"parent_record_offset":78024,"ancestor_record_offset":76896,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":77516,"parent_record_offset":77488,"ancestor_record_offset":76896,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":78108,"parent_record_offset":78048,"ancestor_record_offset":76896,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":83168,"parent_record_offset":83108,"ancestor_record_offset":76896,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":77564,"parent_record_offset":77540,"ancestor_record_offset":76896,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":77540,"parent_record_offset":77516,"ancestor_record_offset":76896,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":77744,"parent_record_offset":77720,"ancestor_record_offset":76896,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":80828,"parent_record_offset":80636,"ancestor_record_offset":76896,"inlinee":8096,"source_line":2015,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":82464,"parent_record_offset":80636,"ancestor_record_offset":76896,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":83284,"parent_record_offset":83260,"ancestor_record_offset":76896,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::impl$2::contains_resource::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":107072,"parent_record_offset":107044,"ancestor_record_offset":76896,"inlinee":9220,"source_line":1828,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":83260,"parent_record_offset":82544,"ancestor_record_offset":76896,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":77432,"parent_record_offset":77408,"ancestor_record_offset":76896,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":77488,"parent_record_offset":77432,"ancestor_record_offset":76896,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":82488,"parent_record_offset":82464,"ancestor_record_offset":76896,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":82544,"parent_record_offset":82488,"ancestor_record_offset":76896,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":194,"record_offset":80596,"parent_record_offset":80568,"ancestor_record_offset":76896,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":194,"record_offset":80568,"parent_record_offset":80540,"ancestor_record_offset":76896,"inlinee":9488,"source_line":152,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":96456,"parent_record_offset":96424,"ancestor_record_offset":76896,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":88684,"parent_record_offset":88652,"ancestor_record_offset":76896,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":88752,"parent_record_offset":88716,"ancestor_record_offset":76896,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":96484,"parent_record_offset":96456,"ancestor_record_offset":76896,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":96392,"parent_record_offset":96356,"ancestor_record_offset":76896,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":96424,"parent_record_offset":96392,"ancestor_record_offset":76896,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":96616,"parent_record_offset":96424,"ancestor_record_offset":76896,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":96736,"parent_record_offset":96712,"ancestor_record_offset":76896,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":84928,"parent_record_offset":80636,"ancestor_record_offset":76896,"inlinee":11953,"source_line":1063,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":84760,"parent_record_offset":80636,"ancestor_record_offset":76896,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":83732,"parent_record_offset":80636,"ancestor_record_offset":76896,"inlinee":11109,"source_line":1425,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":83468,"parent_record_offset":80636,"ancestor_record_offset":76896,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":84784,"parent_record_offset":84760,"ancestor_record_offset":76896,"inlinee":5087,"source_line":1371,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":83836,"parent_record_offset":83812,"ancestor_record_offset":76896,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":194,"record_offset":84000,"parent_record_offset":83944,"ancestor_record_offset":76896,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":194,"record_offset":88532,"parent_record_offset":88504,"ancestor_record_offset":76896,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":90224,"parent_record_offset":90196,"ancestor_record_offset":76896,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":194,"record_offset":95888,"parent_record_offset":95852,"ancestor_record_offset":76896,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":83944,"parent_record_offset":83908,"ancestor_record_offset":76896,"inlinee":9728,"source_line":870,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":83972,"parent_record_offset":83944,"ancestor_record_offset":76896,"inlinee":9729,"source_line":2579,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":88716,"parent_record_offset":88652,"ancestor_record_offset":76896,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":194,"record_offset":77656,"parent_record_offset":77592,"ancestor_record_offset":76896,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":107044,"parent_record_offset":107008,"ancestor_record_offset":76896,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":77680,"parent_record_offset":77656,"ancestor_record_offset":76896,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":84036,"parent_record_offset":83880,"ancestor_record_offset":76896,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":107600,"parent_record_offset":107008,"ancestor_record_offset":76896,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":77720,"parent_record_offset":77540,"ancestor_record_offset":76896,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":85988,"parent_record_offset":80636,"ancestor_record_offset":76896,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":88384,"parent_record_offset":88356,"ancestor_record_offset":76896,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":88040,"parent_record_offset":88012,"ancestor_record_offset":76896,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":77460,"parent_record_offset":77432,"ancestor_record_offset":76896,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":194,"record_offset":83556,"parent_record_offset":83524,"ancestor_record_offset":76896,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":194,"record_offset":83584,"parent_record_offset":83524,"ancestor_record_offset":76896,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":194,"record_offset":95980,"parent_record_offset":95952,"ancestor_record_offset":76896,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":194,"record_offset":96320,"parent_record_offset":96172,"ancestor_record_offset":76896,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=65
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":89668,"parent_record_offset":88076,"ancestor_record_offset":76896,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":88476,"parent_record_offset":88356,"ancestor_record_offset":76896,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":90260,"parent_record_offset":90164,"ancestor_record_offset":76896,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":194,"record_offset":95412,"parent_record_offset":95384,"ancestor_record_offset":76896,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":194,"record_offset":95384,"parent_record_offset":95356,"ancestor_record_offset":76896,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":194,"record_offset":95620,"parent_record_offset":95588,"ancestor_record_offset":76896,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":88560,"parent_record_offset":88532,"ancestor_record_offset":76896,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":88504,"parent_record_offset":88476,"ancestor_record_offset":76896,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":88592,"parent_record_offset":88532,"ancestor_record_offset":76896,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":194,"record_offset":77592,"parent_record_offset":77564,"ancestor_record_offset":76896,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":77972,"parent_record_offset":77948,"ancestor_record_offset":76896,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":77628,"parent_record_offset":77592,"ancestor_record_offset":76896,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":77948,"parent_record_offset":77768,"ancestor_record_offset":76896,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":194,"record_offset":95328,"parent_record_offset":95300,"ancestor_record_offset":76896,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":194,"record_offset":95356,"parent_record_offset":95328,"ancestor_record_offset":76896,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":194,"record_offset":95700,"parent_record_offset":95668,"ancestor_record_offset":76896,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":194,"record_offset":95300,"parent_record_offset":95176,"ancestor_record_offset":76896,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":194,"record_offset":95852,"parent_record_offset":95820,"ancestor_record_offset":76896,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":194,"record_offset":94980,"parent_record_offset":94944,"ancestor_record_offset":76896,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":194,"record_offset":95140,"parent_record_offset":95104,"ancestor_record_offset":76896,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":194,"record_offset":95104,"parent_record_offset":95064,"ancestor_record_offset":76896,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":194,"record_offset":95036,"parent_record_offset":95008,"ancestor_record_offset":76896,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":194,"record_offset":95064,"parent_record_offset":95036,"ancestor_record_offset":76896,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":194,"record_offset":95008,"parent_record_offset":94980,"ancestor_record_offset":76896,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":194,"record_offset":95668,"parent_record_offset":95064,"ancestor_record_offset":76896,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":194,"record_offset":95820,"parent_record_offset":95788,"ancestor_record_offset":76896,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":194,"record_offset":95924,"parent_record_offset":95820,"ancestor_record_offset":76896,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":194,"record_offset":95788,"parent_record_offset":95756,"ancestor_record_offset":76896,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":194,"record_offset":95756,"parent_record_offset":94944,"ancestor_record_offset":76896,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":194,"record_offset":95952,"parent_record_offset":95924,"ancestor_record_offset":76896,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":194,"record_offset":95176,"parent_record_offset":95140,"ancestor_record_offset":76896,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":194,"record_offset":95492,"parent_record_offset":95176,"ancestor_record_offset":76896,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":194,"record_offset":95460,"parent_record_offset":95176,"ancestor_record_offset":76896,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":194,"record_offset":95236,"parent_record_offset":95176,"ancestor_record_offset":76896,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":97344,"parent_record_offset":97316,"ancestor_record_offset":76896,"inlinee":6050,"source_line":1566,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":83308,"parent_record_offset":83284,"ancestor_record_offset":76896,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":97856,"parent_record_offset":97788,"ancestor_record_offset":76896,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":83408,"parent_record_offset":83384,"ancestor_record_offset":76896,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":97884,"parent_record_offset":97856,"ancestor_record_offset":76896,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":83332,"parent_record_offset":83308,"ancestor_record_offset":76896,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":194,"record_offset":83908,"parent_record_offset":83880,"ancestor_record_offset":76896,"inlinee":14538,"source_line":749,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":194,"record_offset":83812,"parent_record_offset":83788,"ancestor_record_offset":76896,"inlinee":5095,"source_line":79,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$5::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":194,"record_offset":83880,"parent_record_offset":80636,"ancestor_record_offset":76896,"inlinee":14537,"source_line":1143,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::adaptive_render_state::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/adaptive_render_state.rs
//     evidence={"module":194,"record_offset":81632,"parent_record_offset":80636,"ancestor_record_offset":76896,"inlinee":14535,"source_line":5,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":194,"record_offset":90164,"parent_record_offset":90128,"ancestor_record_offset":76896,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":194,"record_offset":90064,"parent_record_offset":90036,"ancestor_record_offset":76896,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":194,"record_offset":90128,"parent_record_offset":90064,"ancestor_record_offset":76896,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":194,"record_offset":90096,"parent_record_offset":90064,"ancestor_record_offset":76896,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":194,"record_offset":90196,"parent_record_offset":90164,"ancestor_record_offset":76896,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":194,"record_offset":89740,"parent_record_offset":89716,"ancestor_record_offset":76896,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":194,"record_offset":97756,"parent_record_offset":97728,"ancestor_record_offset":76896,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":83612,"parent_record_offset":83524,"ancestor_record_offset":76896,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":194,"record_offset":83500,"parent_record_offset":80636,"ancestor_record_offset":76896,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":194,"record_offset":83524,"parent_record_offset":83500,"ancestor_record_offset":76896,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":83636,"parent_record_offset":83612,"ancestor_record_offset":76896,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":83384,"parent_record_offset":80636,"ancestor_record_offset":76896,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":194,"record_offset":97316,"parent_record_offset":97288,"ancestor_record_offset":76896,"inlinee":6048,"source_line":49,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":194,"record_offset":97200,"parent_record_offset":97168,"ancestor_record_offset":76896,"inlinee":6042,"source_line":1308,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":194,"record_offset":97288,"parent_record_offset":97256,"ancestor_record_offset":76896,"inlinee":6046,"source_line":963,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":194,"record_offset":97228,"parent_record_offset":97200,"ancestor_record_offset":76896,"inlinee":6044,"source_line":1115,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":194,"record_offset":97256,"parent_record_offset":97228,"ancestor_record_offset":76896,"inlinee":6045,"source_line":1152,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":194,"record_offset":97168,"parent_record_offset":97140,"ancestor_record_offset":76896,"inlinee":6040,"source_line":1375,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":194,"record_offset":97788,"parent_record_offset":97756,"ancestor_record_offset":76896,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":194,"record_offset":97824,"parent_record_offset":97788,"ancestor_record_offset":76896,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":96080,"parent_record_offset":96036,"ancestor_record_offset":76896,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":102012,"parent_record_offset":101972,"ancestor_record_offset":76896,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":102632,"parent_record_offset":102332,"ancestor_record_offset":76896,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":194,"record_offset":96240,"parent_record_offset":96212,"ancestor_record_offset":76896,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":96356,"parent_record_offset":96172,"ancestor_record_offset":76896,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":194,"record_offset":96656,"parent_record_offset":96356,"ancestor_record_offset":76896,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":194,"record_offset":88872,"parent_record_offset":88116,"ancestor_record_offset":76896,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":194,"record_offset":88116,"parent_record_offset":88076,"ancestor_record_offset":76896,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":194,"record_offset":88172,"parent_record_offset":88144,"ancestor_record_offset":76896,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":194,"record_offset":88436,"parent_record_offset":88412,"ancestor_record_offset":76896,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":194,"record_offset":88292,"parent_record_offset":88264,"ancestor_record_offset":76896,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":194,"record_offset":88652,"parent_record_offset":88228,"ancestor_record_offset":76896,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":194,"record_offset":88144,"parent_record_offset":88116,"ancestor_record_offset":76896,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$20::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":194,"record_offset":83788,"parent_record_offset":83764,"ancestor_record_offset":76896,"inlinee":5093,"source_line":14,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":194,"record_offset":88324,"parent_record_offset":88292,"ancestor_record_offset":76896,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":194,"record_offset":88356,"parent_record_offset":88324,"ancestor_record_offset":76896,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":194,"record_offset":88228,"parent_record_offset":88200,"ancestor_record_offset":76896,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":194,"record_offset":88200,"parent_record_offset":88172,"ancestor_record_offset":76896,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$23::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":194,"record_offset":83764,"parent_record_offset":83732,"ancestor_record_offset":76896,"inlinee":5091,"source_line":149,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":110560,"parent_record_offset":110532,"ancestor_record_offset":76896,"inlinee":14542,"source_line":759,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/game_loop.rs
//     evidence={"module":194,"record_offset":111520,"parent_record_offset":111492,"ancestor_record_offset":76896,"inlinee":14543,"source_line":767,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 759
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x38CE
// 首条位置: {"module":194,"inlinee":14542}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 767
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x38CF
// 首条位置: {"module":194,"inlinee":14543}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::maintain_resolution_scale
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 781
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x38C5
// 首条位置: {"module":194,"inlinee":14533}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":83676,"parent_record_offset":80636,"ancestor_record_offset":80636,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":80828,"parent_record_offset":80636,"ancestor_record_offset":80636,"inlinee":8096,"source_line":2015,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":82464,"parent_record_offset":80636,"ancestor_record_offset":80636,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":84928,"parent_record_offset":80636,"ancestor_record_offset":80636,"inlinee":11953,"source_line":1063,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":84760,"parent_record_offset":80636,"ancestor_record_offset":80636,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":83732,"parent_record_offset":80636,"ancestor_record_offset":80636,"inlinee":11109,"source_line":1425,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":83468,"parent_record_offset":80636,"ancestor_record_offset":80636,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":85988,"parent_record_offset":80636,"ancestor_record_offset":80636,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::time::impl$5::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":194,"record_offset":83880,"parent_record_offset":80636,"ancestor_record_offset":80636,"inlinee":14537,"source_line":1143,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::adaptive_render_state::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/adaptive_render_state.rs
//     evidence={"module":194,"record_offset":81632,"parent_record_offset":80636,"ancestor_record_offset":80636,"inlinee":14535,"source_line":5,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":194,"record_offset":83500,"parent_record_offset":80636,"ancestor_record_offset":80636,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":83384,"parent_record_offset":80636,"ancestor_record_offset":80636,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":81276,"parent_record_offset":81252,"ancestor_record_offset":80636,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":81324,"parent_record_offset":81300,"ancestor_record_offset":80636,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":81300,"parent_record_offset":81276,"ancestor_record_offset":80636,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":81252,"parent_record_offset":81228,"ancestor_record_offset":80636,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":81228,"parent_record_offset":81188,"ancestor_record_offset":80636,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":81188,"parent_record_offset":81164,"ancestor_record_offset":80636,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":83700,"parent_record_offset":83676,"ancestor_record_offset":80636,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":81576,"parent_record_offset":81552,"ancestor_record_offset":80636,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":81552,"parent_record_offset":81528,"ancestor_record_offset":80636,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":81500,"parent_record_offset":81468,"ancestor_record_offset":80636,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":81444,"parent_record_offset":80908,"ancestor_record_offset":80636,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":83084,"parent_record_offset":82544,"ancestor_record_offset":80636,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":81468,"parent_record_offset":81444,"ancestor_record_offset":80636,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":80936,"parent_record_offset":80908,"ancestor_record_offset":80636,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":81528,"parent_record_offset":81468,"ancestor_record_offset":80636,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":83168,"parent_record_offset":83108,"ancestor_record_offset":80636,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":80984,"parent_record_offset":80960,"ancestor_record_offset":80636,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":80960,"parent_record_offset":80936,"ancestor_record_offset":80636,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":81164,"parent_record_offset":81140,"ancestor_record_offset":80636,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":83284,"parent_record_offset":83260,"ancestor_record_offset":80636,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":83260,"parent_record_offset":82544,"ancestor_record_offset":80636,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":80852,"parent_record_offset":80828,"ancestor_record_offset":80636,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":80908,"parent_record_offset":80852,"ancestor_record_offset":80636,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":82488,"parent_record_offset":82464,"ancestor_record_offset":80636,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":82544,"parent_record_offset":82488,"ancestor_record_offset":80636,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":84784,"parent_record_offset":84760,"ancestor_record_offset":80636,"inlinee":5087,"source_line":1371,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":83836,"parent_record_offset":83812,"ancestor_record_offset":80636,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":194,"record_offset":84000,"parent_record_offset":83944,"ancestor_record_offset":80636,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":83944,"parent_record_offset":83908,"ancestor_record_offset":80636,"inlinee":9728,"source_line":870,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":83972,"parent_record_offset":83944,"ancestor_record_offset":80636,"inlinee":9729,"source_line":2579,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":194,"record_offset":81076,"parent_record_offset":81012,"ancestor_record_offset":80636,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":81100,"parent_record_offset":81076,"ancestor_record_offset":80636,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":84036,"parent_record_offset":83880,"ancestor_record_offset":80636,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":81140,"parent_record_offset":80960,"ancestor_record_offset":80636,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":80880,"parent_record_offset":80852,"ancestor_record_offset":80636,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":194,"record_offset":83556,"parent_record_offset":83524,"ancestor_record_offset":80636,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":194,"record_offset":83584,"parent_record_offset":83524,"ancestor_record_offset":80636,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":194,"record_offset":81012,"parent_record_offset":80984,"ancestor_record_offset":80636,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":81392,"parent_record_offset":81368,"ancestor_record_offset":80636,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":81048,"parent_record_offset":81012,"ancestor_record_offset":80636,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":81368,"parent_record_offset":81188,"ancestor_record_offset":80636,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":83308,"parent_record_offset":83284,"ancestor_record_offset":80636,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":83408,"parent_record_offset":83384,"ancestor_record_offset":80636,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":83332,"parent_record_offset":83308,"ancestor_record_offset":80636,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":194,"record_offset":83908,"parent_record_offset":83880,"ancestor_record_offset":80636,"inlinee":14538,"source_line":749,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":194,"record_offset":83812,"parent_record_offset":83788,"ancestor_record_offset":80636,"inlinee":5095,"source_line":79,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":83612,"parent_record_offset":83524,"ancestor_record_offset":80636,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":194,"record_offset":83524,"parent_record_offset":83500,"ancestor_record_offset":80636,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":83636,"parent_record_offset":83612,"ancestor_record_offset":80636,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$20::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":194,"record_offset":83788,"parent_record_offset":83764,"ancestor_record_offset":80636,"inlinee":5093,"source_line":14,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$23::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":194,"record_offset":83764,"parent_record_offset":83732,"ancestor_record_offset":80636,"inlinee":5091,"source_line":149,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::apply_setting_changes
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 960
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3898
// 首条位置: {"module":194,"inlinee":14488}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":22744,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::collections::Events::send_default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":194,"record_offset":22808,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":9990,"source_line":164,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":194,"record_offset":18320,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":18448,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":194,"record_offset":18272,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::render_config::impl$22::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":194,"record_offset":20988,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":10686,"source_line":48,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":194,"record_offset":20000,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":194,"record_offset":18932,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":11668,"source_line":41,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Sender::send
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":194,"record_offset":21204,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":7835,"source_line":607,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":17768,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::PendingVideoSettingChanges::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":194,"record_offset":20956,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":14545,"source_line":603,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::PendingVideoSettingChanges::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":194,"record_offset":18900,"parent_record_offset":17620,"ancestor_record_offset":17620,"inlinee":14544,"source_line":585,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":21592,"parent_record_offset":21564,"ancestor_record_offset":17620,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":21564,"parent_record_offset":21504,"ancestor_record_offset":17620,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":19444,"parent_record_offset":19416,"ancestor_record_offset":17620,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":21532,"parent_record_offset":21504,"ancestor_record_offset":17620,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":21504,"parent_record_offset":21476,"ancestor_record_offset":17620,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":19500,"parent_record_offset":19472,"ancestor_record_offset":17620,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":19472,"parent_record_offset":19444,"ancestor_record_offset":17620,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":21476,"parent_record_offset":21452,"ancestor_record_offset":17620,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":22948,"parent_record_offset":22904,"ancestor_record_offset":17620,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":19416,"parent_record_offset":19388,"ancestor_record_offset":17620,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":19388,"parent_record_offset":19344,"ancestor_record_offset":17620,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":22904,"parent_record_offset":22864,"ancestor_record_offset":17620,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":19344,"parent_record_offset":19316,"ancestor_record_offset":17620,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":22772,"parent_record_offset":22744,"ancestor_record_offset":17620,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":194,"record_offset":22836,"parent_record_offset":22808,"ancestor_record_offset":17620,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":194,"record_offset":22864,"parent_record_offset":22836,"ancestor_record_offset":17620,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":19780,"parent_record_offset":19752,"ancestor_record_offset":17620,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":19752,"parent_record_offset":19724,"ancestor_record_offset":17620,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":19692,"parent_record_offset":19660,"ancestor_record_offset":17620,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":20708,"parent_record_offset":20104,"ancestor_record_offset":17620,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":19632,"parent_record_offset":19028,"ancestor_record_offset":17620,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":19660,"parent_record_offset":19632,"ancestor_record_offset":17620,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":19064,"parent_record_offset":19028,"ancestor_record_offset":17620,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":20808,"parent_record_offset":20740,"ancestor_record_offset":17620,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":19724,"parent_record_offset":19660,"ancestor_record_offset":17620,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":19120,"parent_record_offset":19092,"ancestor_record_offset":17620,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":19092,"parent_record_offset":19064,"ancestor_record_offset":17620,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":19316,"parent_record_offset":19288,"ancestor_record_offset":17620,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":19856,"parent_record_offset":19828,"ancestor_record_offset":17620,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":19828,"parent_record_offset":19028,"ancestor_record_offset":17620,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":20036,"parent_record_offset":20000,"ancestor_record_offset":17620,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":20104,"parent_record_offset":20036,"ancestor_record_offset":17620,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":18964,"parent_record_offset":18932,"ancestor_record_offset":17620,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":19028,"parent_record_offset":18964,"ancestor_record_offset":17620,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":194,"record_offset":18348,"parent_record_offset":18320,"ancestor_record_offset":17620,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":194,"record_offset":19220,"parent_record_offset":19152,"ancestor_record_offset":17620,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":19244,"parent_record_offset":19220,"ancestor_record_offset":17620,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":19288,"parent_record_offset":19092,"ancestor_record_offset":17620,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":19964,"parent_record_offset":18932,"ancestor_record_offset":17620,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":18996,"parent_record_offset":18964,"ancestor_record_offset":17620,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":21084,"parent_record_offset":20988,"ancestor_record_offset":17620,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":21260,"parent_record_offset":21236,"ancestor_record_offset":17620,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":18376,"parent_record_offset":18348,"ancestor_record_offset":17620,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":18408,"parent_record_offset":18348,"ancestor_record_offset":17620,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":194,"record_offset":19152,"parent_record_offset":19120,"ancestor_record_offset":17620,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":19576,"parent_record_offset":19548,"ancestor_record_offset":17620,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":19188,"parent_record_offset":19152,"ancestor_record_offset":17620,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":19548,"parent_record_offset":19344,"ancestor_record_offset":17620,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":18184,"parent_record_offset":18152,"ancestor_record_offset":17620,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":18724,"parent_record_offset":18520,"ancestor_record_offset":17620,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":19884,"parent_record_offset":19856,"ancestor_record_offset":17620,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":17840,"parent_record_offset":17808,"ancestor_record_offset":17620,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":18824,"parent_record_offset":18792,"ancestor_record_offset":17620,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":18076,"parent_record_offset":18044,"ancestor_record_offset":17620,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":17868,"parent_record_offset":17840,"ancestor_record_offset":17620,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":18104,"parent_record_offset":18076,"ancestor_record_offset":17620,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":18752,"parent_record_offset":18724,"ancestor_record_offset":17620,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":18852,"parent_record_offset":18824,"ancestor_record_offset":17620,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$13::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":194,"record_offset":21024,"parent_record_offset":20988,"ancestor_record_offset":17620,"inlinee":10688,"source_line":16,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":194,"record_offset":21120,"parent_record_offset":21084,"ancestor_record_offset":17620,"inlinee":10692,"source_line":33,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":194,"record_offset":21052,"parent_record_offset":21024,"ancestor_record_offset":17620,"inlinee":10690,"source_line":7,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":194,"record_offset":18044,"parent_record_offset":18012,"ancestor_record_offset":17620,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":194,"record_offset":18012,"parent_record_offset":17980,"ancestor_record_offset":17620,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":18520,"parent_record_offset":18480,"ancestor_record_offset":17620,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":18152,"parent_record_offset":17940,"ancestor_record_offset":17620,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":17940,"parent_record_offset":17908,"ancestor_record_offset":17620,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":17908,"parent_record_offset":17768,"ancestor_record_offset":17620,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":18480,"parent_record_offset":18448,"ancestor_record_offset":17620,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":194,"record_offset":17808,"parent_record_offset":17768,"ancestor_record_offset":17620,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":194,"record_offset":18792,"parent_record_offset":18480,"ancestor_record_offset":17620,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":194,"record_offset":17980,"parent_record_offset":17940,"ancestor_record_offset":17620,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::GameLoop::extract_available_monitors_info
// 记录形式: compiled_procedure; instances=1
// 源码行: 990
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":194,"record_offset":113940,"rva":5121136,"va":5373830256}
// 编译实例 RVAs: 0x4e2470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8813545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e8e97b8600","size":5,"soft_fail":false,"target_va":5382644528,"va":5373830978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=19 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4214228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e82cb2bfff","size":5,"soft_fail":false,"target_va":5369616112,"va":5373830335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140175450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3592619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e8552ec9ff","size":5,"soft_fail":false,"target_va":5370238032,"va":5373830646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402bd540
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2250494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e802a9ddff","size":5,"soft_fail":false,"target_va":5371581760,"va":5373832249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402be600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2246132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e80cbaddff","size":5,"soft_fail":false,"target_va":5371586048,"va":5373832175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037a140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1477001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e87776e9ff","size":5,"soft_fail":false,"target_va":5372354880,"va":5373831876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14052db10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t305751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e857aa0400","size":5,"soft_fail":false,"target_va":5374139152,"va":5373833396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405c3060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t918235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e8db020e00","size":5,"soft_fail":false,"target_va":5374750816,"va":5373832576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1067710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e8be4a1000","size":5,"soft_fail":false,"target_va":5374900736,"va":5373833021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fc1d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1151066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e85a901100","size":5,"soft_fail":false,"target_va":5374984656,"va":5373833585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fcb80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1153079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e837981100","size":5,"soft_fail":false,"target_va":5374987136,"va":5373834052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1250039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e8f7121300","size":5,"soft_fail":false,"target_va":5375081488,"va":5373831444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1250401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e861141300","size":5,"soft_fail":false,"target_va":5375081504,"va":5373831098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1685230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e8eeb61900","size":5,"soft_fail":false,"target_va":5375516032,"va":5373830797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140697a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1791300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e844551b00","size":5,"soft_fail":false,"target_va":5375621776,"va":5373830471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406a6730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1850748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e87c3d1c00","size":5,"soft_fail":false,"target_va":5375682352,"va":5373831599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38015741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e8fd124402","size":5,"soft_fail":false,"target_va":5411847168,"va":5373831422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37708260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e8e4613f02","size":5,"soft_fail":false,"target_va":5411540992,"va":5373832727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t522977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e8e1fa0700","size":5,"soft_fail":false,"target_va":5374355376,"va":5373832394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37709013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e8d5643f02","size":5,"soft_fail":false,"target_va":5411541776,"va":5373832758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37709356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e82c663f02","size":5,"soft_fail":false,"target_va":5411542064,"va":5373832703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t393405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e8bd000600","size":5,"soft_fail":false,"target_va":5374226016,"va":5373832606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::window::AvailableMonitors::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4617995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e80b774600","size":5,"soft_fail":false,"target_va":5378448448,"va":5373830448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] winit::platform_impl::platform::monitor::get_monitor_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     evidence={"assembly":"\tcallq\t1791201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373830256,"raw_hex":"e8e1541b00","size":5,"soft_fail":false,"target_va":5375621904,"va":5373830698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5373830256,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5373831069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":194,"record_offset":119532,"parent_record_offset":113940,"ancestor_record_offset":113940,"inlinee":6034,"source_line":238,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":115108,"parent_record_offset":113940,"ancestor_record_offset":113940,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":119888,"parent_record_offset":113940,"ancestor_record_offset":113940,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":115052,"parent_record_offset":113940,"ancestor_record_offset":113940,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":114132,"parent_record_offset":113940,"ancestor_record_offset":113940,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":194,"record_offset":115220,"parent_record_offset":113940,"ancestor_record_offset":113940,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":194,"record_offset":119448,"parent_record_offset":113940,"ancestor_record_offset":113940,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":120828,"parent_record_offset":113940,"ancestor_record_offset":113940,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":120084,"parent_record_offset":113940,"ancestor_record_offset":113940,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":194,"record_offset":119692,"parent_record_offset":113940,"ancestor_record_offset":113940,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] winit::monitor::MonitorHandle::video_modes
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/monitor.rs
//     evidence={"module":194,"record_offset":115460,"parent_record_offset":113940,"ancestor_record_offset":113940,"inlinee":6021,"source_line":164,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::available_monitors
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":115136,"parent_record_offset":113940,"ancestor_record_offset":113940,"inlinee":11214,"source_line":1499,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":118008,"parent_record_offset":117984,"ancestor_record_offset":113940,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":118032,"parent_record_offset":118008,"ancestor_record_offset":113940,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":116396,"parent_record_offset":116372,"ancestor_record_offset":113940,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":118636,"parent_record_offset":118608,"ancestor_record_offset":113940,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":117984,"parent_record_offset":117960,"ancestor_record_offset":113940,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":116372,"parent_record_offset":116284,"ancestor_record_offset":113940,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":118608,"parent_record_offset":115496,"ancestor_record_offset":113940,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":117928,"parent_record_offset":117892,"ancestor_record_offset":113940,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":117960,"parent_record_offset":117928,"ancestor_record_offset":113940,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":116284,"parent_record_offset":116256,"ancestor_record_offset":113940,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::borrow::DormantMutRef::reborrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\borrow.rs
//     evidence={"module":194,"record_offset":117836,"parent_record_offset":117768,"ancestor_record_offset":113940,"inlinee":14123,"source_line":51,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":194,"record_offset":116932,"parent_record_offset":116872,"ancestor_record_offset":113940,"inlinee":14549,"source_line":1316,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":194,"record_offset":116872,"parent_record_offset":116828,"ancestor_record_offset":113940,"inlinee":14548,"source_line":1023,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::entry::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map\entry.rs
//     evidence={"module":194,"record_offset":117732,"parent_record_offset":116872,"ancestor_record_offset":113940,"inlinee":14550,"source_line":376,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::entry::VacantEntry::insert_entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map\entry.rs
//     evidence={"module":194,"record_offset":117768,"parent_record_offset":117732,"ancestor_record_offset":113940,"inlinee":14131,"source_line":399,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":194,"record_offset":118952,"parent_record_offset":118928,"ancestor_record_offset":113940,"inlinee":6930,"source_line":188,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":194,"record_offset":116796,"parent_record_offset":116772,"ancestor_record_offset":113940,"inlinee":6033,"source_line":1680,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$34::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":194,"record_offset":119040,"parent_record_offset":119016,"ancestor_record_offset":113940,"inlinee":8153,"source_line":1702,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::descend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":194,"record_offset":117444,"parent_record_offset":117172,"ancestor_record_offset":113940,"inlinee":5891,"source_line":1093,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::drop_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":194,"record_offset":119072,"parent_record_offset":119040,"ancestor_record_offset":113940,"inlinee":11900,"source_line":1185,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::force
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":194,"record_offset":117388,"parent_record_offset":117172,"ancestor_record_offset":113940,"inlinee":5889,"source_line":1671,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::LeafNode::init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":194,"record_offset":118076,"parent_record_offset":117892,"ancestor_record_offset":113940,"inlinee":13218,"source_line":70,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::LeafNode::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":194,"record_offset":117892,"parent_record_offset":117864,"ancestor_record_offset":113940,"inlinee":13217,"source_line":81,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":194,"record_offset":117144,"parent_record_offset":116932,"ancestor_record_offset":113940,"inlinee":13236,"source_line":631,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::find_key_index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\search.rs
//     evidence={"module":194,"record_offset":117232,"parent_record_offset":117208,"ancestor_record_offset":113940,"inlinee":5878,"source_line":217,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::force
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":194,"record_offset":117412,"parent_record_offset":117388,"ancestor_record_offset":113940,"inlinee":5890,"source_line":716,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":194,"record_offset":117264,"parent_record_offset":117232,"ancestor_record_offset":113940,"inlinee":5879,"source_line":384,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::new_leaf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":194,"record_offset":117864,"parent_record_offset":117768,"ancestor_record_offset":113940,"inlinee":13261,"source_line":216,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::push_with_handle
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":194,"record_offset":118196,"parent_record_offset":117768,"ancestor_record_offset":113940,"inlinee":13260,"source_line":654,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::search_node
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\search.rs
//     evidence={"module":194,"record_offset":117208,"parent_record_offset":117172,"ancestor_record_offset":113940,"inlinee":5877,"source_line":195,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::search_tree
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\search.rs
//     evidence={"module":194,"record_offset":117172,"parent_record_offset":116932,"ancestor_record_offset":113940,"inlinee":5876,"source_line":49,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::BTreeSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\set.rs
//     evidence={"module":194,"record_offset":116828,"parent_record_offset":115496,"ancestor_record_offset":113940,"inlinee":6029,"source_line":905,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::BTreeSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\set.rs
//     evidence={"module":194,"record_offset":115648,"parent_record_offset":115496,"ancestor_record_offset":113940,"inlinee":6024,"source_line":344,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\set.rs
//     evidence={"module":194,"record_offset":116772,"parent_record_offset":115496,"ancestor_record_offset":113940,"inlinee":6031,"source_line":1536,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":194,"record_offset":115392,"parent_record_offset":115268,"ancestor_record_offset":113940,"inlinee":4907,"source_line":181,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":194,"record_offset":115268,"parent_record_offset":115244,"ancestor_record_offset":113940,"inlinee":4905,"source_line":1698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":194,"record_offset":115300,"parent_record_offset":115268,"ancestor_record_offset":113940,"inlinee":4738,"source_line":216,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":194,"record_offset":115324,"parent_record_offset":115300,"ancestor_record_offset":113940,"inlinee":4739,"source_line":211,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::into_iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\vec_deque\into_iter.rs
//     evidence={"module":194,"record_offset":115244,"parent_record_offset":115220,"ancestor_record_offset":113940,"inlinee":14547,"source_line":47,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":114584,"parent_record_offset":114560,"ancestor_record_offset":113940,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":120408,"parent_record_offset":120380,"ancestor_record_offset":113940,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":120380,"parent_record_offset":120356,"ancestor_record_offset":113940,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":114632,"parent_record_offset":114608,"ancestor_record_offset":113940,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":114608,"parent_record_offset":114584,"ancestor_record_offset":113940,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":120356,"parent_record_offset":120332,"ancestor_record_offset":113940,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":194,"record_offset":119560,"parent_record_offset":119532,"ancestor_record_offset":113940,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":119932,"parent_record_offset":119888,"ancestor_record_offset":113940,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":114560,"parent_record_offset":114536,"ancestor_record_offset":113940,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":114536,"parent_record_offset":114496,"ancestor_record_offset":113940,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":119472,"parent_record_offset":119448,"ancestor_record_offset":113940,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":120108,"parent_record_offset":120084,"ancestor_record_offset":113940,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":114496,"parent_record_offset":114472,"ancestor_record_offset":113940,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":115076,"parent_record_offset":115052,"ancestor_record_offset":113940,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":114884,"parent_record_offset":114860,"ancestor_record_offset":113940,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":114860,"parent_record_offset":114836,"ancestor_record_offset":113940,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":114808,"parent_record_offset":114776,"ancestor_record_offset":113940,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":114752,"parent_record_offset":114220,"ancestor_record_offset":113940,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":114776,"parent_record_offset":114752,"ancestor_record_offset":113940,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":114252,"parent_record_offset":114220,"ancestor_record_offset":113940,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":114836,"parent_record_offset":114776,"ancestor_record_offset":113940,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":114300,"parent_record_offset":114276,"ancestor_record_offset":113940,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":114276,"parent_record_offset":114252,"ancestor_record_offset":113940,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":114472,"parent_record_offset":114448,"ancestor_record_offset":113940,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":114952,"parent_record_offset":114928,"ancestor_record_offset":113940,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":114928,"parent_record_offset":114220,"ancestor_record_offset":113940,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":114184,"parent_record_offset":114156,"ancestor_record_offset":113940,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":114156,"parent_record_offset":114132,"ancestor_record_offset":113940,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":114220,"parent_record_offset":114156,"ancestor_record_offset":113940,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":194,"record_offset":116316,"parent_record_offset":116284,"ancestor_record_offset":113940,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":115760,"parent_record_offset":115496,"ancestor_record_offset":113940,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":118576,"parent_record_offset":118552,"ancestor_record_offset":113940,"inlinee":5188,"source_line":1740,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":194,"record_offset":119624,"parent_record_offset":119588,"ancestor_record_offset":113940,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":194,"record_offset":118512,"parent_record_offset":118488,"ancestor_record_offset":113940,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":194,"record_offset":117292,"parent_record_offset":117232,"ancestor_record_offset":113940,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":194,"record_offset":118872,"parent_record_offset":118848,"ancestor_record_offset":113940,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":194,"record_offset":119496,"parent_record_offset":119472,"ancestor_record_offset":113940,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":194,"record_offset":118848,"parent_record_offset":115496,"ancestor_record_offset":113940,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":194,"record_offset":119184,"parent_record_offset":119072,"ancestor_record_offset":113940,"inlinee":11420,"source_line":751,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":194,"record_offset":117468,"parent_record_offset":117444,"ancestor_record_offset":113940,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":194,"record_offset":118224,"parent_record_offset":118196,"ancestor_record_offset":113940,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":194,"record_offset":118464,"parent_record_offset":118440,"ancestor_record_offset":113940,"inlinee":6026,"source_line":416,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":194,"record_offset":116340,"parent_record_offset":116316,"ancestor_record_offset":113940,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":194,"record_offset":118440,"parent_record_offset":115496,"ancestor_record_offset":113940,"inlinee":6025,"source_line":653,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":120436,"parent_record_offset":120408,"ancestor_record_offset":113940,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":115348,"parent_record_offset":115324,"ancestor_record_offset":113940,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":194,"record_offset":114388,"parent_record_offset":114328,"ancestor_record_offset":113940,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":114408,"parent_record_offset":114388,"ancestor_record_offset":113940,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":118168,"parent_record_offset":117768,"ancestor_record_offset":113940,"inlinee":14124,"source_line":1613,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":114448,"parent_record_offset":114276,"ancestor_record_offset":113940,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":194,"record_offset":115848,"parent_record_offset":115816,"ancestor_record_offset":113940,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":194,"record_offset":115876,"parent_record_offset":115816,"ancestor_record_offset":113940,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":194,"record_offset":117492,"parent_record_offset":117468,"ancestor_record_offset":113940,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":115968,"parent_record_offset":115496,"ancestor_record_offset":113940,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":119740,"parent_record_offset":119716,"ancestor_record_offset":113940,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":116084,"parent_record_offset":116056,"ancestor_record_offset":113940,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":116132,"parent_record_offset":116108,"ancestor_record_offset":113940,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":116108,"parent_record_offset":116084,"ancestor_record_offset":113940,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":118100,"parent_record_offset":118076,"ancestor_record_offset":113940,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":118488,"parent_record_offset":118464,"ancestor_record_offset":113940,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":194,"record_offset":117344,"parent_record_offset":117316,"ancestor_record_offset":113940,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":115416,"parent_record_offset":115392,"ancestor_record_offset":113940,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":118744,"parent_record_offset":118716,"ancestor_record_offset":113940,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":118124,"parent_record_offset":118100,"ancestor_record_offset":113940,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":194,"record_offset":114328,"parent_record_offset":114300,"ancestor_record_offset":113940,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":194,"record_offset":119100,"parent_record_offset":119072,"ancestor_record_offset":113940,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":119148,"parent_record_offset":119124,"ancestor_record_offset":113940,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":114700,"parent_record_offset":114676,"ancestor_record_offset":113940,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":114360,"parent_record_offset":114328,"ancestor_record_offset":113940,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":114676,"parent_record_offset":114496,"ancestor_record_offset":113940,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":119124,"parent_record_offset":119100,"ancestor_record_offset":113940,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":194,"record_offset":119716,"parent_record_offset":119692,"ancestor_record_offset":113940,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":194,"record_offset":117316,"parent_record_offset":117292,"ancestor_record_offset":113940,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":194,"record_offset":119588,"parent_record_offset":119560,"ancestor_record_offset":113940,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":114976,"parent_record_offset":114952,"ancestor_record_offset":113940,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":115700,"parent_record_offset":115676,"ancestor_record_offset":113940,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":115000,"parent_record_offset":114976,"ancestor_record_offset":113940,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":115904,"parent_record_offset":115816,"ancestor_record_offset":113940,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":194,"record_offset":115792,"parent_record_offset":115496,"ancestor_record_offset":113940,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":194,"record_offset":115816,"parent_record_offset":115792,"ancestor_record_offset":113940,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":115928,"parent_record_offset":115904,"ancestor_record_offset":113940,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":115676,"parent_record_offset":115496,"ancestor_record_offset":113940,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":194,"record_offset":116056,"parent_record_offset":116024,"ancestor_record_offset":113940,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":194,"record_offset":116024,"parent_record_offset":115996,"ancestor_record_offset":113940,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::MonitorHandle::video_modes
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":194,"record_offset":115496,"parent_record_offset":115460,"ancestor_record_offset":113940,"inlinee":6022,"source_line":229,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::util::has_flag
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/util.rs
//     evidence={"module":194,"record_offset":118552,"parent_record_offset":115496,"ancestor_record_offset":113940,"inlinee":5213,"source_line":53,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_wait_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":194,"record_offset":115160,"parent_record_offset":115136,"ancestor_record_offset":113940,"inlinee":8425,"source_line":117,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$14::available_monitors::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":115184,"parent_record_offset":115160,"ancestor_record_offset":113940,"inlinee":11216,"source_line":1500,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::on_cursor_moved_absolute
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 1014
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x389E
// 首条位置: {"module":194,"inlinee":14494}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":37408,"parent_record_offset":36452,"ancestor_record_offset":36452,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":194,"record_offset":37464,"parent_record_offset":36452,"ancestor_record_offset":36452,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":36488,"parent_record_offset":36452,"ancestor_record_offset":36452,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":37728,"parent_record_offset":36452,"ancestor_record_offset":36452,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":36940,"parent_record_offset":36916,"ancestor_record_offset":36452,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":36988,"parent_record_offset":36964,"ancestor_record_offset":36452,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":36964,"parent_record_offset":36940,"ancestor_record_offset":36452,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":37568,"parent_record_offset":37524,"ancestor_record_offset":36452,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":36916,"parent_record_offset":36892,"ancestor_record_offset":36452,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":36892,"parent_record_offset":36852,"ancestor_record_offset":36452,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":37524,"parent_record_offset":37488,"ancestor_record_offset":36452,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":36852,"parent_record_offset":36828,"ancestor_record_offset":36452,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":194,"record_offset":37432,"parent_record_offset":37408,"ancestor_record_offset":36452,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":194,"record_offset":37488,"parent_record_offset":37464,"ancestor_record_offset":36452,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":37240,"parent_record_offset":37216,"ancestor_record_offset":36452,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":37216,"parent_record_offset":37192,"ancestor_record_offset":36452,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":37164,"parent_record_offset":37132,"ancestor_record_offset":36452,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":37108,"parent_record_offset":36568,"ancestor_record_offset":36452,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":37132,"parent_record_offset":37108,"ancestor_record_offset":36452,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":36600,"parent_record_offset":36568,"ancestor_record_offset":36452,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":37192,"parent_record_offset":37132,"ancestor_record_offset":36452,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":36648,"parent_record_offset":36624,"ancestor_record_offset":36452,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":36624,"parent_record_offset":36600,"ancestor_record_offset":36452,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":36828,"parent_record_offset":36804,"ancestor_record_offset":36452,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":37308,"parent_record_offset":37284,"ancestor_record_offset":36452,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":37284,"parent_record_offset":36568,"ancestor_record_offset":36452,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":36512,"parent_record_offset":36488,"ancestor_record_offset":36452,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":36568,"parent_record_offset":36512,"ancestor_record_offset":36452,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":194,"record_offset":36740,"parent_record_offset":36676,"ancestor_record_offset":36452,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":36764,"parent_record_offset":36740,"ancestor_record_offset":36452,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":36804,"parent_record_offset":36624,"ancestor_record_offset":36452,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":36540,"parent_record_offset":36512,"ancestor_record_offset":36452,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":37688,"parent_record_offset":37524,"ancestor_record_offset":36452,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":194,"record_offset":36676,"parent_record_offset":36648,"ancestor_record_offset":36452,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":37056,"parent_record_offset":37032,"ancestor_record_offset":36452,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":36712,"parent_record_offset":36676,"ancestor_record_offset":36452,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":37032,"parent_record_offset":36852,"ancestor_record_offset":36452,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":37332,"parent_record_offset":37308,"ancestor_record_offset":36452,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":37356,"parent_record_offset":37332,"ancestor_record_offset":36452,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::shut_down_everything
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1020
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x388E
// 首条位置: {"module":194,"inlinee":14478}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":10984,"parent_record_offset":6480,"ancestor_record_offset":6480,"inlinee":8096,"source_line":2015,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::remove_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":194,"record_offset":14228,"parent_record_offset":6480,"ancestor_record_offset":6480,"inlinee":9405,"source_line":1793,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":15060,"parent_record_offset":6480,"ancestor_record_offset":6480,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":15672,"parent_record_offset":6480,"ancestor_record_offset":6480,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":13400,"parent_record_offset":6480,"ancestor_record_offset":6480,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":194,"record_offset":15728,"parent_record_offset":6480,"ancestor_record_offset":6480,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":194,"record_offset":15092,"parent_record_offset":6480,"ancestor_record_offset":6480,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":14976,"parent_record_offset":6480,"ancestor_record_offset":6480,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":194,"record_offset":11800,"parent_record_offset":6480,"ancestor_record_offset":6480,"inlinee":10610,"source_line":2217,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":194,"record_offset":12680,"parent_record_offset":6480,"ancestor_record_offset":6480,"inlinee":9318,"source_line":347,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] winit::window::Window::set_fullscreen
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":6600,"parent_record_offset":6480,"ancestor_record_offset":6480,"inlinee":14479,"source_line":1085,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":6480,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":7884,"parent_record_offset":7860,"ancestor_record_offset":6480,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":9008,"parent_record_offset":8984,"ancestor_record_offset":6480,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":7836,"parent_record_offset":7812,"ancestor_record_offset":6480,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":194,"record_offset":8984,"parent_record_offset":8960,"ancestor_record_offset":6480,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":7744,"parent_record_offset":7688,"ancestor_record_offset":6480,"inlinee":6152,"source_line":1742,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":7780,"parent_record_offset":7744,"ancestor_record_offset":6480,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":7812,"parent_record_offset":7780,"ancestor_record_offset":6480,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":194,"record_offset":8960,"parent_record_offset":8936,"ancestor_record_offset":6480,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":6480,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":11964,"parent_record_offset":11936,"ancestor_record_offset":6480,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":12372,"parent_record_offset":12344,"ancestor_record_offset":6480,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":6480,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":11492,"parent_record_offset":11468,"ancestor_record_offset":6480,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":6480,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":12024,"parent_record_offset":11992,"ancestor_record_offset":6480,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":11992,"parent_record_offset":11964,"ancestor_record_offset":6480,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":194,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":6480,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_inner_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":7060,"parent_record_offset":6936,"ancestor_record_offset":6480,"inlinee":6797,"source_line":281,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":6968,"parent_record_offset":6936,"ancestor_record_offset":6480,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":6936,"parent_record_offset":6756,"ancestor_record_offset":6480,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":7092,"parent_record_offset":6756,"ancestor_record_offset":6480,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":194,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":6480,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":6480,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":14580,"parent_record_offset":14540,"ancestor_record_offset":6480,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":6480,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":11396,"parent_record_offset":11356,"ancestor_record_offset":6480,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":11908,"parent_record_offset":11848,"ancestor_record_offset":6480,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":11936,"parent_record_offset":11908,"ancestor_record_offset":6480,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":11356,"parent_record_offset":11332,"ancestor_record_offset":6480,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":14540,"parent_record_offset":14516,"ancestor_record_offset":6480,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":11744,"parent_record_offset":11720,"ancestor_record_offset":6480,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":11720,"parent_record_offset":11696,"ancestor_record_offset":6480,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":194,"record_offset":11668,"parent_record_offset":11636,"ancestor_record_offset":6480,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":11612,"parent_record_offset":11076,"ancestor_record_offset":6480,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":11636,"parent_record_offset":11612,"ancestor_record_offset":6480,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":14796,"parent_record_offset":14228,"ancestor_record_offset":6480,"inlinee":9406,"source_line":258,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":11104,"parent_record_offset":11076,"ancestor_record_offset":6480,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":14288,"parent_record_offset":14228,"ancestor_record_offset":6480,"inlinee":9260,"source_line":353,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":194,"record_offset":11696,"parent_record_offset":11636,"ancestor_record_offset":6480,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":6480,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":11128,"parent_record_offset":11104,"ancestor_record_offset":6480,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":14312,"parent_record_offset":14288,"ancestor_record_offset":6480,"inlinee":9261,"source_line":405,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":11332,"parent_record_offset":11308,"ancestor_record_offset":6480,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get_mut::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":194,"record_offset":14516,"parent_record_offset":14492,"ancestor_record_offset":6480,"inlinee":9266,"source_line":407,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":11036,"parent_record_offset":11008,"ancestor_record_offset":6480,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":11008,"parent_record_offset":10984,"ancestor_record_offset":6480,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":194,"record_offset":11076,"parent_record_offset":11008,"ancestor_record_offset":6480,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::OwningPtr::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":194,"record_offset":14888,"parent_record_offset":14228,"ancestor_record_offset":6480,"inlinee":9407,"source_line":455,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":194,"record_offset":13768,"parent_record_offset":13736,"ancestor_record_offset":6480,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impl$0::clone_to_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":194,"record_offset":7928,"parent_record_offset":7744,"ancestor_record_offset":6480,"inlinee":6154,"source_line":275,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::uninit::impl$1::clone_one
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone\uninit.rs
//     evidence={"module":194,"record_offset":7952,"parent_record_offset":7928,"ancestor_record_offset":6480,"inlinee":6836,"source_line":56,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":8164,"parent_record_offset":8128,"ancestor_record_offset":6480,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":194,"record_offset":12716,"parent_record_offset":12680,"ancestor_record_offset":6480,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":194,"record_offset":7976,"parent_record_offset":7952,"ancestor_record_offset":6480,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":194,"record_offset":9932,"parent_record_offset":6756,"ancestor_record_offset":6480,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":194,"record_offset":13792,"parent_record_offset":13768,"ancestor_record_offset":6480,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":194,"record_offset":11244,"parent_record_offset":11180,"ancestor_record_offset":6480,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":11268,"parent_record_offset":11244,"ancestor_record_offset":6480,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":11308,"parent_record_offset":11128,"ancestor_record_offset":6480,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":8088,"parent_record_offset":6756,"ancestor_record_offset":6480,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":14260,"parent_record_offset":14228,"ancestor_record_offset":6480,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":7620,"parent_record_offset":6756,"ancestor_record_offset":6480,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":194,"record_offset":15148,"parent_record_offset":15116,"ancestor_record_offset":6480,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":194,"record_offset":15176,"parent_record_offset":15116,"ancestor_record_offset":6480,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":8256,"parent_record_offset":6756,"ancestor_record_offset":6480,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":14912,"parent_record_offset":14888,"ancestor_record_offset":6480,"inlinee":4834,"source_line":1354,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":13536,"parent_record_offset":13512,"ancestor_record_offset":6480,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":13584,"parent_record_offset":13560,"ancestor_record_offset":6480,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":13560,"parent_record_offset":13536,"ancestor_record_offset":6480,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":194,"record_offset":7000,"parent_record_offset":6968,"ancestor_record_offset":6480,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":14936,"parent_record_offset":14912,"ancestor_record_offset":6480,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":194,"record_offset":7580,"parent_record_offset":6756,"ancestor_record_offset":6480,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":194,"record_offset":11180,"parent_record_offset":11152,"ancestor_record_offset":6480,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":14744,"parent_record_offset":14720,"ancestor_record_offset":6480,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":11560,"parent_record_offset":11536,"ancestor_record_offset":6480,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":11216,"parent_record_offset":11180,"ancestor_record_offset":6480,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":11536,"parent_record_offset":11356,"ancestor_record_offset":6480,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":14720,"parent_record_offset":14540,"ancestor_record_offset":6480,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":7500,"parent_record_offset":7472,"ancestor_record_offset":6480,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":8516,"parent_record_offset":8328,"ancestor_record_offset":6480,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":7188,"parent_record_offset":7156,"ancestor_record_offset":6480,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":8608,"parent_record_offset":8576,"ancestor_record_offset":6480,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":7404,"parent_record_offset":7372,"ancestor_record_offset":6480,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":7028,"parent_record_offset":6968,"ancestor_record_offset":6480,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":7212,"parent_record_offset":7188,"ancestor_record_offset":6480,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":7428,"parent_record_offset":7404,"ancestor_record_offset":6480,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":8540,"parent_record_offset":8516,"ancestor_record_offset":6480,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":8796,"parent_record_offset":8736,"ancestor_record_offset":6480,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":8632,"parent_record_offset":8608,"ancestor_record_offset":6480,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":8828,"parent_record_offset":8700,"ancestor_record_offset":6480,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":15204,"parent_record_offset":15116,"ancestor_record_offset":6480,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":194,"record_offset":15116,"parent_record_offset":15092,"ancestor_record_offset":6480,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":194,"record_offset":15228,"parent_record_offset":15204,"ancestor_record_offset":6480,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] serde::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/ser/impls.rs
//     evidence={"module":194,"record_offset":12240,"parent_record_offset":12156,"ancestor_record_offset":6480,"inlinee":8878,"source_line":499,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":194,"record_offset":12184,"parent_record_offset":12156,"ancestor_record_offset":6480,"inlinee":8160,"source_line":25,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":194,"record_offset":12208,"parent_record_offset":12184,"ancestor_record_offset":6480,"inlinee":8161,"source_line":49,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":194,"record_offset":11848,"parent_record_offset":11800,"ancestor_record_offset":6480,"inlinee":8157,"source_line":2185,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":194,"record_offset":12156,"parent_record_offset":11848,"ancestor_record_offset":6480,"inlinee":8158,"source_line":2149,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":194,"record_offset":12828,"parent_record_offset":12800,"ancestor_record_offset":6480,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":194,"record_offset":12884,"parent_record_offset":12856,"ancestor_record_offset":6480,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":194,"record_offset":12856,"parent_record_offset":12828,"ancestor_record_offset":6480,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":194,"record_offset":13512,"parent_record_offset":13484,"ancestor_record_offset":6480,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":194,"record_offset":13484,"parent_record_offset":13456,"ancestor_record_offset":6480,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":194,"record_offset":7372,"parent_record_offset":7344,"ancestor_record_offset":6480,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":194,"record_offset":7344,"parent_record_offset":7316,"ancestor_record_offset":6480,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":194,"record_offset":12800,"parent_record_offset":12772,"ancestor_record_offset":6480,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":194,"record_offset":12772,"parent_record_offset":12744,"ancestor_record_offset":6480,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":194,"record_offset":12744,"parent_record_offset":12716,"ancestor_record_offset":6480,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":8328,"parent_record_offset":8288,"ancestor_record_offset":6480,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":7472,"parent_record_offset":7276,"ancestor_record_offset":6480,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":7276,"parent_record_offset":7248,"ancestor_record_offset":6480,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":7120,"parent_record_offset":6756,"ancestor_record_offset":6480,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":7248,"parent_record_offset":7120,"ancestor_record_offset":6480,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":8288,"parent_record_offset":8256,"ancestor_record_offset":6480,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":194,"record_offset":7156,"parent_record_offset":7120,"ancestor_record_offset":6480,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":194,"record_offset":8576,"parent_record_offset":8288,"ancestor_record_offset":6480,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":194,"record_offset":7316,"parent_record_offset":7276,"ancestor_record_offset":6480,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$2::clone
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/mod.rs
//     evidence={"module":194,"record_offset":7656,"parent_record_offset":7620,"ancestor_record_offset":6480,"inlinee":6875,"source_line":29,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::impl$5::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/mod.rs
//     evidence={"module":194,"record_offset":8128,"parent_record_offset":8088,"ancestor_record_offset":6480,"inlinee":14484,"source_line":29,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$10::clone
//     owner=git_dependency_definition; count=3
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":194,"record_offset":7716,"parent_record_offset":7688,"ancestor_record_offset":6480,"inlinee":6834,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$13::eq
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":194,"record_offset":9060,"parent_record_offset":6756,"ancestor_record_offset":6480,"inlinee":14486,"source_line":87,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::monitor::impl$8::clone
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/monitor.rs
//     evidence={"module":194,"record_offset":7688,"parent_record_offset":7656,"ancestor_record_offset":6480,"inlinee":6832,"source_line":28,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::maybe_queue_on_main
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":194,"record_offset":6652,"parent_record_offset":6600,"ancestor_record_offset":6480,"inlinee":6002,"source_line":112,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::platform_impl::platform::window::Window::set_fullscreen
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/platform_impl/windows/window.rs
//     evidence={"module":194,"record_offset":6756,"parent_record_offset":6704,"ancestor_record_offset":6480,"inlinee":14482,"source_line":686,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] winit::window::impl$12::set_fullscreen::closure$0
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"winit","version":"0.29.15","source":"git+https://github.com/PounceLight/winit.git?rev=d03de0a0a881be9422c360083151b0e7a8a2582e#d03de0a0a881be9422c360083151b0e7a8a2582e"}]
//     definition_source=/home/h3/.cargo/git/checkouts/winit-1ef96b18fbe2a7d1/d03de0a/src/window.rs
//     evidence={"module":194,"record_offset":6704,"parent_record_offset":6652,"ancestor_record_offset":6480,"inlinee":14481,"source_line":1087,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::shut_down_everything::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1034
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3DC3
// 首条位置: {"module":245,"inlinee":15811}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::CursorRequestClient::next_request_id
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1078
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x38A0
// 首条位置: {"module":194,"inlinee":14496}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::CursorRequestClient::on_confirmation
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1083
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x38A4
// 首条位置: {"module":194,"inlinee":14500}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":50100,"parent_record_offset":50072,"ancestor_record_offset":50072,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::CursorRequestClient::is_fully_confirmed
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1087
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x38B1
// 首条位置: {"module":194,"inlinee":14513}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":65772,"parent_record_offset":65748,"ancestor_record_offset":65748,"inlinee":11109,"source_line":1425,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":194,"record_offset":65820,"parent_record_offset":65796,"ancestor_record_offset":65748,"inlinee":14516,"source_line":1820,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::event_loop::impl$5::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/event_loop.rs
//     evidence={"module":194,"record_offset":65796,"parent_record_offset":65772,"ancestor_record_offset":65748,"inlinee":14515,"source_line":83,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::impl$3::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 1095
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":194,"record_offset":121652,"rva":5124976,"va":5373834096}
// 编译实例 RVAs: 0x4e3370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140327ff0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-1815432","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5373834096,"raw_hex":"e9784ce4ff","size":5,"soft_fail":false,"target_va":5372018672,"va":5373834099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":194,"record_offset":121772,"parent_record_offset":121652,"ancestor_record_offset":121652,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::game_loop::gather_pending_video_setting_changes
// 记录形式: compiled_procedure; instances=1
// 源码行: 1100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":194,"record_offset":121804,"rva":5124992,"va":5373834112}
// 编译实例 RVAs: 0x4e3380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: game_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t979662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373834112,"raw_hex":"e8cef20e00","size":5,"soft_fail":false,"target_va":5374814320,"va":5373834653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37708287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373834112,"raw_hex":"e8ff613f02","size":5,"soft_fail":false,"target_va":5411542880,"va":5373834588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37704141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373834112,"raw_hex":"e8cd513f02","size":5,"soft_fail":false,"target_va":5411538608,"va":5373834462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37704271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373834112,"raw_hex":"e84f523f02","size":5,"soft_fail":false,"target_va":5411538768,"va":5373834492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37704633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373834112,"raw_hex":"e8b9533f02","size":5,"soft_fail":false,"target_va":5411538960,"va":5373834322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":194,"record_offset":122664,"parent_record_offset":121804,"ancestor_record_offset":121804,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":194,"record_offset":121968,"parent_record_offset":121804,"ancestor_record_offset":121804,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":123044,"parent_record_offset":121804,"ancestor_record_offset":121804,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":194,"record_offset":123908,"parent_record_offset":121804,"ancestor_record_offset":121804,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":123444,"parent_record_offset":121804,"ancestor_record_offset":121804,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::app_config::impl$24::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":194,"record_offset":123952,"parent_record_offset":121804,"ancestor_record_offset":121804,"inlinee":14518,"source_line":739,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":122308,"parent_record_offset":122284,"ancestor_record_offset":121804,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":194,"record_offset":122284,"parent_record_offset":122056,"ancestor_record_offset":121804,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":194,"record_offset":122032,"parent_record_offset":121992,"ancestor_record_offset":121804,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":194,"record_offset":122056,"parent_record_offset":122032,"ancestor_record_offset":121804,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":194,"record_offset":122696,"parent_record_offset":122664,"ancestor_record_offset":121804,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":194,"record_offset":121992,"parent_record_offset":121968,"ancestor_record_offset":121804,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":194,"record_offset":122764,"parent_record_offset":122732,"ancestor_record_offset":121804,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":194,"record_offset":122732,"parent_record_offset":122696,"ancestor_record_offset":121804,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":194,"record_offset":122860,"parent_record_offset":122836,"ancestor_record_offset":121804,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":122240,"parent_record_offset":122212,"ancestor_record_offset":121804,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":122100,"parent_record_offset":122056,"ancestor_record_offset":121804,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":122796,"parent_record_offset":122764,"ancestor_record_offset":121804,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":122836,"parent_record_offset":122732,"ancestor_record_offset":121804,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":122556,"parent_record_offset":122532,"ancestor_record_offset":121804,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":194,"record_offset":122984,"parent_record_offset":122960,"ancestor_record_offset":121804,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":194,"record_offset":122156,"parent_record_offset":122056,"ancestor_record_offset":121804,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":194,"record_offset":122508,"parent_record_offset":122056,"ancestor_record_offset":121804,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":122212,"parent_record_offset":122184,"ancestor_record_offset":121804,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":194,"record_offset":122184,"parent_record_offset":122156,"ancestor_record_offset":121804,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":194,"record_offset":122532,"parent_record_offset":122508,"ancestor_record_offset":121804,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":194,"record_offset":122960,"parent_record_offset":122936,"ancestor_record_offset":121804,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":194,"record_offset":122912,"parent_record_offset":122860,"ancestor_record_offset":121804,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":194,"record_offset":122936,"parent_record_offset":122912,"ancestor_record_offset":121804,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":123828,"parent_record_offset":123800,"ancestor_record_offset":121804,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":123376,"parent_record_offset":123204,"ancestor_record_offset":121804,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":123512,"parent_record_offset":123480,"ancestor_record_offset":121804,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":123144,"parent_record_offset":123112,"ancestor_record_offset":121804,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":123308,"parent_record_offset":123280,"ancestor_record_offset":121804,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":123536,"parent_record_offset":123512,"ancestor_record_offset":121804,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":123332,"parent_record_offset":123308,"ancestor_record_offset":121804,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":123400,"parent_record_offset":123376,"ancestor_record_offset":121804,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":194,"record_offset":123168,"parent_record_offset":123144,"ancestor_record_offset":121804,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":194,"record_offset":123280,"parent_record_offset":123256,"ancestor_record_offset":121804,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":194,"record_offset":123256,"parent_record_offset":123232,"ancestor_record_offset":121804,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":123204,"parent_record_offset":123076,"ancestor_record_offset":121804,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":123800,"parent_record_offset":123600,"ancestor_record_offset":121804,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":194,"record_offset":123600,"parent_record_offset":123572,"ancestor_record_offset":121804,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":123572,"parent_record_offset":123444,"ancestor_record_offset":121804,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":194,"record_offset":123076,"parent_record_offset":123044,"ancestor_record_offset":121804,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":194,"record_offset":123480,"parent_record_offset":123444,"ancestor_record_offset":121804,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":194,"record_offset":123112,"parent_record_offset":123076,"ancestor_record_offset":121804,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":194,"record_offset":123232,"parent_record_offset":123204,"ancestor_record_offset":121804,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00194.jsonl.gz"}
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
// 完整结构及原始类型记录: game_loop.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::game_loop::CursorRequestClient
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::game_loop::GameLoop
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 11; 过程引用数: 4
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::game_loop::GameLoopPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$15::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::run_game::ReflectionListingPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$15::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::run_game::ReflectionListingPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::ready<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "bevy_app::plugin::Plugin::ready", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::finish<tiny_glade::game_loop::GameLoopPlugin>", "constructor": "bevy_app::plugin::Plugin::finish", "arguments": ["tiny_glade::game_loop::GameLoopPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: game_loop.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: game_loop.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::CursorRequestClient::is_fully_confirmed
// 信息 ID: a64334b024b33edf0fdab47d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=65748; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::CursorRequestClient::next_request_id
// 信息 ID: f75dcc9e555eef6f6385786e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=37764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::CursorRequestClient::on_confirmation
// 信息 ID: f1630e4283d09bd13cfefe8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=50072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::apply_setting_changes
// 信息 ID: 52660d79145954d21bdf453e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=17620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::extract_available_monitors_info
// 信息 ID: 497bc53ec0f779b03655d538
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x4d9628 -> 0x4e2470 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::main_loop
// 信息 ID: 6c38215cf6900d8bdba55fc0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::run @ 0x4d8284 -> 0x4d9580 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::maintain_resolution_scale
// 信息 ID: 39684016fa3937a02808dd6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=80636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::new
// 信息 ID: 30ba8335bd41b389ed19acc2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x5a614e -> 0x4d8100 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x5a5f88 -> 0x4d8100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::process_app_event
// 信息 ID: 605be133bbf668df38b0b7c4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=42400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::receive_app_events
// 信息 ID: ef7992e1bc1a600f8b8a4bec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=35448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::run
// 信息 ID: c44389a102c74fa485bc3d1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x5a617d -> 0x4d8250 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x5a5fa9 -> 0x4d8250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::send_window_events
// 信息 ID: 07e1a1d4eb6569730ca7045d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=35676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::GameLoop::simulate_and_render_frame
// 信息 ID: c5c80a106dd0147b7de4b6d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=76896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::gather_pending_video_setting_changes
// 信息 ID: 5c9ccc7bb76e9506ffa55ccd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f428ef437e0508207e23ffd0: void (*)(bevy_ecs::event::reader::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>),void (*)(bevy_ecs::event::reader::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)> > @ 0x366855 -> 0x4e3380 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>),void (*)(bevy_ecs::event::reader::EventReader<tiny_glade::app_config::ApplyVideoSettingsCmd>)> > > @ 0x448b73 -> 0x4e3380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::main_loop::closure$0
// 信息 ID: 9a0ccd5640fd6abfb910404d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::game_loop::impl$0::main_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=25200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::main_loop::closure$1
// 信息 ID: 008243ee964eba259799b66e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::game_loop::impl$0::main_loop", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=27528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::process_app_event::closure$0
// 信息 ID: a51468a7c2baa9e79893cef2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::game_loop::impl$0::process_app_event", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=68548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$1
// 信息 ID: 62daff13e6422a437aa6d378
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::game_loop::impl$0::simulate_and_render_frame", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=110560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$0::simulate_and_render_frame::closure$2
// 信息 ID: 2ca335837ddd1f41f1276040
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::game_loop::impl$0::simulate_and_render_frame", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=111520; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::impl$3::build
// 信息 ID: cec4f928d0e163fa45deac9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::on_cursor_moved_absolute
// 信息 ID: d8cd04098a81453ac43927b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=36452; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=65860; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::shut_down_everything
// 信息 ID: eefff5e0692a5cdbef3cd962
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::run; module=194, inline_record=6480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::game_loop::shut_down_everything::closure$0
// 信息 ID: db064538c47c7f3c4fca4087
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::game_loop::shut_down_everything", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::game_loop::shut_down_everything::closure_env$0 <- rhapsody::raw::device::GraphicsDevice::with_global_allocator<tiny_glade::game_loop::shut_down_everything::closure_env$0,tuple$<u64,u64> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::with_global_allocator<tiny_glade::game_loop::shut_down_everything::closure_env$0,tuple$<u64,u64> >; module=245, inline_record=29312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
