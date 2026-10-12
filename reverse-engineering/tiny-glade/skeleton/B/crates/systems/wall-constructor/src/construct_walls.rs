// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/construct_walls.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a82d886131f37827e661b887b8985f32124fb8735dbbd52100f5b182f797dae1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: construct_walls.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 394, 1415, 1417, 1432
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
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
// 函数符号: system_wall_constructor::construct_walls::construct_walls
// 记录形式: compiled_procedure; instances=1
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":394,"record_offset":2700,"rva":7698432,"va":5376407552}
// 编译实例 RVAs: 0x757800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=3
//     target_aliases=12 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7142845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8430293ff","size":5,"soft_fail":false,"target_va":5369267664,"va":5376410504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014b320
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6353143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8090f9fff","size":5,"soft_fail":false,"target_va":5370065696,"va":5376418834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ba9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4840697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e80723b6ff","size":5,"soft_fail":false,"target_va":5371570656,"va":5376411348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1733375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8018de5ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5376409706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1606471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8b97ce7ff","size":5,"soft_fail":false,"target_va":5374813424,"va":5376419890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1329100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e834b8ebff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376410583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1332881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e86fa9ebff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376414380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833230
//     owner=unresolved_namespace; count=4
//     target_aliases=9 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t897087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e83fb00d00","size":5,"soft_fail":false,"target_va":5377307184,"va":5376410092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140858790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1046466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8c2f70f00","size":5,"soft_fail":false,"target_va":5377460112,"va":5376413641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bc510
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1454258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8b2301600","size":5,"soft_fail":false,"target_va":5377869072,"va":5376414809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5670
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1495872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e840d31600","size":5,"soft_fail":false,"target_va":5377906288,"va":5376410411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140958d60
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2092455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8a7ed1f00","size":5,"soft_fail":false,"target_va":5378510176,"va":5376417716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b26a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3988697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8d9dc3c00","size":5,"soft_fail":false,"target_va":5380401776,"va":5376413074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c904a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5469101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8ad735300","size":5,"soft_fail":false,"target_va":5381883040,"va":5376413934,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140eec970
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7942083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8c32f7900","size":5,"soft_fail":false,"target_va":5384358256,"va":5376416168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f9920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11145777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e83112aa00","size":5,"soft_fail":false,"target_va":5387557152,"va":5376411370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141201e70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11175836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e89c87aa00","size":5,"soft_fail":false,"target_va":5387591280,"va":5376415439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412036d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11188975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8efbaaa00","size":5,"soft_fail":false,"target_va":5387597520,"va":5376408540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141209c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11207930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8fa04ab00","size":5,"soft_fail":false,"target_va":5387623488,"va":5376415553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412259c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11322751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e87fc5ac00","size":5,"soft_fail":false,"target_va":5387737536,"va":5376414780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a9a00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34929938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e812fd1402","size":5,"soft_fail":false,"target_va":5411346944,"va":5376417001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428b80c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34989419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e86be51502","size":5,"soft_fail":false,"target_va":5411406016,"va":5376416592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35431529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e869a41c02","size":5,"soft_fail":false,"target_va":5411847152,"va":5376415618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: construct_walls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35436069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e825b61c02","size":5,"soft_fail":false,"target_va":5411847168,"va":5376411094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35123426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8e2f01702","size":5,"soft_fail":false,"target_va":5411540992,"va":5376417561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-2053627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e805aae0ff","size":5,"soft_fail":false,"target_va":5374355568,"va":5376409190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34930865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8b1001502","size":5,"soft_fail":false,"target_va":5411339744,"va":5376408874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35123307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e86bf01702","size":5,"soft_fail":false,"target_va":5411540963,"va":5376417651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11230163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8d35bab00","size":5,"soft_fail":false,"target_va":5387646288,"va":5376416120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35124360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e888f41702","size":5,"soft_fail":false,"target_va":5411541584,"va":5376417219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35125077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e855f71702","size":5,"soft_fail":false,"target_va":5411541776,"va":5376416694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35065226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e88a0d1702","size":5,"soft_fail":false,"target_va":5411482433,"va":5376417202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35124365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e88df41702","size":5,"soft_fail":false,"target_va":5411541904,"va":5376417534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::walls::wall_holes::WallHoleStorage> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-13392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8b0cbffff","size":5,"soft_fail":false,"target_va":5376406144,"va":5376419531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35125530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e81af91702","size":5,"soft_fail":false,"target_va":5411542880,"va":5376417345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e803faffff","size":5,"soft_fail":false,"target_va":5376406288,"va":5376407816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35135462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8e61f1802","size":5,"soft_fail":false,"target_va":5411543360,"va":5376407893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::InstancedWall::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1044271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e82fef0f00","size":5,"soft_fail":false,"target_va":5377460224,"va":5376415948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::InstancedWall::update
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1045098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e86af20f00","size":5,"soft_fail":false,"target_va":5377460272,"va":5376415169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::gable_sides
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1618050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e882b01800","size":5,"soft_fail":false,"target_va":5378028512,"va":5376410457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1617300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e894ad1800","size":5,"soft_fail":false,"target_va":5378028416,"va":5376411111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1618106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8bab01800","size":5,"soft_fail":false,"target_va":5378028464,"va":5376410353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1617736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e848af1800","size":5,"soft_fail":false,"target_va":5378028112,"va":5376410371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4005468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e85c1e3d00","size":5,"soft_fail":false,"target_va":5380416976,"va":5376411503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4005580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8cc1e3d00","size":5,"soft_fail":false,"target_va":5380417056,"va":5376411471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4003130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e83a153d00","size":5,"soft_fail":false,"target_va":5380417024,"va":5376413889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3383833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e819a23300","size":5,"soft_fail":false,"target_va":5379792624,"va":5376408786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3383519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8dfa03300","size":5,"soft_fail":false,"target_va":5379793584,"va":5376410060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::WallEntities::wall_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3377784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8788a3300","size":5,"soft_fail":false,"target_va":5379792384,"va":5376414595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::as_finalized
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8578062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e80ee48200","size":5,"soft_fail":false,"target_va":5384990384,"va":5376412317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::as_wip_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8577726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8bee28200","size":5,"soft_fail":false,"target_va":5384990464,"va":5376412733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::clone_and_reopen
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8577917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e87de38200","size":5,"soft_fail":false,"target_va":5384990544,"va":5376412622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::finalize
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8582349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8cdf48200","size":5,"soft_fail":false,"target_va":5384991488,"va":5376409134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::new_finalized
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8578198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e896e48200","size":5,"soft_fail":false,"target_va":5384990288,"va":5376412085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::default_tris
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1632857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e859ea1800","size":5,"soft_fail":false,"target_va":5378048224,"va":5376415362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::recompute
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1641158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8c60a1900","size":5,"soft_fail":false,"target_va":5378055888,"va":5376414725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11169732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8c46faa00","size":5,"soft_fail":false,"target_va":5387581248,"va":5376411511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4977221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e845f24b00","size":5,"soft_fail":false,"target_va":5381385536,"va":5376408310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4982410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e88a064c00","size":5,"soft_fail":false,"target_va":5381390640,"va":5376408225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4969634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8a2d44b00","size":5,"soft_fail":false,"target_va":5381386480,"va":5376416841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4976913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e811f14b00","size":5,"soft_fail":false,"target_va":5381384704,"va":5376407786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35223802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8fa781902","size":5,"soft_fail":false,"target_va":5411640768,"va":5376416961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35120050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8b2e31702","size":5,"soft_fail":false,"target_va":5411537104,"va":5376417049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::half_timber::half_timbering
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11272188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8fcffab00","size":5,"soft_fail":false,"target_va":5387685424,"va":5376413231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::half_timber::roof_outline
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11267436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e86cedab00","size":5,"soft_fail":false,"target_va":5387680736,"va":5376413295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11316565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e855adac00","size":5,"soft_fail":false,"target_va":5387730192,"va":5376413622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::wall_corners::add_wall_corners
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11257928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e848c8ab00","size":5,"soft_fail":false,"target_va":5387670848,"va":5376412915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::wall_skirts::add_skirts
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11309691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e87b92ac00","size":5,"soft_fail":false,"target_va":5387723248,"va":5376413552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::wall_constructor::WallConstructor::from_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t66812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8fc040100","size":5,"soft_fail":false,"target_va":5376481120,"va":5376414303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5508921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376407552,"raw_hex":"e8390f5400","size":5,"soft_fail":false,"target_va":5381920464,"va":5376411538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5376407552,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5376415649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":29292,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":10753,"source_line":3172,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":11148,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":8400,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":14080,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":9948,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":6960,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":394,"record_offset":10004,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":394,"record_offset":7980,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":394,"record_offset":25784,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":394,"record_offset":5428,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":394,"record_offset":11176,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":394,"record_offset":11500,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":394,"record_offset":23644,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":13654,"source_line":1576,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":394,"record_offset":20664,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":394,"record_offset":14920,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":394,"record_offset":6196,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":394,"record_offset":6140,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":394,"record_offset":20784,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":394,"record_offset":20692,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":394,"record_offset":14692,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":394,"record_offset":15196,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":394,"record_offset":21100,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":394,"record_offset":29180,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":394,"record_offset":29208,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":394,"record_offset":9272,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":24453,"source_line":2972,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":394,"record_offset":14752,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":394,"record_offset":26608,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":394,"record_offset":26884,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":24520,"source_line":117,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":394,"record_offset":19960,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":24514,"source_line":273,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":394,"record_offset":9244,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":24415,"source_line":118,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_trigger_volume::impl$6::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_trigger_volume.rs
//     evidence={"module":394,"record_offset":22904,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":24518,"source_line":116,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":394,"record_offset":4444,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":394,"record_offset":3020,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":394,"record_offset":3468,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_insert_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":394,"record_offset":17440,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":7951,"source_line":2822,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":394,"record_offset":15288,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":7716,"source_line":872,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":394,"record_offset":10120,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":394,"record_offset":27488,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$45::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":394,"record_offset":8040,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":19936,"source_line":2352,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$50::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":394,"record_offset":8584,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":14698,"source_line":2428,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_walls::construct_walls::closure$6
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":394,"record_offset":22932,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":24519,"source_line":369,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_walls::create_wall
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":394,"record_offset":27392,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":24522,"source_line":424,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$5::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":394,"record_offset":9920,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":14282,"source_line":50,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_to_curve2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":394,"record_offset":14952,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":24511,"source_line":1147,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":394,"record_offset":20720,"parent_record_offset":2700,"ancestor_record_offset":2700,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":394,"record_offset":13568,"parent_record_offset":13544,"ancestor_record_offset":2700,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":394,"record_offset":13592,"parent_record_offset":13568,"ancestor_record_offset":2700,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":394,"record_offset":20592,"parent_record_offset":20568,"ancestor_record_offset":2700,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":394,"record_offset":26952,"parent_record_offset":26924,"ancestor_record_offset":2700,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":394,"record_offset":13544,"parent_record_offset":13364,"ancestor_record_offset":2700,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":394,"record_offset":20568,"parent_record_offset":20480,"ancestor_record_offset":2700,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":394,"record_offset":26924,"parent_record_offset":26884,"ancestor_record_offset":2700,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":394,"record_offset":7080,"parent_record_offset":7048,"ancestor_record_offset":2700,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":394,"record_offset":7908,"parent_record_offset":7884,"ancestor_record_offset":2700,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":394,"record_offset":13296,"parent_record_offset":13264,"ancestor_record_offset":2700,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":394,"record_offset":20508,"parent_record_offset":20480,"ancestor_record_offset":2700,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":394,"record_offset":20480,"parent_record_offset":20456,"ancestor_record_offset":2700,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":394,"record_offset":7128,"parent_record_offset":7104,"ancestor_record_offset":2700,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":394,"record_offset":7104,"parent_record_offset":7080,"ancestor_record_offset":2700,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":394,"record_offset":7932,"parent_record_offset":7908,"ancestor_record_offset":2700,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":394,"record_offset":13364,"parent_record_offset":13328,"ancestor_record_offset":2700,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":394,"record_offset":13328,"parent_record_offset":13296,"ancestor_record_offset":2700,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":394,"record_offset":20456,"parent_record_offset":20432,"ancestor_record_offset":2700,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":394,"record_offset":13404,"parent_record_offset":13364,"ancestor_record_offset":2700,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":394,"record_offset":14212,"parent_record_offset":14180,"ancestor_record_offset":2700,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":394,"record_offset":14180,"parent_record_offset":14112,"ancestor_record_offset":2700,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::downgrade
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":394,"record_offset":27108,"parent_record_offset":26884,"ancestor_record_offset":2700,"inlinee":12282,"source_line":1709,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":394,"record_offset":27160,"parent_record_offset":27108,"ancestor_record_offset":2700,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":7048,"parent_record_offset":7008,"ancestor_record_offset":2700,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":11736,"parent_record_offset":11712,"ancestor_record_offset":2700,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":5768,"parent_record_offset":5744,"ancestor_record_offset":2700,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":23108,"parent_record_offset":22932,"ancestor_record_offset":2700,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":29320,"parent_record_offset":29292,"ancestor_record_offset":2700,"inlinee":10754,"source_line":2264,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":2700,"inlinee":11829,"source_line":3524,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":13672,"parent_record_offset":13648,"ancestor_record_offset":2700,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":7856,"parent_record_offset":7008,"ancestor_record_offset":2700,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":11296,"parent_record_offset":11260,"ancestor_record_offset":2700,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":7884,"parent_record_offset":7008,"ancestor_record_offset":2700,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":7168,"parent_record_offset":7008,"ancestor_record_offset":2700,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":13232,"parent_record_offset":13192,"ancestor_record_offset":2700,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":13264,"parent_record_offset":13232,"ancestor_record_offset":2700,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":6220,"parent_record_offset":6196,"ancestor_record_offset":2700,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":23220,"parent_record_offset":22932,"ancestor_record_offset":2700,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":13888,"parent_record_offset":13844,"ancestor_record_offset":2700,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$5::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":29408,"parent_record_offset":29320,"ancestor_record_offset":2700,"inlinee":10756,"source_line":3173,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":5744,"parent_record_offset":5516,"ancestor_record_offset":2700,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":394,"record_offset":27568,"parent_record_offset":27544,"ancestor_record_offset":2700,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":394,"record_offset":27616,"parent_record_offset":27592,"ancestor_record_offset":2700,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":394,"record_offset":14000,"parent_record_offset":13888,"ancestor_record_offset":2700,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":394,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":2700,"inlinee":11862,"source_line":18,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":394,"record_offset":13648,"parent_record_offset":13192,"ancestor_record_offset":2700,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":394,"record_offset":13156,"parent_record_offset":13120,"ancestor_record_offset":2700,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":394,"record_offset":13192,"parent_record_offset":13156,"ancestor_record_offset":2700,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":394,"record_offset":11912,"parent_record_offset":11580,"ancestor_record_offset":2700,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":394,"record_offset":8008,"parent_record_offset":7980,"ancestor_record_offset":2700,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":394,"record_offset":11200,"parent_record_offset":11176,"ancestor_record_offset":2700,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":394,"record_offset":25808,"parent_record_offset":25784,"ancestor_record_offset":2700,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":394,"record_offset":11580,"parent_record_offset":11532,"ancestor_record_offset":2700,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":394,"record_offset":23380,"parent_record_offset":23356,"ancestor_record_offset":2700,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":394,"record_offset":23356,"parent_record_offset":23276,"ancestor_record_offset":2700,"inlinee":9990,"source_line":164,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":394,"record_offset":11260,"parent_record_offset":11200,"ancestor_record_offset":2700,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":394,"record_offset":5492,"parent_record_offset":5452,"ancestor_record_offset":2700,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":394,"record_offset":5516,"parent_record_offset":5492,"ancestor_record_offset":2700,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":394,"record_offset":7276,"parent_record_offset":7244,"ancestor_record_offset":2700,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":394,"record_offset":7304,"parent_record_offset":7276,"ancestor_record_offset":2700,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::writer::EventWriter::write_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":394,"record_offset":23276,"parent_record_offset":22932,"ancestor_record_offset":2700,"inlinee":9992,"source_line":95,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":394,"record_offset":12932,"parent_record_offset":12908,"ancestor_record_offset":2700,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":394,"record_offset":12384,"parent_record_offset":11532,"ancestor_record_offset":2700,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":394,"record_offset":12408,"parent_record_offset":12384,"ancestor_record_offset":2700,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":394,"record_offset":12908,"parent_record_offset":11532,"ancestor_record_offset":2700,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":394,"record_offset":12956,"parent_record_offset":12932,"ancestor_record_offset":2700,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":394,"record_offset":24540,"parent_record_offset":24516,"ancestor_record_offset":2700,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":394,"record_offset":24564,"parent_record_offset":24540,"ancestor_record_offset":2700,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":394,"record_offset":25432,"parent_record_offset":25408,"ancestor_record_offset":2700,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":394,"record_offset":25480,"parent_record_offset":25456,"ancestor_record_offset":2700,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":394,"record_offset":25408,"parent_record_offset":23668,"ancestor_record_offset":2700,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":394,"record_offset":24516,"parent_record_offset":23668,"ancestor_record_offset":2700,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":394,"record_offset":12856,"parent_record_offset":12832,"ancestor_record_offset":2700,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":394,"record_offset":12520,"parent_record_offset":12492,"ancestor_record_offset":2700,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":394,"record_offset":12492,"parent_record_offset":12464,"ancestor_record_offset":2700,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":394,"record_offset":12716,"parent_record_offset":12692,"ancestor_record_offset":2700,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":394,"record_offset":12464,"parent_record_offset":12432,"ancestor_record_offset":2700,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":394,"record_offset":12432,"parent_record_offset":12408,"ancestor_record_offset":2700,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":394,"record_offset":12124,"parent_record_offset":11532,"ancestor_record_offset":2700,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":394,"record_offset":24908,"parent_record_offset":24564,"ancestor_record_offset":2700,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":394,"record_offset":12832,"parent_record_offset":12808,"ancestor_record_offset":2700,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":394,"record_offset":12808,"parent_record_offset":12784,"ancestor_record_offset":2700,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":394,"record_offset":24932,"parent_record_offset":24908,"ancestor_record_offset":2700,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":394,"record_offset":11532,"parent_record_offset":11500,"ancestor_record_offset":2700,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":394,"record_offset":5452,"parent_record_offset":5428,"ancestor_record_offset":2700,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":394,"record_offset":12980,"parent_record_offset":12956,"ancestor_record_offset":2700,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":394,"record_offset":13428,"parent_record_offset":13404,"ancestor_record_offset":2700,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":394,"record_offset":13452,"parent_record_offset":13428,"ancestor_record_offset":2700,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":394,"record_offset":11972,"parent_record_offset":11944,"ancestor_record_offset":2700,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":394,"record_offset":4780,"parent_record_offset":4752,"ancestor_record_offset":2700,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":394,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":2700,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":394,"record_offset":4808,"parent_record_offset":4780,"ancestor_record_offset":2700,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":394,"record_offset":4728,"parent_record_offset":4700,"ancestor_record_offset":2700,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":394,"record_offset":4752,"parent_record_offset":4728,"ancestor_record_offset":2700,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":394,"record_offset":4924,"parent_record_offset":4752,"ancestor_record_offset":2700,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":394,"record_offset":5036,"parent_record_offset":5012,"ancestor_record_offset":2700,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":394,"record_offset":9648,"parent_record_offset":9624,"ancestor_record_offset":2700,"inlinee":5087,"source_line":1371,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":394,"record_offset":29432,"parent_record_offset":29408,"ancestor_record_offset":2700,"inlinee":10758,"source_line":1976,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":394,"record_offset":11880,"parent_record_offset":11856,"ancestor_record_offset":2700,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":394,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":2700,"inlinee":14516,"source_line":1820,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":394,"record_offset":10904,"parent_record_offset":10880,"ancestor_record_offset":2700,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":394,"record_offset":19984,"parent_record_offset":19960,"ancestor_record_offset":2700,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":394,"record_offset":8252,"parent_record_offset":8224,"ancestor_record_offset":2700,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":394,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":2700,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":394,"record_offset":8312,"parent_record_offset":8280,"ancestor_record_offset":2700,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_pause
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":394,"record_offset":27240,"parent_record_offset":27216,"ancestor_record_offset":2700,"inlinee":11098,"source_line":22,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":394,"record_offset":20748,"parent_record_offset":20720,"ancestor_record_offset":2700,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":394,"record_offset":29256,"parent_record_offset":29232,"ancestor_record_offset":2700,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::spin_loop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":394,"record_offset":27216,"parent_record_offset":27108,"ancestor_record_offset":2700,"inlinee":11097,"source_line":268,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":394,"record_offset":9588,"parent_record_offset":9564,"ancestor_record_offset":2700,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":394,"record_offset":9348,"parent_record_offset":9308,"ancestor_record_offset":2700,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":394,"record_offset":16292,"parent_record_offset":16260,"ancestor_record_offset":2700,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":394,"record_offset":7368,"parent_record_offset":7340,"ancestor_record_offset":2700,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":394,"record_offset":7340,"parent_record_offset":7304,"ancestor_record_offset":2700,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":394,"record_offset":7540,"parent_record_offset":7512,"ancestor_record_offset":2700,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":394,"record_offset":7196,"parent_record_offset":7008,"ancestor_record_offset":2700,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":394,"record_offset":6164,"parent_record_offset":6140,"ancestor_record_offset":2700,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":394,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":2700,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":394,"record_offset":13800,"parent_record_offset":13744,"ancestor_record_offset":2700,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":394,"record_offset":23244,"parent_record_offset":23220,"ancestor_record_offset":2700,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":394,"record_offset":15068,"parent_record_offset":14952,"ancestor_record_offset":2700,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":394,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":2700,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":394,"record_offset":7732,"parent_record_offset":7244,"ancestor_record_offset":2700,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":394,"record_offset":13696,"parent_record_offset":13672,"ancestor_record_offset":2700,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":394,"record_offset":13844,"parent_record_offset":13800,"ancestor_record_offset":2700,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":394,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":2700,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":394,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":2700,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":394,"record_offset":13476,"parent_record_offset":13452,"ancestor_record_offset":2700,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":394,"record_offset":5700,"parent_record_offset":5672,"ancestor_record_offset":2700,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":394,"record_offset":13500,"parent_record_offset":13476,"ancestor_record_offset":2700,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":394,"record_offset":5560,"parent_record_offset":5516,"ancestor_record_offset":2700,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":394,"record_offset":20536,"parent_record_offset":20508,"ancestor_record_offset":2700,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":394,"record_offset":18420,"parent_record_offset":18392,"ancestor_record_offset":2700,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":394,"record_offset":3168,"parent_record_offset":3136,"ancestor_record_offset":2700,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":394,"record_offset":8852,"parent_record_offset":8820,"ancestor_record_offset":2700,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":394,"record_offset":8820,"parent_record_offset":8788,"ancestor_record_offset":2700,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":394,"record_offset":11856,"parent_record_offset":11580,"ancestor_record_offset":2700,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":394,"record_offset":21060,"parent_record_offset":20980,"ancestor_record_offset":2700,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":394,"record_offset":21032,"parent_record_offset":20980,"ancestor_record_offset":2700,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":394,"record_offset":21004,"parent_record_offset":20980,"ancestor_record_offset":2700,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":394,"record_offset":9624,"parent_record_offset":9500,"ancestor_record_offset":2700,"inlinee":4196,"source_line":166,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":394,"record_offset":4596,"parent_record_offset":4552,"ancestor_record_offset":2700,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":394,"record_offset":7756,"parent_record_offset":7732,"ancestor_record_offset":2700,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":394,"record_offset":24664,"parent_record_offset":24640,"ancestor_record_offset":2700,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":394,"record_offset":7396,"parent_record_offset":7368,"ancestor_record_offset":2700,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":394,"record_offset":12652,"parent_record_offset":12628,"ancestor_record_offset":2700,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":394,"record_offset":7804,"parent_record_offset":7780,"ancestor_record_offset":2700,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":394,"record_offset":12692,"parent_record_offset":12492,"ancestor_record_offset":2700,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":394,"record_offset":7512,"parent_record_offset":7340,"ancestor_record_offset":2700,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":394,"record_offset":3416,"parent_record_offset":3388,"ancestor_record_offset":2700,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":394,"record_offset":5384,"parent_record_offset":4552,"ancestor_record_offset":2700,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":394,"record_offset":14620,"parent_record_offset":14212,"ancestor_record_offset":2700,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":394,"record_offset":4988,"parent_record_offset":4700,"ancestor_record_offset":2700,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":394,"record_offset":6032,"parent_record_offset":6008,"ancestor_record_offset":2700,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":394,"record_offset":9112,"parent_record_offset":9088,"ancestor_record_offset":2700,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":394,"record_offset":18596,"parent_record_offset":18572,"ancestor_record_offset":2700,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":394,"record_offset":7672,"parent_record_offset":7648,"ancestor_record_offset":2700,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":394,"record_offset":27184,"parent_record_offset":27160,"ancestor_record_offset":2700,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":394,"record_offset":3880,"parent_record_offset":3856,"ancestor_record_offset":2700,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":394,"record_offset":10060,"parent_record_offset":10004,"ancestor_record_offset":2700,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":394,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":2700,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":394,"record_offset":4064,"parent_record_offset":4036,"ancestor_record_offset":2700,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":394,"record_offset":10084,"parent_record_offset":10060,"ancestor_record_offset":2700,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":394,"record_offset":27060,"parent_record_offset":27032,"ancestor_record_offset":2700,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":394,"record_offset":4880,"parent_record_offset":4856,"ancestor_record_offset":2700,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":394,"record_offset":15152,"parent_record_offset":15124,"ancestor_record_offset":2700,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":394,"record_offset":13048,"parent_record_offset":11532,"ancestor_record_offset":2700,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":394,"record_offset":26760,"parent_record_offset":26732,"ancestor_record_offset":2700,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":394,"record_offset":26788,"parent_record_offset":26760,"ancestor_record_offset":2700,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":394,"record_offset":5616,"parent_record_offset":5516,"ancestor_record_offset":2700,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":394,"record_offset":5984,"parent_record_offset":5516,"ancestor_record_offset":2700,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":394,"record_offset":11680,"parent_record_offset":11652,"ancestor_record_offset":2700,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":394,"record_offset":11652,"parent_record_offset":11612,"ancestor_record_offset":2700,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":394,"record_offset":24832,"parent_record_offset":24800,"ancestor_record_offset":2700,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":394,"record_offset":5672,"parent_record_offset":5644,"ancestor_record_offset":2700,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":394,"record_offset":5644,"parent_record_offset":5616,"ancestor_record_offset":2700,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":394,"record_offset":6008,"parent_record_offset":5984,"ancestor_record_offset":2700,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":394,"record_offset":3808,"parent_record_offset":3784,"ancestor_record_offset":2700,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":394,"record_offset":7648,"parent_record_offset":7624,"ancestor_record_offset":2700,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":394,"record_offset":3832,"parent_record_offset":3808,"ancestor_record_offset":2700,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":394,"record_offset":14856,"parent_record_offset":14832,"ancestor_record_offset":2700,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":394,"record_offset":4136,"parent_record_offset":4108,"ancestor_record_offset":2700,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":394,"record_offset":13744,"parent_record_offset":13720,"ancestor_record_offset":2700,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":394,"record_offset":7448,"parent_record_offset":7424,"ancestor_record_offset":2700,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":394,"record_offset":3784,"parent_record_offset":3668,"ancestor_record_offset":2700,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":394,"record_offset":7624,"parent_record_offset":7600,"ancestor_record_offset":2700,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":394,"record_offset":14832,"parent_record_offset":14752,"ancestor_record_offset":2700,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":394,"record_offset":9564,"parent_record_offset":9540,"ancestor_record_offset":2700,"inlinee":10771,"source_line":302,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insert_tail
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":394,"record_offset":9500,"parent_record_offset":9404,"ancestor_record_offset":2700,"inlinee":10769,"source_line":542,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":394,"record_offset":9404,"parent_record_offset":9308,"ancestor_record_offset":2700,"inlinee":7664,"source_line":580,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::unstable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\unstable\mod.rs
//     evidence={"module":394,"record_offset":9308,"parent_record_offset":9272,"ancestor_record_offset":2700,"inlinee":7632,"source_line":20,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":394,"record_offset":3136,"parent_record_offset":3108,"ancestor_record_offset":2700,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":394,"record_offset":3496,"parent_record_offset":3468,"ancestor_record_offset":2700,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":394,"record_offset":3636,"parent_record_offset":3604,"ancestor_record_offset":2700,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":394,"record_offset":3604,"parent_record_offset":3568,"ancestor_record_offset":2700,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":394,"record_offset":3544,"parent_record_offset":3520,"ancestor_record_offset":2700,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":394,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":2700,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":394,"record_offset":3520,"parent_record_offset":3496,"ancestor_record_offset":2700,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":394,"record_offset":4108,"parent_record_offset":3568,"ancestor_record_offset":2700,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":394,"record_offset":3108,"parent_record_offset":3076,"ancestor_record_offset":2700,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":394,"record_offset":3076,"parent_record_offset":3052,"ancestor_record_offset":2700,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":394,"record_offset":3268,"parent_record_offset":3236,"ancestor_record_offset":2700,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":394,"record_offset":3364,"parent_record_offset":3268,"ancestor_record_offset":2700,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":394,"record_offset":3236,"parent_record_offset":3212,"ancestor_record_offset":2700,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":394,"record_offset":26704,"parent_record_offset":26676,"ancestor_record_offset":2700,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":394,"record_offset":3052,"parent_record_offset":3020,"ancestor_record_offset":2700,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":394,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":2700,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":394,"record_offset":3668,"parent_record_offset":3636,"ancestor_record_offset":2700,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":394,"record_offset":3952,"parent_record_offset":3668,"ancestor_record_offset":2700,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":394,"record_offset":3924,"parent_record_offset":3668,"ancestor_record_offset":2700,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":394,"record_offset":3728,"parent_record_offset":3668,"ancestor_record_offset":2700,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":394,"record_offset":27328,"parent_record_offset":27108,"ancestor_record_offset":2700,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":394,"record_offset":27272,"parent_record_offset":27108,"ancestor_record_offset":2700,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":394,"record_offset":27352,"parent_record_offset":27328,"ancestor_record_offset":2700,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":394,"record_offset":27296,"parent_record_offset":27272,"ancestor_record_offset":2700,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":394,"record_offset":26636,"parent_record_offset":26608,"ancestor_record_offset":2700,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::rt::RtMesh::set_albedo
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/mod.rs
//     evidence={"module":394,"record_offset":23040,"parent_record_offset":22932,"ancestor_record_offset":2700,"inlinee":4592,"source_line":67,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$11::partial_cmp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":394,"record_offset":9672,"parent_record_offset":9648,"ancestor_record_offset":2700,"inlinee":24458,"source_line":69,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":394,"record_offset":10928,"parent_record_offset":10904,"ancestor_record_offset":2700,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":394,"record_offset":11944,"parent_record_offset":11532,"ancestor_record_offset":2700,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":394,"record_offset":12020,"parent_record_offset":11996,"ancestor_record_offset":2700,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":394,"record_offset":12044,"parent_record_offset":12020,"ancestor_record_offset":2700,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":394,"record_offset":11996,"parent_record_offset":11972,"ancestor_record_offset":2700,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":394,"record_offset":8748,"parent_record_offset":8708,"ancestor_record_offset":2700,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":394,"record_offset":8788,"parent_record_offset":8748,"ancestor_record_offset":2700,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":394,"record_offset":8896,"parent_record_offset":8708,"ancestor_record_offset":2700,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":394,"record_offset":8708,"parent_record_offset":8660,"ancestor_record_offset":2700,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":394,"record_offset":10388,"parent_record_offset":10276,"ancestor_record_offset":2700,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":394,"record_offset":8224,"parent_record_offset":8196,"ancestor_record_offset":2700,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":394,"record_offset":11000,"parent_record_offset":10276,"ancestor_record_offset":2700,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":394,"record_offset":8280,"parent_record_offset":8224,"ancestor_record_offset":2700,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":394,"record_offset":8988,"parent_record_offset":8660,"ancestor_record_offset":2700,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":394,"record_offset":10472,"parent_record_offset":10276,"ancestor_record_offset":2700,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":394,"record_offset":10360,"parent_record_offset":10276,"ancestor_record_offset":2700,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":394,"record_offset":18544,"parent_record_offset":17588,"ancestor_record_offset":2700,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":394,"record_offset":10880,"parent_record_offset":10856,"ancestor_record_offset":2700,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":394,"record_offset":10144,"parent_record_offset":10120,"ancestor_record_offset":2700,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":394,"record_offset":10168,"parent_record_offset":10144,"ancestor_record_offset":2700,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":394,"record_offset":8088,"parent_record_offset":8064,"ancestor_record_offset":2700,"inlinee":13574,"source_line":799,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":394,"record_offset":16232,"parent_record_offset":15328,"ancestor_record_offset":2700,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":394,"record_offset":15328,"parent_record_offset":15288,"ancestor_record_offset":2700,"inlinee":7717,"source_line":34,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":394,"record_offset":10856,"parent_record_offset":10688,"ancestor_record_offset":2700,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":394,"record_offset":8608,"parent_record_offset":8584,"ancestor_record_offset":2700,"inlinee":11990,"source_line":3219,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":394,"record_offset":10200,"parent_record_offset":10168,"ancestor_record_offset":2700,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":10796,"parent_record_offset":10712,"ancestor_record_offset":2700,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":10740,"parent_record_offset":10712,"ancestor_record_offset":2700,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":9088,"parent_record_offset":8660,"ancestor_record_offset":2700,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":18572,"parent_record_offset":17588,"ancestor_record_offset":2700,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":11092,"parent_record_offset":10276,"ancestor_record_offset":2700,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":8196,"parent_record_offset":8136,"ancestor_record_offset":2700,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":8660,"parent_record_offset":8632,"ancestor_record_offset":2700,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":10712,"parent_record_offset":10688,"ancestor_record_offset":2700,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":10252,"parent_record_offset":10228,"ancestor_record_offset":2700,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":10228,"parent_record_offset":10168,"ancestor_record_offset":2700,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":17588,"parent_record_offset":17552,"ancestor_record_offset":2700,"inlinee":6287,"source_line":1062,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":8112,"parent_record_offset":8088,"ancestor_record_offset":2700,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":16260,"parent_record_offset":16232,"ancestor_record_offset":2700,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":18460,"parent_record_offset":17588,"ancestor_record_offset":2700,"inlinee":6232,"source_line":2195,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":8168,"parent_record_offset":8136,"ancestor_record_offset":2700,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":10276,"parent_record_offset":10252,"ancestor_record_offset":2700,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":17684,"parent_record_offset":17640,"ancestor_record_offset":2700,"inlinee":6231,"source_line":1805,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":17868,"parent_record_offset":17684,"ancestor_record_offset":2700,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":18064,"parent_record_offset":17684,"ancestor_record_offset":2700,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":18100,"parent_record_offset":18064,"ancestor_record_offset":2700,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":8136,"parent_record_offset":8112,"ancestor_record_offset":2700,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":17640,"parent_record_offset":17588,"ancestor_record_offset":2700,"inlinee":6230,"source_line":1762,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":10332,"parent_record_offset":10276,"ancestor_record_offset":2700,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":18392,"parent_record_offset":18368,"ancestor_record_offset":2700,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":18368,"parent_record_offset":17640,"ancestor_record_offset":2700,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":8632,"parent_record_offset":8608,"ancestor_record_offset":2700,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":394,"record_offset":10688,"parent_record_offset":10276,"ancestor_record_offset":2700,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":394,"record_offset":17552,"parent_record_offset":17516,"ancestor_record_offset":2700,"inlinee":7725,"source_line":512,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":394,"record_offset":16012,"parent_record_offset":15844,"ancestor_record_offset":2700,"inlinee":7719,"source_line":36,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":394,"record_offset":20980,"parent_record_offset":20916,"ancestor_record_offset":2700,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":394,"record_offset":12740,"parent_record_offset":12716,"ancestor_record_offset":2700,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":394,"record_offset":4480,"parent_record_offset":4444,"ancestor_record_offset":2700,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":394,"record_offset":28188,"parent_record_offset":28156,"ancestor_record_offset":2700,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":394,"record_offset":28704,"parent_record_offset":28444,"ancestor_record_offset":2700,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":394,"record_offset":4624,"parent_record_offset":4596,"ancestor_record_offset":2700,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":394,"record_offset":4700,"parent_record_offset":4552,"ancestor_record_offset":2700,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":394,"record_offset":4960,"parent_record_offset":4700,"ancestor_record_offset":2700,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":394,"record_offset":8064,"parent_record_offset":8040,"ancestor_record_offset":2700,"inlinee":13573,"source_line":570,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":394,"record_offset":17516,"parent_record_offset":17440,"ancestor_record_offset":2700,"inlinee":7723,"source_line":3192,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":394,"record_offset":4652,"parent_record_offset":4624,"ancestor_record_offset":2700,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":394,"record_offset":4552,"parent_record_offset":4516,"ancestor_record_offset":2700,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":394,"record_offset":4516,"parent_record_offset":4480,"ancestor_record_offset":2700,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_walls::construct_walls::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":394,"record_offset":7780,"parent_record_offset":7756,"ancestor_record_offset":2700,"inlinee":24509,"source_line":76,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_walls::construct_walls::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":394,"record_offset":14720,"parent_record_offset":14692,"ancestor_record_offset":2700,"inlinee":24510,"source_line":161,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_walls::construct_walls::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":394,"record_offset":20812,"parent_record_offset":20784,"ancestor_record_offset":2700,"inlinee":24515,"source_line":321,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_walls::construct_walls::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":394,"record_offset":20916,"parent_record_offset":20888,"ancestor_record_offset":2700,"inlinee":24516,"source_line":340,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_walls::construct_walls::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":394,"record_offset":29232,"parent_record_offset":29208,"ancestor_record_offset":2700,"inlinee":24523,"source_line":389,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":394,"record_offset":15124,"parent_record_offset":14952,"ancestor_record_offset":2700,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::construct_walls::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FBD
// 首条位置: {"module":394,"inlinee":24509}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":394,"record_offset":7804,"parent_record_offset":7780,"ancestor_record_offset":7780,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::construct_walls::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 161
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FBE
// 首条位置: {"module":394,"inlinee":24510}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::construct_walls::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 321
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FC3
// 首条位置: {"module":394,"inlinee":24515}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":394,"record_offset":20848,"parent_record_offset":20812,"ancestor_record_offset":20812,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::construct_walls::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 340
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FC4
// 首条位置: {"module":394,"inlinee":24516}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":394,"record_offset":20948,"parent_record_offset":20916,"ancestor_record_offset":20916,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":394,"record_offset":20980,"parent_record_offset":20916,"ancestor_record_offset":20916,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":394,"record_offset":21060,"parent_record_offset":20980,"ancestor_record_offset":20916,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":394,"record_offset":21032,"parent_record_offset":20980,"ancestor_record_offset":20916,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":394,"record_offset":21004,"parent_record_offset":20980,"ancestor_record_offset":20916,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::construct_walls::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 369
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FC7
// 首条位置: {"module":394,"inlinee":24519}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":23108,"parent_record_offset":22932,"ancestor_record_offset":22932,"inlinee":4586,"source_line":2652,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":23220,"parent_record_offset":22932,"ancestor_record_offset":22932,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":394,"record_offset":22984,"parent_record_offset":22932,"ancestor_record_offset":22932,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":394,"record_offset":23276,"parent_record_offset":22932,"ancestor_record_offset":22932,"inlinee":9992,"source_line":95,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":394,"record_offset":23136,"parent_record_offset":22932,"ancestor_record_offset":22932,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::rt::RtMesh::set_albedo
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/mod.rs
//     evidence={"module":394,"record_offset":23040,"parent_record_offset":22932,"ancestor_record_offset":22932,"inlinee":4592,"source_line":67,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":394,"record_offset":23504,"parent_record_offset":23480,"ancestor_record_offset":22932,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":394,"record_offset":23552,"parent_record_offset":23528,"ancestor_record_offset":22932,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":394,"record_offset":23528,"parent_record_offset":23504,"ancestor_record_offset":22932,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":23480,"parent_record_offset":23440,"ancestor_record_offset":22932,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":23440,"parent_record_offset":23404,"ancestor_record_offset":22932,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":394,"record_offset":23008,"parent_record_offset":22984,"ancestor_record_offset":22932,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":394,"record_offset":23300,"parent_record_offset":23276,"ancestor_record_offset":22932,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":394,"record_offset":23380,"parent_record_offset":23356,"ancestor_record_offset":22932,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_default
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":394,"record_offset":23356,"parent_record_offset":23276,"ancestor_record_offset":22932,"inlinee":9990,"source_line":164,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":394,"record_offset":23404,"parent_record_offset":23380,"ancestor_record_offset":22932,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":394,"record_offset":23076,"parent_record_offset":23040,"ancestor_record_offset":22932,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":394,"record_offset":23244,"parent_record_offset":23220,"ancestor_record_offset":22932,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":394,"record_offset":23184,"parent_record_offset":23160,"ancestor_record_offset":22932,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":394,"record_offset":23592,"parent_record_offset":23440,"ancestor_record_offset":22932,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":394,"record_offset":23160,"parent_record_offset":23136,"ancestor_record_offset":22932,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::construct_walls::closure$6::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 380
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1146E
// 首条位置: {"module":1415,"inlinee":70766}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::construct_walls::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 389
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FCB
// 首条位置: {"module":394,"inlinee":24523}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":394,"record_offset":29256,"parent_record_offset":29232,"ancestor_record_offset":29232,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00394.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_walls::create_wall
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 424
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FCA
// 首条位置: {"module":394,"inlinee":24522}
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
// 完整结构及原始类型记录: construct_walls.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: construct_walls.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: construct_walls.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)
// 候选参数类型: ["tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>", "bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::construct_walls
// 信息 ID: 8a1bca9d0800f3a7aaefdfa0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 72d0f828a0fdc6bc2e6a27fc: void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> > @ 0x204b -> 0x757800 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> > >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::walls::plaster::RecalculateHalfTimberMeshCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> > > @ 0x430de5 -> 0x757800 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::construct_walls::closure$1
// 信息 ID: ae26a0e2e511c11b111fc4c6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_walls::construct_walls", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=394, inline_record=7780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::construct_walls::closure$3
// 信息 ID: 8ab5a87dd2980fb2fcadd6cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::construct_walls::construct_walls", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=394, inline_record=14720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::construct_walls::closure$4
// 信息 ID: 7289af49f9fc52fe6b4103d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::construct_walls::construct_walls", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=394, inline_record=20812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::construct_walls::closure$5
// 信息 ID: 16cecb063def1e30d8dc86be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_wall_constructor::construct_walls::construct_walls", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=394, inline_record=20916; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::construct_walls::closure$6
// 信息 ID: 456f093cdfc1baeb83c31743
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "system_wall_constructor::construct_walls::construct_walls", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=394, inline_record=22932; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=394, inline_record=25896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::construct_walls::closure$6::closure$0
// 信息 ID: a348042c19f1d5433b961ffe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "system_wall_constructor::construct_walls::construct_walls", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_walls::construct_walls::closure$6::closure_env$0 <- alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_walls::construct_walls::closure$6::closure_env$0>,alloc::alloc::Global>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_extend::impl$1::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_walls::construct_walls::closure$6::closure_env$0>,alloc::alloc::Global>; module=1415, inline_record=23936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::construct_walls::closure$7
// 信息 ID: 8d38b00c1133af83476f4dd0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "system_wall_constructor::construct_walls::construct_walls", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=394, inline_record=29232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_walls::create_wall
// 信息 ID: 99883d51cac018485748c0b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=394, inline_record=27392; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
