// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/wall/eraser/wall_eraser.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 692f4df8547ff0c379ee74a9b2190f68e6894a65918f16352f6dd7718def0744
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_eraser.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 575, 1643, 1682
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_eraser
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":575,"record_offset":45160,"rva":12099584,"va":5380808704}
// 编译实例 RVAs: 0xb8a000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t3827454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8fe663a00","size":5,"soft_fail":false,"target_va":5384644912,"va":5380817453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3794214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e826e53900","size":5,"soft_fail":false,"target_va":5384609408,"va":5380815189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-5133537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e81fabb1ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380812220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=20
//     evidence={"assembly":"\tcallq\t-5130883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e87db5b1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380809582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12090014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8628547ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380812201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f890
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11592096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8601e4fff","size":5,"soft_fail":false,"target_va":5369231504,"va":5380823595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=10
//     target_aliases=11 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11579332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e83c504fff","size":5,"soft_fail":false,"target_va":5369240528,"va":5380819855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bd090
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11334371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e81d0d53ff","size":5,"soft_fail":false,"target_va":5369483408,"va":5380817774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403ca630
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8125824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8800284ff","size":5,"soft_fail":false,"target_va":5372683824,"va":5380809643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403cc380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8118284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8f41f84ff","size":5,"soft_fail":false,"target_va":5372691328,"va":5380809607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=9
//     target_aliases=6 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7332227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e87d1e90ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5380811262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409792d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2183629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e833aedeff","size":5,"soft_fail":false,"target_va":5378642640,"va":5380826264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140983970
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2139179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8d55bdfff","size":5,"soft_fail":false,"target_va":5378685296,"va":5380824470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098b930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2107684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8dcd6dfff","size":5,"soft_fail":false,"target_va":5378718000,"va":5380825679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a978c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1002147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e85db5f0ff","size":5,"soft_fail":false,"target_va":5379815616,"va":5380817758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a99710
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-990226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8eee3f0ff","size":5,"soft_fail":false,"target_va":5379823376,"va":5380813597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c71040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t938289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e831510e00","size":5,"soft_fail":false,"target_va":5381754944,"va":5380816650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2456965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8857d2500","size":5,"soft_fail":false,"target_va":5383273968,"va":5380816998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2503405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8ed322600","size":5,"soft_fail":false,"target_va":5383325056,"va":5380821646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414786b0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9354025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e829bb8e00","size":5,"soft_fail":false,"target_va":5390173872,"va":5380819842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415fede0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10959319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8d739a700","size":5,"soft_fail":false,"target_va":5391773152,"va":5380813828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141617850
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11063429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e885d0a800","size":5,"soft_fail":false,"target_va":5391874128,"va":5380810694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11951067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8db5bb600","size":5,"soft_fail":false,"target_va":5392780768,"va":5380829696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196dd90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14557680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8f021de00","size":5,"soft_fail":false,"target_va":5395373456,"va":5380815771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141971340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14575413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e83567de00","size":5,"soft_fail":false,"target_va":5395387200,"va":5380811782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35534815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8df371e02","size":5,"soft_fail":false,"target_va":5416350384,"va":5380815564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35528863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e89f201e02","size":5,"soft_fail":false,"target_va":5416350608,"va":5380821740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35540878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e88e4f1e02","size":5,"soft_fail":false,"target_va":5416350720,"va":5380809837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35533099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e82b311e02","size":5,"soft_fail":false,"target_va":5416350944,"va":5380817840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35540812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e84c4f1e02","size":5,"soft_fail":false,"target_va":5416351056,"va":5380810239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35562395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e89ba31e02","size":5,"soft_fail":false,"target_va":5416385024,"va":5380822624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d7a5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35573084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e85ccd1e02","size":5,"soft_fail":false,"target_va":5416396272,"va":5380823183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_eraser.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36247382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e856172902","size":5,"soft_fail":false,"target_va":5417057056,"va":5380809669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t35567298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8c2b61e02","size":5,"soft_fail":false,"target_va":5416380224,"va":5380812921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t35798919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8873f2202","size":5,"soft_fail":false,"target_va":5416620794,"va":5380821870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35798789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8053f2202","size":5,"soft_fail":false,"target_va":5416621408,"va":5380822614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t35800116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e834442202","size":5,"soft_fail":false,"target_va":5416621600,"va":5380821479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t35799527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8e7412202","size":5,"soft_fail":false,"target_va":5416622208,"va":5380822676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t35798772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8f43e2202","size":5,"soft_fail":false,"target_va":5416621762,"va":5380822985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8b95c0000","size":5,"soft_fail":false,"target_va":5380833440,"va":5380809698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35798317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e82d3d2202","size":5,"soft_fail":false,"target_va":5416621376,"va":5380823054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1302604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8b41fecff","size":5,"soft_fail":false,"target_va":5379511040,"va":5380813639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::on_wall_split
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1343743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8017febff","size":5,"soft_fail":false,"target_va":5379477184,"va":5380820922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::new_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t493564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8fc870700","size":5,"soft_fail":false,"target_va":5381310896,"va":5380817327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::remove
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t490492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8fc7b0700","size":5,"soft_fail":false,"target_va":5381310080,"va":5380819583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t484953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e859660700","size":5,"soft_fail":false,"target_va":5381304336,"va":5380819378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t488757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e835750700","size":5,"soft_fail":false,"target_va":5381307200,"va":5380818438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_enclosure
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t489803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e84b790700","size":5,"soft_fail":false,"target_va":5381308304,"va":5380818496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_component
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t88082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e812580100","size":5,"soft_fail":false,"target_va":5380902880,"va":5380814793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2687641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e867fdd6ff","size":5,"soft_fail":false,"target_va":5378121712,"va":5380809348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::add_fully_erased_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1832218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8e60ae4ff","size":5,"soft_fail":false,"target_va":5378987328,"va":5380819541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::add_split_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1834308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8bc02e4ff","size":5,"soft_fail":false,"target_va":5378986208,"va":5380820511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::cache_if_original_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1825542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8fa24e4ff","size":5,"soft_fail":false,"target_va":5378988144,"va":5380813681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1819810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e85e3be4ff","size":5,"soft_fail":false,"target_va":5378990144,"va":5380809949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1819874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e81e3be4ff","size":5,"soft_fail":false,"target_va":5378989648,"va":5380809517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::record_rewiring_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1834473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e81702e4ff","size":5,"soft_fail":false,"target_va":5378986960,"va":5380821428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::shrink_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1830457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8c711e4ff","size":5,"soft_fail":false,"target_va":5378985424,"va":5380815876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::geo::rewire_enclosed_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t786756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e844010c00","size":5,"soft_fail":false,"target_va":5381598752,"va":5380811991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3153272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8781d3000","size":5,"soft_fail":false,"target_va":5383963792,"va":5380810515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3161779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8b33e3000","size":5,"soft_fail":false,"target_va":5383972224,"va":5380810440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3142534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e886f32f00","size":5,"soft_fail":false,"target_va":5383964912,"va":5380822373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3153137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8f11c3000","size":5,"soft_fail":false,"target_va":5383962816,"va":5380809674,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t3908680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e848a43b00","size":5,"soft_fail":false,"target_va":5384726688,"va":5380818003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35796710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8e6362202","size":5,"soft_fail":false,"target_va":5416619360,"va":5380822645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::intersect_ray
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2491637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8f5042600","size":5,"soft_fail":false,"target_va":5383304128,"va":5380812486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2495169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8c1122600","size":5,"soft_fail":false,"target_va":5383305920,"va":5380810746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2469595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380808704,"raw_hex":"e8dbae2500","size":5,"soft_fail":false,"target_va":5383281568,"va":5380811968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":57360,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":71000,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":60340,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":67300,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":51840,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":575,"record_offset":51896,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":575,"record_offset":45772,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":575,"record_offset":47372,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":575,"record_offset":64676,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":575,"record_offset":69392,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":575,"record_offset":57840,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":575,"record_offset":81836,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":575,"record_offset":51744,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":575,"record_offset":68748,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":575,"record_offset":70428,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":575,"record_offset":78992,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":46040,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":575,"record_offset":61000,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":575,"record_offset":69292,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::tuple::impl$10::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\tuple.rs
//     evidence={"module":575,"record_offset":60116,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":14084,"source_line":30,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::WallColorTable::get
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":575,"record_offset":61196,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":30588,"source_line":67,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":575,"record_offset":51224,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":28649,"source_line":35,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::latest
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":575,"record_offset":70856,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":26924,"source_line":205,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::impl$19::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":575,"record_offset":71272,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":32676,"source_line":33,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::StructureType::is_fence
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":575,"record_offset":61168,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":31532,"source_line":197,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallState::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":575,"record_offset":64212,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":36484,"source_line":249,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":575,"record_offset":53264,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":32576,"source_line":256,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallStyle::convert_to_stone_version
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":575,"record_offset":77616,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":30592,"source_line":100,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$45::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":575,"record_offset":55404,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":30580,"source_line":115,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":575,"record_offset":82796,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":29283,"source_line":128,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::get_unwrapped
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":575,"record_offset":52260,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":32591,"source_line":137,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::has_room_for
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":575,"record_offset":64564,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":36492,"source_line":78,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::wall_ids
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":575,"record_offset":51280,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":36474,"source_line":43,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":575,"record_offset":69420,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":28986,"source_line":38,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":575,"record_offset":45544,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":28328,"source_line":110,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":575,"record_offset":45892,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":28143,"source_line":287,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":575,"record_offset":45628,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":28326,"source_line":297,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer::contains
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs
//     evidence={"module":575,"record_offset":52044,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":36476,"source_line":67,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::archivist::WallEraserArchivist::affected_walls
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/archivist.rs
//     evidence={"module":575,"record_offset":48192,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":30662,"source_line":39,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::archivist::WallEraserArchivist::has_anything_been_affected
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/archivist.rs
//     evidence={"module":575,"record_offset":47288,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":30667,"source_line":43,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":575,"record_offset":64240,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":36486,"source_line":584,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":575,"record_offset":64404,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":36489,"source_line":606,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":575,"record_offset":58188,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":36482,"source_line":214,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::wall_eraser::wall_height_based_on_length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":575,"record_offset":70560,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":36494,"source_line":634,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::child_wall::ChildWall::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/child_wall.rs
//     evidence={"module":575,"record_offset":70884,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":36495,"source_line":17,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":575,"record_offset":54900,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":575,"record_offset":54844,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":575,"record_offset":64732,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":575,"record_offset":64592,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":575,"record_offset":50416,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":575,"record_offset":46900,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":575,"record_offset":49440,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$49::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":575,"record_offset":48556,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":11018,"source_line":2201,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":575,"record_offset":54360,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_u_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":575,"record_offset":55020,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":26649,"source_line":845,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":575,"record_offset":67396,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":28987,"source_line":165,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":575,"record_offset":57332,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::segments_with_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":575,"record_offset":57388,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":36479,"source_line":146,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":575,"record_offset":58556,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":26650,"source_line":136,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::max
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":575,"record_offset":58132,"parent_record_offset":45160,"ancestor_record_offset":45160,"inlinee":36480,"source_line":28,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":56000,"parent_record_offset":55976,"ancestor_record_offset":45160,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":56024,"parent_record_offset":56000,"ancestor_record_offset":45160,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":46436,"parent_record_offset":46408,"ancestor_record_offset":45160,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":46464,"parent_record_offset":46436,"ancestor_record_offset":45160,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":56048,"parent_record_offset":56024,"ancestor_record_offset":45160,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":46496,"parent_record_offset":46464,"ancestor_record_offset":45160,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":55976,"parent_record_offset":55796,"ancestor_record_offset":45160,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":46408,"parent_record_offset":46220,"ancestor_record_offset":45160,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":47588,"parent_record_offset":47564,"ancestor_record_offset":45160,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":55732,"parent_record_offset":55700,"ancestor_record_offset":45160,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":57084,"parent_record_offset":57056,"ancestor_record_offset":45160,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":57056,"parent_record_offset":57028,"ancestor_record_offset":45160,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":47636,"parent_record_offset":47612,"ancestor_record_offset":45160,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":47612,"parent_record_offset":47588,"ancestor_record_offset":45160,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":55796,"parent_record_offset":55764,"ancestor_record_offset":45160,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":55764,"parent_record_offset":55732,"ancestor_record_offset":45160,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":57028,"parent_record_offset":57004,"ancestor_record_offset":45160,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":55832,"parent_record_offset":55796,"ancestor_record_offset":45160,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":575,"record_offset":56280,"parent_record_offset":56208,"ancestor_record_offset":45160,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":575,"record_offset":55660,"parent_record_offset":55588,"ancestor_record_offset":45160,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":47564,"parent_record_offset":47524,"ancestor_record_offset":45160,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":60516,"parent_record_offset":60492,"ancestor_record_offset":45160,"inlinee":4484,"source_line":1834,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":55632,"parent_record_offset":55588,"ancestor_record_offset":45160,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":60464,"parent_record_offset":60376,"ancestor_record_offset":45160,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":47492,"parent_record_offset":47456,"ancestor_record_offset":45160,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":47524,"parent_record_offset":47492,"ancestor_record_offset":45160,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":60376,"parent_record_offset":60340,"ancestor_record_offset":45160,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":60964,"parent_record_offset":60376,"ancestor_record_offset":45160,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":55700,"parent_record_offset":55660,"ancestor_record_offset":45160,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":60492,"parent_record_offset":60376,"ancestor_record_offset":45160,"inlinee":4483,"source_line":3736,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":55552,"parent_record_offset":55488,"ancestor_record_offset":45160,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":54416,"parent_record_offset":54360,"ancestor_record_offset":45160,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":51772,"parent_record_offset":51744,"ancestor_record_offset":45160,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":60660,"parent_record_offset":60376,"ancestor_record_offset":45160,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":64868,"parent_record_offset":64844,"ancestor_record_offset":45160,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":55588,"parent_record_offset":55552,"ancestor_record_offset":45160,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":575,"record_offset":77916,"parent_record_offset":77892,"ancestor_record_offset":45160,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":575,"record_offset":78632,"parent_record_offset":78608,"ancestor_record_offset":45160,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":575,"record_offset":68904,"parent_record_offset":68864,"ancestor_record_offset":45160,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=32
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":575,"record_offset":45800,"parent_record_offset":45772,"ancestor_record_offset":45160,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":575,"record_offset":47396,"parent_record_offset":47372,"ancestor_record_offset":45160,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":575,"record_offset":49080,"parent_record_offset":49052,"ancestor_record_offset":45160,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":575,"record_offset":47456,"parent_record_offset":47396,"ancestor_record_offset":45160,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":575,"record_offset":55856,"parent_record_offset":55832,"ancestor_record_offset":45160,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":575,"record_offset":50744,"parent_record_offset":50716,"ancestor_record_offset":45160,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":575,"record_offset":71904,"parent_record_offset":71876,"ancestor_record_offset":45160,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":575,"record_offset":50772,"parent_record_offset":50744,"ancestor_record_offset":45160,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":575,"record_offset":50692,"parent_record_offset":50664,"ancestor_record_offset":45160,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":575,"record_offset":50716,"parent_record_offset":50692,"ancestor_record_offset":45160,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":575,"record_offset":50832,"parent_record_offset":50716,"ancestor_record_offset":45160,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":575,"record_offset":72048,"parent_record_offset":71336,"ancestor_record_offset":45160,"inlinee":5355,"source_line":2346,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":575,"record_offset":50944,"parent_record_offset":50920,"ancestor_record_offset":45160,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$62::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":575,"record_offset":71336,"parent_record_offset":71272,"ancestor_record_offset":45160,"inlinee":23161,"source_line":2572,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":575,"record_offset":57552,"parent_record_offset":57528,"ancestor_record_offset":45160,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":575,"record_offset":58256,"parent_record_offset":58232,"ancestor_record_offset":45160,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":575,"record_offset":70912,"parent_record_offset":70884,"ancestor_record_offset":45160,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":575,"record_offset":55332,"parent_record_offset":55308,"ancestor_record_offset":45160,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":575,"record_offset":55356,"parent_record_offset":55332,"ancestor_record_offset":45160,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":575,"record_offset":70960,"parent_record_offset":70936,"ancestor_record_offset":45160,"inlinee":11574,"source_line":1957,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":575,"record_offset":52992,"parent_record_offset":52968,"ancestor_record_offset":45160,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":575,"record_offset":57528,"parent_record_offset":57504,"ancestor_record_offset":45160,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":575,"record_offset":48468,"parent_record_offset":48444,"ancestor_record_offset":45160,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":575,"record_offset":52560,"parent_record_offset":52536,"ancestor_record_offset":45160,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":575,"record_offset":51652,"parent_record_offset":51628,"ancestor_record_offset":45160,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":575,"record_offset":70596,"parent_record_offset":70560,"ancestor_record_offset":45160,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":575,"record_offset":58156,"parent_record_offset":58132,"ancestor_record_offset":45160,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":575,"record_offset":70768,"parent_record_offset":70560,"ancestor_record_offset":45160,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":575,"record_offset":54980,"parent_record_offset":54956,"ancestor_record_offset":45160,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":575,"record_offset":60748,"parent_record_offset":60376,"ancestor_record_offset":45160,"inlinee":10761,"source_line":713,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":575,"record_offset":53060,"parent_record_offset":52424,"ancestor_record_offset":45160,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":575,"record_offset":55936,"parent_record_offset":55880,"ancestor_record_offset":45160,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":575,"record_offset":56784,"parent_record_offset":56736,"ancestor_record_offset":45160,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":575,"record_offset":57736,"parent_record_offset":57712,"ancestor_record_offset":45160,"inlinee":10930,"source_line":91,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":575,"record_offset":56736,"parent_record_offset":56280,"ancestor_record_offset":45160,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":575,"record_offset":57712,"parent_record_offset":57684,"ancestor_record_offset":45160,"inlinee":5521,"source_line":587,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":575,"record_offset":57480,"parent_record_offset":57456,"ancestor_record_offset":45160,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":575,"record_offset":57920,"parent_record_offset":57892,"ancestor_record_offset":45160,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":575,"record_offset":68796,"parent_record_offset":68772,"ancestor_record_offset":45160,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":575,"record_offset":57864,"parent_record_offset":57840,"ancestor_record_offset":45160,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":575,"record_offset":57504,"parent_record_offset":57480,"ancestor_record_offset":45160,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":575,"record_offset":57944,"parent_record_offset":57920,"ancestor_record_offset":45160,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":575,"record_offset":51800,"parent_record_offset":51772,"ancestor_record_offset":45160,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":575,"record_offset":57456,"parent_record_offset":57424,"ancestor_record_offset":45160,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":575,"record_offset":56936,"parent_record_offset":56280,"ancestor_record_offset":45160,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":575,"record_offset":50796,"parent_record_offset":50772,"ancestor_record_offset":45160,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":575,"record_offset":46312,"parent_record_offset":46284,"ancestor_record_offset":45160,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":575,"record_offset":46340,"parent_record_offset":46312,"ancestor_record_offset":45160,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":575,"record_offset":57600,"parent_record_offset":57388,"ancestor_record_offset":45160,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":575,"record_offset":57116,"parent_record_offset":57084,"ancestor_record_offset":45160,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":575,"record_offset":71932,"parent_record_offset":71876,"ancestor_record_offset":45160,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":575,"record_offset":52792,"parent_record_offset":52764,"ancestor_record_offset":45160,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":575,"record_offset":48796,"parent_record_offset":48772,"ancestor_record_offset":45160,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":575,"record_offset":54800,"parent_record_offset":54776,"ancestor_record_offset":45160,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":575,"record_offset":54716,"parent_record_offset":54692,"ancestor_record_offset":45160,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":575,"record_offset":54868,"parent_record_offset":54844,"ancestor_record_offset":45160,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":575,"record_offset":50564,"parent_record_offset":50524,"ancestor_record_offset":45160,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":575,"record_offset":55308,"parent_record_offset":55284,"ancestor_record_offset":45160,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":575,"record_offset":55284,"parent_record_offset":55248,"ancestor_record_offset":45160,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":575,"record_offset":45736,"parent_record_offset":45652,"ancestor_record_offset":45160,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":575,"record_offset":64700,"parent_record_offset":64676,"ancestor_record_offset":45160,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":575,"record_offset":64276,"parent_record_offset":64240,"ancestor_record_offset":45160,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":575,"record_offset":53232,"parent_record_offset":52260,"ancestor_record_offset":45160,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":575,"record_offset":51248,"parent_record_offset":51224,"ancestor_record_offset":45160,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":575,"record_offset":70512,"parent_record_offset":70484,"ancestor_record_offset":45160,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":575,"record_offset":69052,"parent_record_offset":68864,"ancestor_record_offset":45160,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":575,"record_offset":47236,"parent_record_offset":47208,"ancestor_record_offset":45160,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":575,"record_offset":51180,"parent_record_offset":50524,"ancestor_record_offset":45160,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":575,"record_offset":56104,"parent_record_offset":55660,"ancestor_record_offset":45160,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":52584,"parent_record_offset":52560,"ancestor_record_offset":45160,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=60
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":46068,"parent_record_offset":46040,"ancestor_record_offset":45160,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":575,"record_offset":51864,"parent_record_offset":51840,"ancestor_record_offset":45160,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":575,"record_offset":71720,"parent_record_offset":71600,"ancestor_record_offset":45160,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":575,"record_offset":46376,"parent_record_offset":46252,"ancestor_record_offset":45160,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":575,"record_offset":52012,"parent_record_offset":51896,"ancestor_record_offset":45160,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":575,"record_offset":49840,"parent_record_offset":49816,"ancestor_record_offset":45160,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":575,"record_offset":51956,"parent_record_offset":51896,"ancestor_record_offset":45160,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":575,"record_offset":49816,"parent_record_offset":49792,"ancestor_record_offset":45160,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":575,"record_offset":50024,"parent_record_offset":49996,"ancestor_record_offset":45160,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":51980,"parent_record_offset":51956,"ancestor_record_offset":45160,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":71744,"parent_record_offset":71720,"ancestor_record_offset":45160,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":47676,"parent_record_offset":47524,"ancestor_record_offset":45160,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":575,"record_offset":50116,"parent_record_offset":50092,"ancestor_record_offset":45160,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":575,"record_offset":50092,"parent_record_offset":50068,"ancestor_record_offset":45160,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":575,"record_offset":58316,"parent_record_offset":58188,"ancestor_record_offset":45160,"inlinee":5770,"source_line":304,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":575,"record_offset":54444,"parent_record_offset":54416,"ancestor_record_offset":45160,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":575,"record_offset":54472,"parent_record_offset":54444,"ancestor_record_offset":45160,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":575,"record_offset":61024,"parent_record_offset":61000,"ancestor_record_offset":45160,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":575,"record_offset":49768,"parent_record_offset":49744,"ancestor_record_offset":45160,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":575,"record_offset":49792,"parent_record_offset":49768,"ancestor_record_offset":45160,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":575,"record_offset":50140,"parent_record_offset":50116,"ancestor_record_offset":45160,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":575,"record_offset":52104,"parent_record_offset":52068,"ancestor_record_offset":45160,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":575,"record_offset":49744,"parent_record_offset":49632,"ancestor_record_offset":45160,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":575,"record_offset":47028,"parent_record_offset":47004,"ancestor_record_offset":45160,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":575,"record_offset":49472,"parent_record_offset":49440,"ancestor_record_offset":45160,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":575,"record_offset":49604,"parent_record_offset":49576,"ancestor_record_offset":45160,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":575,"record_offset":49576,"parent_record_offset":49544,"ancestor_record_offset":45160,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":575,"record_offset":49520,"parent_record_offset":49496,"ancestor_record_offset":45160,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":575,"record_offset":49544,"parent_record_offset":49520,"ancestor_record_offset":45160,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":575,"record_offset":49496,"parent_record_offset":49472,"ancestor_record_offset":45160,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":575,"record_offset":50068,"parent_record_offset":49544,"ancestor_record_offset":45160,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":575,"record_offset":47004,"parent_record_offset":46972,"ancestor_record_offset":45160,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":575,"record_offset":46972,"parent_record_offset":46944,"ancestor_record_offset":45160,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":575,"record_offset":47128,"parent_record_offset":47096,"ancestor_record_offset":45160,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":575,"record_offset":47184,"parent_record_offset":47128,"ancestor_record_offset":45160,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":575,"record_offset":47096,"parent_record_offset":47068,"ancestor_record_offset":45160,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":575,"record_offset":46944,"parent_record_offset":46900,"ancestor_record_offset":45160,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":575,"record_offset":47208,"parent_record_offset":47184,"ancestor_record_offset":45160,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":575,"record_offset":49632,"parent_record_offset":49604,"ancestor_record_offset":45160,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":575,"record_offset":49912,"parent_record_offset":49632,"ancestor_record_offset":45160,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":575,"record_offset":49884,"parent_record_offset":49632,"ancestor_record_offset":45160,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":575,"record_offset":49688,"parent_record_offset":49632,"ancestor_record_offset":45160,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":575,"record_offset":64616,"parent_record_offset":64592,"ancestor_record_offset":45160,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":575,"record_offset":64640,"parent_record_offset":64616,"ancestor_record_offset":45160,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::ne
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\tuple.rs
//     evidence={"module":575,"record_offset":58232,"parent_record_offset":58188,"ancestor_record_offset":45160,"inlinee":27934,"source_line":34,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":575,"record_offset":64328,"parent_record_offset":64304,"ancestor_record_offset":45160,"inlinee":28991,"source_line":224,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":575,"record_offset":64520,"parent_record_offset":64496,"ancestor_record_offset":45160,"inlinee":31232,"source_line":201,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":575,"record_offset":52220,"parent_record_offset":52196,"ancestor_record_offset":45160,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":575,"record_offset":70484,"parent_record_offset":70456,"ancestor_record_offset":45160,"inlinee":29140,"source_line":190,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":575,"record_offset":45568,"parent_record_offset":45544,"ancestor_record_offset":45160,"inlinee":28117,"source_line":101,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":575,"record_offset":45916,"parent_record_offset":45892,"ancestor_record_offset":45160,"inlinee":28114,"source_line":165,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::intersecting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":575,"record_offset":45676,"parent_record_offset":45652,"ancestor_record_offset":45160,"inlinee":28107,"source_line":183,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::is_active
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":575,"record_offset":45652,"parent_record_offset":45628,"ancestor_record_offset":45160,"inlinee":28116,"source_line":198,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$14::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":575,"record_offset":45704,"parent_record_offset":45676,"ancestor_record_offset":45160,"inlinee":28109,"source_line":14,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":575,"record_offset":45592,"parent_record_offset":45568,"ancestor_record_offset":45160,"inlinee":28119,"source_line":183,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::animate_wall_bulldoze::impl$2::contains::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs
//     evidence={"module":575,"record_offset":52196,"parent_record_offset":52068,"ancestor_record_offset":45160,"inlinee":36478,"source_line":68,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":575,"record_offset":64304,"parent_record_offset":64276,"ancestor_record_offset":45160,"inlinee":36488,"source_line":593,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":575,"record_offset":64472,"parent_record_offset":64444,"ancestor_record_offset":45160,"inlinee":36491,"source_line":617,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":575,"record_offset":60684,"parent_record_offset":60660,"ancestor_record_offset":45160,"inlinee":36483,"source_line":306,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":575,"record_offset":70456,"parent_record_offset":70428,"ancestor_record_offset":45160,"inlinee":36493,"source_line":469,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":575,"record_offset":57300,"parent_record_offset":55404,"ancestor_record_offset":45160,"inlinee":30582,"source_line":11,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":575,"record_offset":54924,"parent_record_offset":54900,"ancestor_record_offset":45160,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":575,"record_offset":54664,"parent_record_offset":54636,"ancestor_record_offset":45160,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":575,"record_offset":54692,"parent_record_offset":54664,"ancestor_record_offset":45160,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":575,"record_offset":54776,"parent_record_offset":54664,"ancestor_record_offset":45160,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":575,"record_offset":48740,"parent_record_offset":48712,"ancestor_record_offset":45160,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":575,"record_offset":48772,"parent_record_offset":48740,"ancestor_record_offset":45160,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":575,"record_offset":82548,"parent_record_offset":82396,"ancestor_record_offset":45160,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=11
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":575,"record_offset":48712,"parent_record_offset":48672,"ancestor_record_offset":45160,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":575,"record_offset":52536,"parent_record_offset":52424,"ancestor_record_offset":45160,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":575,"record_offset":48444,"parent_record_offset":48416,"ancestor_record_offset":45160,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":575,"record_offset":53088,"parent_record_offset":52424,"ancestor_record_offset":45160,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":575,"record_offset":51628,"parent_record_offset":51572,"ancestor_record_offset":45160,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=18
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":575,"record_offset":52620,"parent_record_offset":52424,"ancestor_record_offset":45160,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":575,"record_offset":52480,"parent_record_offset":52424,"ancestor_record_offset":45160,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":575,"record_offset":52968,"parent_record_offset":52944,"ancestor_record_offset":45160,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":575,"record_offset":52316,"parent_record_offset":52292,"ancestor_record_offset":45160,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":575,"record_offset":47336,"parent_record_offset":47312,"ancestor_record_offset":45160,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":575,"record_offset":48288,"parent_record_offset":48256,"ancestor_record_offset":45160,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":575,"record_offset":52944,"parent_record_offset":52836,"ancestor_record_offset":45160,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":575,"record_offset":48612,"parent_record_offset":48584,"ancestor_record_offset":45160,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":575,"record_offset":52348,"parent_record_offset":52316,"ancestor_record_offset":45160,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":52884,"parent_record_offset":52860,"ancestor_record_offset":45160,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":48892,"parent_record_offset":48672,"ancestor_record_offset":45160,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":53180,"parent_record_offset":52424,"ancestor_record_offset":45160,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":48416,"parent_record_offset":48352,"ancestor_record_offset":45160,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":48672,"parent_record_offset":48640,"ancestor_record_offset":45160,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":52860,"parent_record_offset":52836,"ancestor_record_offset":45160,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":52400,"parent_record_offset":52376,"ancestor_record_offset":45160,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":52376,"parent_record_offset":52316,"ancestor_record_offset":45160,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":48320,"parent_record_offset":48288,"ancestor_record_offset":45160,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":46252,"parent_record_offset":46220,"ancestor_record_offset":45160,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":51484,"parent_record_offset":51420,"ancestor_record_offset":45160,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":48388,"parent_record_offset":48352,"ancestor_record_offset":45160,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":46164,"parent_record_offset":46140,"ancestor_record_offset":45160,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":52424,"parent_record_offset":52400,"ancestor_record_offset":45160,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":46220,"parent_record_offset":46164,"ancestor_record_offset":45160,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":46192,"parent_record_offset":46164,"ancestor_record_offset":45160,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":48352,"parent_record_offset":48320,"ancestor_record_offset":45160,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":52508,"parent_record_offset":52424,"ancestor_record_offset":45160,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":46284,"parent_record_offset":46252,"ancestor_record_offset":45160,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":46140,"parent_record_offset":46116,"ancestor_record_offset":45160,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":48640,"parent_record_offset":48612,"ancestor_record_offset":45160,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":575,"record_offset":52836,"parent_record_offset":52424,"ancestor_record_offset":45160,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":575,"record_offset":70628,"parent_record_offset":70560,"ancestor_record_offset":45160,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":575,"record_offset":64780,"parent_record_offset":64756,"ancestor_record_offset":45160,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":575,"record_offset":64756,"parent_record_offset":64732,"ancestor_record_offset":45160,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":575,"record_offset":64804,"parent_record_offset":64780,"ancestor_record_offset":45160,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":575,"record_offset":50452,"parent_record_offset":50416,"ancestor_record_offset":45160,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":575,"record_offset":83828,"parent_record_offset":83796,"ancestor_record_offset":45160,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":575,"record_offset":84280,"parent_record_offset":84076,"ancestor_record_offset":45160,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":575,"record_offset":50592,"parent_record_offset":50564,"ancestor_record_offset":45160,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":575,"record_offset":50664,"parent_record_offset":50524,"ancestor_record_offset":45160,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":575,"record_offset":50868,"parent_record_offset":50664,"ancestor_record_offset":45160,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":575,"record_offset":52292,"parent_record_offset":52260,"ancestor_record_offset":45160,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":575,"record_offset":47312,"parent_record_offset":47288,"ancestor_record_offset":45160,"inlinee":12122,"source_line":708,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":575,"record_offset":48256,"parent_record_offset":48224,"ancestor_record_offset":45160,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":575,"record_offset":48224,"parent_record_offset":48192,"ancestor_record_offset":45160,"inlinee":10241,"source_line":481,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":575,"record_offset":48584,"parent_record_offset":48556,"ancestor_record_offset":45160,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":575,"record_offset":71364,"parent_record_offset":71336,"ancestor_record_offset":45160,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":575,"record_offset":72080,"parent_record_offset":71272,"ancestor_record_offset":45160,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":575,"record_offset":54956,"parent_record_offset":54924,"ancestor_record_offset":45160,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":575,"record_offset":71420,"parent_record_offset":71392,"ancestor_record_offset":45160,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":575,"record_offset":71680,"parent_record_offset":71656,"ancestor_record_offset":45160,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":575,"record_offset":71540,"parent_record_offset":71512,"ancestor_record_offset":45160,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":575,"record_offset":71876,"parent_record_offset":71476,"ancestor_record_offset":45160,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":575,"record_offset":71392,"parent_record_offset":71364,"ancestor_record_offset":45160,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":575,"record_offset":50620,"parent_record_offset":50592,"ancestor_record_offset":45160,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":575,"record_offset":71600,"parent_record_offset":71568,"ancestor_record_offset":45160,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":575,"record_offset":50524,"parent_record_offset":50488,"ancestor_record_offset":45160,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":575,"record_offset":50488,"parent_record_offset":50452,"ancestor_record_offset":45160,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":575,"record_offset":70396,"parent_record_offset":69420,"ancestor_record_offset":45160,"inlinee":28987,"source_line":165,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::points_with_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":575,"record_offset":57424,"parent_record_offset":57388,"ancestor_record_offset":45160,"inlinee":12081,"source_line":132,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":575,"record_offset":54388,"parent_record_offset":54360,"ancestor_record_offset":45160,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":575,"record_offset":54512,"parent_record_offset":54360,"ancestor_record_offset":45160,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":575,"record_offset":55248,"parent_record_offset":55020,"ancestor_record_offset":45160,"inlinee":26650,"source_line":136,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":575,"record_offset":60712,"parent_record_offset":60684,"ancestor_record_offset":45160,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":575,"record_offset":58280,"parent_record_offset":58256,"ancestor_record_offset":45160,"inlinee":17826,"source_line":6,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":575,"record_offset":70936,"parent_record_offset":70912,"ancestor_record_offset":45160,"inlinee":32708,"source_line":6,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":575,"record_offset":55488,"parent_record_offset":55404,"ancestor_record_offset":45160,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":575,"record_offset":54636,"parent_record_offset":54360,"ancestor_record_offset":45160,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 214
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E82
// 首条位置: {"module":575,"inlinee":36482}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":58344,"parent_record_offset":58188,"ancestor_record_offset":58188,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":575,"record_offset":58316,"parent_record_offset":58188,"ancestor_record_offset":58188,"inlinee":5770,"source_line":304,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::tuple::impl$10::ne
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\tuple.rs
//     evidence={"module":575,"record_offset":58232,"parent_record_offset":58188,"ancestor_record_offset":58188,"inlinee":27934,"source_line":34,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":58428,"parent_record_offset":58404,"ancestor_record_offset":58188,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":58476,"parent_record_offset":58452,"ancestor_record_offset":58188,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":575,"record_offset":58452,"parent_record_offset":58428,"ancestor_record_offset":58188,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":58404,"parent_record_offset":58368,"ancestor_record_offset":58188,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":58368,"parent_record_offset":58344,"ancestor_record_offset":58188,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":575,"record_offset":58256,"parent_record_offset":58232,"ancestor_record_offset":58188,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":58516,"parent_record_offset":58368,"ancestor_record_offset":58188,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":575,"record_offset":58280,"parent_record_offset":58256,"ancestor_record_offset":58188,"inlinee":17826,"source_line":6,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 306
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E83
// 首条位置: {"module":575,"inlinee":36483}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":575,"record_offset":60712,"parent_record_offset":60684,"ancestor_record_offset":60684,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 411
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D96
// 首条位置: {"module":1643,"inlinee":81302}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":377744,"parent_record_offset":377712,"ancestor_record_offset":377712,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":377768,"parent_record_offset":377744,"ancestor_record_offset":377712,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1643,"record_offset":377816,"parent_record_offset":377792,"ancestor_record_offset":377712,"inlinee":11574,"source_line":1957,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1643,"record_offset":377792,"parent_record_offset":377768,"ancestor_record_offset":377712,"inlinee":32708,"source_line":6,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 469
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E8D
// 首条位置: {"module":575,"inlinee":36493}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":575,"record_offset":70484,"parent_record_offset":70456,"ancestor_record_offset":70456,"inlinee":29140,"source_line":190,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":575,"record_offset":70512,"parent_record_offset":70484,"ancestor_record_offset":70456,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 584
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E86
// 首条位置: {"module":575,"inlinee":36486}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":575,"record_offset":64276,"parent_record_offset":64240,"ancestor_record_offset":64240,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":575,"record_offset":64328,"parent_record_offset":64304,"ancestor_record_offset":64240,"inlinee":28991,"source_line":224,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":575,"record_offset":64304,"parent_record_offset":64276,"ancestor_record_offset":64240,"inlinee":36488,"source_line":593,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 593
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E88
// 首条位置: {"module":575,"inlinee":36488}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":575,"record_offset":64328,"parent_record_offset":64304,"ancestor_record_offset":64304,"inlinee":28991,"source_line":224,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 606
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E89
// 首条位置: {"module":575,"inlinee":36489}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":575,"record_offset":64444,"parent_record_offset":64404,"ancestor_record_offset":64404,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":575,"record_offset":64496,"parent_record_offset":64472,"ancestor_record_offset":64404,"inlinee":28991,"source_line":224,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":575,"record_offset":64520,"parent_record_offset":64496,"ancestor_record_offset":64404,"inlinee":31232,"source_line":201,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/wall_eraser.rs
//     evidence={"module":575,"record_offset":64472,"parent_record_offset":64444,"ancestor_record_offset":64404,"inlinee":36491,"source_line":617,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 617
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E8B
// 首条位置: {"module":575,"inlinee":36491}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::is_flat_w_no_rim
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":575,"record_offset":64496,"parent_record_offset":64472,"ancestor_record_offset":64472,"inlinee":28991,"source_line":224,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":575,"record_offset":64520,"parent_record_offset":64496,"ancestor_record_offset":64472,"inlinee":31232,"source_line":201,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::wall_eraser::wall_height_based_on_length
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 634
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8E8E
// 首条位置: {"module":575,"inlinee":36494}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":575,"record_offset":70596,"parent_record_offset":70560,"ancestor_record_offset":70560,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":575,"record_offset":70768,"parent_record_offset":70560,"ancestor_record_offset":70560,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":575,"record_offset":70628,"parent_record_offset":70560,"ancestor_record_offset":70560,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":575,"record_offset":70736,"parent_record_offset":70628,"ancestor_record_offset":70560,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":575,"record_offset":70680,"parent_record_offset":70628,"ancestor_record_offset":70560,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":575,"record_offset":70652,"parent_record_offset":70628,"ancestor_record_offset":70560,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00575.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 28; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] ref$<str$> (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)
// 候选参数类型: ["tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)
// 候选参数类型: ["rhapsody::raw::buffer::Buffer"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims
// 信息 ID: 4b8b6cfee88d9e5d717d230d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=64240; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=77452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims::closure$0
// 信息 ID: c8d021633722054a0beb7458
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=64304; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=77516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty
// 信息 ID: dda3b328faac787d7e0cd56a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=64404; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=67148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty::closure$0
// 信息 ID: cc094d46200881b2eb590282
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=64472; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=67208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_eraser
// 信息 ID: 6af9cbce0ef1b305006df9d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d7a8ae042bc15ac91bae521f: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> > @ 0x2ce8e7 -> 0xb8a000 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> > >,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> @ 0x5e5d00 -> 0xb8a000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$0
// 信息 ID: 99e9f6ba32afc7f564211adb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::eraser::wall_eraser::wall_eraser", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=58188; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=58708; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=59076; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=59744; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$1
// 信息 ID: 29a86601b5952a215a911af2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::eraser::wall_eraser::wall_eraser", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=60684; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=60836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$2
// 信息 ID: f9f9f80e8816d05bba1e35d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::wall::eraser::wall_eraser::wall_eraser", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >,country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> > >,country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure_env$2> >; module=1643, inline_record=377712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_eraser::closure$3
// 信息 ID: f4e7a6162b1e029849416b78
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::systems::wall::eraser::wall_eraser::wall_eraser", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=70456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::wall_eraser::wall_height_based_on_length
// 信息 ID: 708f208245a43d95ccaecbba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=70560; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=575, inline_record=77276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
