// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/wall/wall_splitter.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: a5b3ab5230aa8665722faaa45ff363b3161e55445b3d6ba18dfa4e1a57b397c8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_splitter.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 561, 579, 1643, 1682
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
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::wall_splitter::wall_splitter
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":579,"record_offset":30656,"rva":12277088,"va":5380986208}
// 编译实例 RVAs: 0xbb5560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3652649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e829bc3700","size":5,"soft_fail":false,"target_va":5384644912,"va":5380992258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3618497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8c1363700","size":5,"soft_fail":false,"target_va":5384609408,"va":5380990906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-5310178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e81ef9aeff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380988861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-5310463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e801f8aeff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380989162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=8
//     target_aliases=1699 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12266659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e85dd344ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380988846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14007f890
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11764672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8407c4cff","size":5,"soft_fail":false,"target_va":5369231504,"va":5380996171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=2
//     target_aliases=11 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11755267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8fda04cff","size":5,"soft_fail":false,"target_va":5369240528,"va":5380995790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400bd090
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11509101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8936250ff","size":5,"soft_fail":false,"target_va":5369483408,"va":5380992504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=2
//     target_aliases=34 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11331848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8f81653ff","size":5,"soft_fail":false,"target_va":5369660368,"va":5380992211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403ca630
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8306238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8c24181ff","size":5,"soft_fail":false,"target_va":5372683824,"va":5380990057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403cc380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8298702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8325f81ff","size":5,"soft_fail":false,"target_va":5372691328,"va":5380990025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=5
//     target_aliases=6 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7508209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e80f6f8dff","size":5,"soft_fail":false,"target_va":5373479040,"va":5380987244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fa820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2864081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e82f4cd4ff","size":5,"soft_fail":false,"target_va":5378123808,"va":5380987884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140983970
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2310695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8d9bddcff","size":5,"soft_fail":false,"target_va":5378685296,"va":5380995986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a978c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1176877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8d30aeeff","size":5,"soft_fail":false,"target_va":5379815616,"va":5380992488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a99710
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1166795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e83532eeff","size":5,"soft_fail":false,"target_va":5379823376,"va":5380990166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c71040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t763158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e816a50b00","size":5,"soft_fail":false,"target_va":5381754944,"va":5380991781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c922e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t895568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e850aa0d00","size":5,"soft_fail":false,"target_va":5381890784,"va":5380995211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2336565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e835a72300","size":5,"soft_fail":false,"target_va":5383325056,"va":5380988486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416173c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10885321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8c918a600","size":5,"soft_fail":false,"target_va":5391872960,"va":5380987634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418055f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12907815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e827f5c400","size":5,"soft_fail":false,"target_va":5393896944,"va":5380989124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196dd90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14381744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8b072db00","size":5,"soft_fail":false,"target_va":5395373456,"va":5380991707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196f620
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14391520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8e098db00","size":5,"soft_fail":false,"target_va":5395379744,"va":5380988219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141970d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14394133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e815a3db00","size":5,"soft_fail":false,"target_va":5395385648,"va":5380991510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419710f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14398629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8a5b4db00","size":5,"soft_fail":false,"target_va":5395386608,"va":5380987974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35356645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8e57f1b02","size":5,"soft_fail":false,"target_va":5416350384,"va":5380993734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35356590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8ae7f1b02","size":5,"soft_fail":false,"target_va":5416350496,"va":5380993901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35358359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e897861b02","size":5,"soft_fail":false,"target_va":5416350944,"va":5380992580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35358910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8be881b02","size":5,"soft_fail":false,"target_va":5416351056,"va":5380992141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d7a5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35401492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8142f1c02","size":5,"soft_fail":false,"target_va":5416396272,"va":5380994775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36068033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8c15a2602","size":5,"soft_fail":false,"target_va":5417057056,"va":5380989018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t35626355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8739d1f02","size":5,"soft_fail":false,"target_va":5416620769,"va":5380994409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t3807161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8b9173a00","size":5,"soft_fail":false,"target_va":5384794864,"va":5380987698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35626620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e87c9e1f02","size":5,"soft_fail":false,"target_va":5416620794,"va":5380994169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t35627433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8a9a11f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5380994162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35627966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8bea31f02","size":5,"soft_fail":false,"target_va":5416622208,"va":5380994237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t35627156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e894a01f02","size":5,"soft_fail":false,"target_va":5416621762,"va":5380994601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35627907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e883a31f02","size":5,"soft_fail":false,"target_va":5416622240,"va":5380994328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::systems::ui::resources::widget_params::WidgetVolumeShape> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-209860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e83cccfcff","size":5,"soft_fail":false,"target_va":5380787328,"va":5380997183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-440543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e82147f9ff","size":5,"soft_fail":false,"target_va":5380552224,"va":5380992762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1479190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8ea6de9ff","size":5,"soft_fail":false,"target_va":5379511040,"va":5380990225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::on_wall_split
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1516404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e88cdce8ff","size":5,"soft_fail":false,"target_va":5379477184,"va":5380993583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::new_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t318811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e85bdd0400","size":5,"soft_fail":false,"target_va":5381310896,"va":5380992080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t319709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8dde00400","size":5,"soft_fail":false,"target_va":5381310080,"va":5380990366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::add_split_event
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2007163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8855fe1ff","size":5,"soft_fail":false,"target_va":5378986208,"va":5380993366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::cache_if_original_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2002135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e82973e1ff","size":5,"soft_fail":false,"target_va":5378988144,"va":5380990274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2003539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8ad6de1ff","size":5,"soft_fail":false,"target_va":5378990144,"va":5380993678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::archivist::WallEraserArchivist::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2000267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8757ae1ff","size":5,"soft_fail":false,"target_va":5378989648,"va":5380989910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3733390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e88ef73800","size":5,"soft_fail":false,"target_va":5384726688,"va":5380993293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35624565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e875961f02","size":5,"soft_fail":false,"target_va":5416619360,"va":5380994790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2293487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e8effe2200","size":5,"soft_fail":false,"target_va":5383281568,"va":5380988076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2291591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380986208,"raw_hex":"e887f72200","size":5,"soft_fail":false,"target_va":5383280256,"va":5380988660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":579,"record_offset":33496,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":579,"record_offset":36296,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":46520,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":45372,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":579,"record_offset":40164,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":579,"record_offset":48308,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":579,"record_offset":45716,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":31948,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":42476,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":579,"record_offset":46192,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":579,"record_offset":33580,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":579,"record_offset":31348,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":579,"record_offset":33412,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":579,"record_offset":41332,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":37360,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":579,"record_offset":46008,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":579,"record_offset":46092,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::WallColorTable::get
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":579,"record_offset":41372,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":30588,"source_line":67,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::latest
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":579,"record_offset":46344,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":26924,"source_line":205,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::impl$19::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":579,"record_offset":46796,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":32676,"source_line":33,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":579,"record_offset":45980,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":32576,"source_line":256,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::WallStyle::convert_to_stone_version
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":579,"record_offset":46376,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":30592,"source_line":100,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":579,"record_offset":39104,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":29283,"source_line":128,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::has_room_for
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":579,"record_offset":41248,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":36492,"source_line":78,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":579,"record_offset":33012,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":579,"record_offset":46220,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":29140,"source_line":190,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":579,"record_offset":32004,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":28986,"source_line":38,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::wall_ids
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":579,"record_offset":30980,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":32574,"source_line":59,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":579,"record_offset":40072,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":28143,"source_line":287,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":579,"record_offset":33636,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":28321,"source_line":292,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":579,"record_offset":33608,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":28125,"source_line":95,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":579,"record_offset":36520,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":28605,"source_line":191,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::OwnedSignifierStream::write
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":579,"record_offset":37332,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":28613,"source_line":95,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_splitter.rs
//     evidence={"module":579,"record_offset":33068,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":35254,"source_line":368,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::child_wall::ChildWall::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/child_wall.rs
//     evidence={"module":579,"record_offset":46404,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":36495,"source_line":17,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":579,"record_offset":42532,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":579,"record_offset":42392,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":34584,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":35496,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":26416,"source_line":792,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_u_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":33736,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":26649,"source_line":845,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":33040,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::max
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":579,"record_offset":34472,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":36480,"source_line":28,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::min
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":579,"record_offset":34528,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":36874,"source_line":35,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":579,"record_offset":34176,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":579,"record_offset":45656,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":26089,"source_line":207,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":35236,"parent_record_offset":30656,"ancestor_record_offset":30656,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":36416,"parent_record_offset":36392,"ancestor_record_offset":30656,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":36440,"parent_record_offset":36416,"ancestor_record_offset":30656,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":37536,"parent_record_offset":37512,"ancestor_record_offset":30656,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":37560,"parent_record_offset":37536,"ancestor_record_offset":30656,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":36464,"parent_record_offset":36440,"ancestor_record_offset":30656,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":37584,"parent_record_offset":37560,"ancestor_record_offset":30656,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":36392,"parent_record_offset":36360,"ancestor_record_offset":30656,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":37512,"parent_record_offset":37456,"ancestor_record_offset":30656,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":579,"record_offset":36360,"parent_record_offset":36324,"ancestor_record_offset":30656,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":579,"record_offset":36324,"parent_record_offset":36296,"ancestor_record_offset":30656,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":579,"record_offset":33544,"parent_record_offset":33520,"ancestor_record_offset":30656,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":46616,"parent_record_offset":46584,"ancestor_record_offset":30656,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":36760,"parent_record_offset":36732,"ancestor_record_offset":30656,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":37484,"parent_record_offset":37456,"ancestor_record_offset":30656,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":37456,"parent_record_offset":37432,"ancestor_record_offset":30656,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":46664,"parent_record_offset":46640,"ancestor_record_offset":30656,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":46640,"parent_record_offset":46616,"ancestor_record_offset":30656,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":36820,"parent_record_offset":36788,"ancestor_record_offset":30656,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":36788,"parent_record_offset":36760,"ancestor_record_offset":30656,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":37432,"parent_record_offset":37408,"ancestor_record_offset":30656,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":36856,"parent_record_offset":36820,"ancestor_record_offset":30656,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":579,"record_offset":36640,"parent_record_offset":36612,"ancestor_record_offset":30656,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":579,"record_offset":36668,"parent_record_offset":36640,"ancestor_record_offset":30656,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":579,"record_offset":36696,"parent_record_offset":36668,"ancestor_record_offset":30656,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":579,"record_offset":36612,"parent_record_offset":36584,"ancestor_record_offset":30656,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":579,"record_offset":36584,"parent_record_offset":36556,"ancestor_record_offset":30656,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":579,"record_offset":36556,"parent_record_offset":36520,"ancestor_record_offset":30656,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":579,"record_offset":37200,"parent_record_offset":37176,"ancestor_record_offset":30656,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":46584,"parent_record_offset":46544,"ancestor_record_offset":30656,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":48428,"parent_record_offset":48392,"ancestor_record_offset":30656,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":46544,"parent_record_offset":46520,"ancestor_record_offset":30656,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":36732,"parent_record_offset":36696,"ancestor_record_offset":30656,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":33800,"parent_record_offset":33736,"ancestor_record_offset":30656,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":33436,"parent_record_offset":33412,"ancestor_record_offset":30656,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":579,"record_offset":40188,"parent_record_offset":40164,"ancestor_record_offset":30656,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":579,"record_offset":48332,"parent_record_offset":48308,"ancestor_record_offset":30656,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":579,"record_offset":48392,"parent_record_offset":48332,"ancestor_record_offset":30656,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":579,"record_offset":36880,"parent_record_offset":36856,"ancestor_record_offset":30656,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":579,"record_offset":45740,"parent_record_offset":45716,"ancestor_record_offset":30656,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":579,"record_offset":45856,"parent_record_offset":45764,"ancestor_record_offset":30656,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":579,"record_offset":47084,"parent_record_offset":47056,"ancestor_record_offset":30656,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":579,"record_offset":47136,"parent_record_offset":47112,"ancestor_record_offset":30656,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":579,"record_offset":47112,"parent_record_offset":47056,"ancestor_record_offset":30656,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":579,"record_offset":47220,"parent_record_offset":46856,"ancestor_record_offset":30656,"inlinee":5355,"source_line":2346,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$62::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":579,"record_offset":46856,"parent_record_offset":46796,"ancestor_record_offset":30656,"inlinee":23161,"source_line":2572,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":33364,"parent_record_offset":33340,"ancestor_record_offset":30656,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":46432,"parent_record_offset":46404,"ancestor_record_offset":30656,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":34104,"parent_record_offset":34080,"ancestor_record_offset":30656,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":34128,"parent_record_offset":34104,"ancestor_record_offset":30656,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":46480,"parent_record_offset":46456,"ancestor_record_offset":30656,"inlinee":11574,"source_line":1957,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":32740,"parent_record_offset":32716,"ancestor_record_offset":30656,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":579,"record_offset":33340,"parent_record_offset":33316,"ancestor_record_offset":30656,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":579,"record_offset":31260,"parent_record_offset":31236,"ancestor_record_offset":30656,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":579,"record_offset":32308,"parent_record_offset":32284,"ancestor_record_offset":30656,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":579,"record_offset":32392,"parent_record_offset":32368,"ancestor_record_offset":30656,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":579,"record_offset":45680,"parent_record_offset":45656,"ancestor_record_offset":30656,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":579,"record_offset":34496,"parent_record_offset":34472,"ancestor_record_offset":30656,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":579,"record_offset":34552,"parent_record_offset":34528,"ancestor_record_offset":30656,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":579,"record_offset":36072,"parent_record_offset":36048,"ancestor_record_offset":30656,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":579,"record_offset":32808,"parent_record_offset":32172,"ancestor_record_offset":30656,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":579,"record_offset":36928,"parent_record_offset":36904,"ancestor_record_offset":30656,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":579,"record_offset":33292,"parent_record_offset":33268,"ancestor_record_offset":30656,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":579,"record_offset":33316,"parent_record_offset":33292,"ancestor_record_offset":30656,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":579,"record_offset":33460,"parent_record_offset":33436,"ancestor_record_offset":30656,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":579,"record_offset":33268,"parent_record_offset":33068,"ancestor_record_offset":30656,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":579,"record_offset":45880,"parent_record_offset":45856,"ancestor_record_offset":30656,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":579,"record_offset":47160,"parent_record_offset":47136,"ancestor_record_offset":30656,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":579,"record_offset":36904,"parent_record_offset":36880,"ancestor_record_offset":30656,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":579,"record_offset":40520,"parent_record_offset":40492,"ancestor_record_offset":30656,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":579,"record_offset":45824,"parent_record_offset":45800,"ancestor_record_offset":30656,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":579,"record_offset":51576,"parent_record_offset":51544,"ancestor_record_offset":30656,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":579,"record_offset":47820,"parent_record_offset":47764,"ancestor_record_offset":30656,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":579,"record_offset":31628,"parent_record_offset":31604,"ancestor_record_offset":30656,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":579,"record_offset":31604,"parent_record_offset":31580,"ancestor_record_offset":30656,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":579,"record_offset":35160,"parent_record_offset":35128,"ancestor_record_offset":30656,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":579,"record_offset":35008,"parent_record_offset":34976,"ancestor_record_offset":30656,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":579,"record_offset":35408,"parent_record_offset":35236,"ancestor_record_offset":30656,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":579,"record_offset":47400,"parent_record_offset":47364,"ancestor_record_offset":30656,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":579,"record_offset":45800,"parent_record_offset":45764,"ancestor_record_offset":30656,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":579,"record_offset":45764,"parent_record_offset":45740,"ancestor_record_offset":30656,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":579,"record_offset":34080,"parent_record_offset":34056,"ancestor_record_offset":30656,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":579,"record_offset":34056,"parent_record_offset":34020,"ancestor_record_offset":30656,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":579,"record_offset":31916,"parent_record_offset":31348,"ancestor_record_offset":30656,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":579,"record_offset":42500,"parent_record_offset":42476,"ancestor_record_offset":30656,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":579,"record_offset":33520,"parent_record_offset":33496,"ancestor_record_offset":30656,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":579,"record_offset":46248,"parent_record_offset":46220,"ancestor_record_offset":30656,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":579,"record_offset":31292,"parent_record_offset":31204,"ancestor_record_offset":30656,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":579,"record_offset":47028,"parent_record_offset":47000,"ancestor_record_offset":30656,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":579,"record_offset":37096,"parent_record_offset":36696,"ancestor_record_offset":30656,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":579,"record_offset":45904,"parent_record_offset":45880,"ancestor_record_offset":30656,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":32332,"parent_record_offset":32308,"ancestor_record_offset":30656,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":37176,"parent_record_offset":36520,"ancestor_record_offset":30656,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":579,"record_offset":33148,"parent_record_offset":33124,"ancestor_record_offset":30656,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":579,"record_offset":47608,"parent_record_offset":47488,"ancestor_record_offset":30656,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":579,"record_offset":31780,"parent_record_offset":31756,"ancestor_record_offset":30656,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":579,"record_offset":46160,"parent_record_offset":46092,"ancestor_record_offset":30656,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":579,"record_offset":46128,"parent_record_offset":46092,"ancestor_record_offset":30656,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":45928,"parent_record_offset":45904,"ancestor_record_offset":30656,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":47632,"parent_record_offset":47608,"ancestor_record_offset":30656,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":46704,"parent_record_offset":46544,"ancestor_record_offset":30656,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":579,"record_offset":33100,"parent_record_offset":33068,"ancestor_record_offset":30656,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":579,"record_offset":33828,"parent_record_offset":33800,"ancestor_record_offset":30656,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":579,"record_offset":33856,"parent_record_offset":33828,"ancestor_record_offset":30656,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":579,"record_offset":33124,"parent_record_offset":33100,"ancestor_record_offset":30656,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":37236,"parent_record_offset":37200,"ancestor_record_offset":30656,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":42416,"parent_record_offset":42392,"ancestor_record_offset":30656,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":42440,"parent_record_offset":42416,"ancestor_record_offset":30656,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":37260,"parent_record_offset":37236,"ancestor_record_offset":30656,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":579,"record_offset":37292,"parent_record_offset":37200,"ancestor_record_offset":30656,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":579,"record_offset":31972,"parent_record_offset":31948,"ancestor_record_offset":30656,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":579,"record_offset":40096,"parent_record_offset":40072,"ancestor_record_offset":30656,"inlinee":28114,"source_line":165,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::intersecting
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":579,"record_offset":33664,"parent_record_offset":33636,"ancestor_record_offset":30656,"inlinee":28107,"source_line":183,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$14::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":579,"record_offset":33700,"parent_record_offset":33664,"ancestor_record_offset":30656,"inlinee":28109,"source_line":14,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":36012,"parent_record_offset":35976,"ancestor_record_offset":30656,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":35976,"parent_record_offset":35940,"ancestor_record_offset":30656,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":35940,"parent_record_offset":35912,"ancestor_record_offset":30656,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":34948,"parent_record_offset":34920,"ancestor_record_offset":30656,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":35912,"parent_record_offset":35884,"ancestor_record_offset":30656,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":34976,"parent_record_offset":34948,"ancestor_record_offset":30656,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":35128,"parent_record_offset":34948,"ancestor_record_offset":30656,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":35792,"parent_record_offset":35496,"ancestor_record_offset":30656,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":579,"record_offset":31548,"parent_record_offset":31516,"ancestor_record_offset":30656,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":579,"record_offset":31580,"parent_record_offset":31548,"ancestor_record_offset":30656,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":579,"record_offset":31668,"parent_record_offset":31516,"ancestor_record_offset":30656,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":579,"record_offset":31516,"parent_record_offset":31476,"ancestor_record_offset":30656,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":579,"record_offset":32284,"parent_record_offset":32172,"ancestor_record_offset":30656,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":579,"record_offset":31236,"parent_record_offset":31204,"ancestor_record_offset":30656,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":579,"record_offset":32836,"parent_record_offset":32172,"ancestor_record_offset":30656,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":579,"record_offset":32368,"parent_record_offset":32172,"ancestor_record_offset":30656,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":579,"record_offset":32228,"parent_record_offset":32172,"ancestor_record_offset":30656,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":579,"record_offset":32716,"parent_record_offset":32692,"ancestor_record_offset":30656,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":579,"record_offset":32064,"parent_record_offset":32040,"ancestor_record_offset":30656,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":579,"record_offset":31076,"parent_record_offset":31044,"ancestor_record_offset":30656,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":579,"record_offset":32692,"parent_record_offset":32584,"ancestor_record_offset":30656,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":579,"record_offset":31420,"parent_record_offset":31396,"ancestor_record_offset":30656,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":579,"record_offset":32096,"parent_record_offset":32064,"ancestor_record_offset":30656,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":32632,"parent_record_offset":32608,"ancestor_record_offset":30656,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":31756,"parent_record_offset":31476,"ancestor_record_offset":30656,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":32928,"parent_record_offset":32172,"ancestor_record_offset":30656,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":31204,"parent_record_offset":31140,"ancestor_record_offset":30656,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":31476,"parent_record_offset":31444,"ancestor_record_offset":30656,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":32608,"parent_record_offset":32584,"ancestor_record_offset":30656,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":32148,"parent_record_offset":32124,"ancestor_record_offset":30656,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":32124,"parent_record_offset":32064,"ancestor_record_offset":30656,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":31108,"parent_record_offset":31076,"ancestor_record_offset":30656,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":40432,"parent_record_offset":40400,"ancestor_record_offset":30656,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":31176,"parent_record_offset":31140,"ancestor_record_offset":30656,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":40344,"parent_record_offset":40320,"ancestor_record_offset":30656,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":32172,"parent_record_offset":32148,"ancestor_record_offset":30656,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":40400,"parent_record_offset":40344,"ancestor_record_offset":30656,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":40372,"parent_record_offset":40344,"ancestor_record_offset":30656,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":31140,"parent_record_offset":31108,"ancestor_record_offset":30656,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":32256,"parent_record_offset":32172,"ancestor_record_offset":30656,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":40464,"parent_record_offset":40432,"ancestor_record_offset":30656,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":40320,"parent_record_offset":40296,"ancestor_record_offset":30656,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":31444,"parent_record_offset":31420,"ancestor_record_offset":30656,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":579,"record_offset":32584,"parent_record_offset":32172,"ancestor_record_offset":30656,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":579,"record_offset":42580,"parent_record_offset":42556,"ancestor_record_offset":30656,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":579,"record_offset":42556,"parent_record_offset":42532,"ancestor_record_offset":30656,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":579,"record_offset":42604,"parent_record_offset":42580,"ancestor_record_offset":30656,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":579,"record_offset":32040,"parent_record_offset":32004,"ancestor_record_offset":30656,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":579,"record_offset":31044,"parent_record_offset":31012,"ancestor_record_offset":30656,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":579,"record_offset":31012,"parent_record_offset":30980,"ancestor_record_offset":30656,"inlinee":10241,"source_line":481,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":579,"record_offset":31396,"parent_record_offset":31372,"ancestor_record_offset":30656,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$49::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":579,"record_offset":31372,"parent_record_offset":31348,"ancestor_record_offset":30656,"inlinee":11018,"source_line":2201,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":579,"record_offset":46884,"parent_record_offset":46856,"ancestor_record_offset":30656,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":579,"record_offset":47252,"parent_record_offset":46796,"ancestor_record_offset":30656,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":579,"record_offset":36048,"parent_record_offset":36012,"ancestor_record_offset":30656,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":579,"record_offset":46940,"parent_record_offset":46912,"ancestor_record_offset":30656,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":579,"record_offset":47568,"parent_record_offset":47544,"ancestor_record_offset":30656,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":579,"record_offset":47428,"parent_record_offset":47400,"ancestor_record_offset":30656,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":579,"record_offset":47056,"parent_record_offset":47000,"ancestor_record_offset":30656,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":579,"record_offset":46912,"parent_record_offset":46884,"ancestor_record_offset":30656,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":579,"record_offset":47456,"parent_record_offset":47428,"ancestor_record_offset":30656,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":579,"record_offset":47488,"parent_record_offset":47456,"ancestor_record_offset":30656,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":579,"record_offset":47000,"parent_record_offset":46968,"ancestor_record_offset":30656,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":579,"record_offset":46968,"parent_record_offset":46940,"ancestor_record_offset":30656,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":35540,"parent_record_offset":35496,"ancestor_record_offset":30656,"inlinee":26417,"source_line":681,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":34616,"parent_record_offset":34584,"ancestor_record_offset":30656,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":32980,"parent_record_offset":32004,"ancestor_record_offset":30656,"inlinee":28987,"source_line":165,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":33772,"parent_record_offset":33736,"ancestor_record_offset":30656,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":579,"record_offset":33896,"parent_record_offset":33736,"ancestor_record_offset":30656,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":579,"record_offset":34020,"parent_record_offset":33736,"ancestor_record_offset":30656,"inlinee":26650,"source_line":136,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$13::partial_cmp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":579,"record_offset":46456,"parent_record_offset":46432,"ancestor_record_offset":30656,"inlinee":32708,"source_line":6,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":579,"record_offset":34920,"parent_record_offset":34616,"ancestor_record_offset":30656,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":579,"record_offset":35884,"parent_record_offset":35496,"ancestor_record_offset":30656,"inlinee":26418,"source_line":99,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::wall_splitter::wall_splitter::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":579,"record_offset":52956,"rva":12288496,"va":5380997616}
// 编译实例 RVAs: 0xbb81f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5318989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380997616,"raw_hex":"e8b3d6aeff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380997672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5319338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380997616,"raw_hex":"e856d5aeff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380998037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12275470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380997616,"raw_hex":"e8f2b044ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380997657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35622791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380997616,"raw_hex":"e8878f1f02","size":5,"soft_fail":false,"target_va":5416620769,"va":5380997973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2059179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380997616,"raw_hex":"e85594e0ff","size":5,"soft_fail":false,"target_va":5378938736,"va":5380997910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":579,"record_offset":53540,"parent_record_offset":52956,"ancestor_record_offset":52956,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":579,"record_offset":53124,"parent_record_offset":52956,"ancestor_record_offset":52956,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":53624,"parent_record_offset":52956,"ancestor_record_offset":52956,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::line_topology
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":579,"record_offset":53512,"parent_record_offset":52956,"ancestor_record_offset":52956,"inlinee":28623,"source_line":284,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":579,"record_offset":53484,"parent_record_offset":52956,"ancestor_record_offset":52956,"inlinee":28622,"source_line":234,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":53208,"parent_record_offset":53184,"ancestor_record_offset":52956,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":53232,"parent_record_offset":53208,"ancestor_record_offset":52956,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":53696,"parent_record_offset":53672,"ancestor_record_offset":52956,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":53720,"parent_record_offset":53696,"ancestor_record_offset":52956,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":53256,"parent_record_offset":53232,"ancestor_record_offset":52956,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":53744,"parent_record_offset":53720,"ancestor_record_offset":52956,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":53184,"parent_record_offset":53152,"ancestor_record_offset":52956,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":53672,"parent_record_offset":53648,"ancestor_record_offset":52956,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":579,"record_offset":53152,"parent_record_offset":53124,"ancestor_record_offset":52956,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":579,"record_offset":53648,"parent_record_offset":53624,"ancestor_record_offset":52956,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":579,"record_offset":53564,"parent_record_offset":53540,"ancestor_record_offset":52956,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":53588,"parent_record_offset":53564,"ancestor_record_offset":52956,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::wall_splitter::wall_splitter::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":579,"record_offset":53796,"rva":12288944,"va":5380998064}
// 编译实例 RVAs: 0xbb83b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5319437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380998064,"raw_hex":"e8f3d4aeff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380998120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5319786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380998064,"raw_hex":"e896d3aeff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380998485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: wall_splitter.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12275918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380998064,"raw_hex":"e832af44ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380998105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35622349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380998064,"raw_hex":"e8cd8d1f02","size":5,"soft_fail":false,"target_va":5416620769,"va":5380998415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35622358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380998064,"raw_hex":"e8d68d1f02","size":5,"soft_fail":false,"target_va":5416620794,"va":5380998431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2059621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380998064,"raw_hex":"e89b92e0ff","size":5,"soft_fail":false,"target_va":5378938736,"va":5380998352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":579,"record_offset":54776,"parent_record_offset":53796,"ancestor_record_offset":53796,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":579,"record_offset":53968,"parent_record_offset":53796,"ancestor_record_offset":53796,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":579,"record_offset":54148,"parent_record_offset":53796,"ancestor_record_offset":53796,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":54860,"parent_record_offset":53796,"ancestor_record_offset":53796,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":579,"record_offset":54744,"parent_record_offset":53796,"ancestor_record_offset":53796,"inlinee":30518,"source_line":301,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":579,"record_offset":54680,"parent_record_offset":53796,"ancestor_record_offset":53796,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":54052,"parent_record_offset":54028,"ancestor_record_offset":53796,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":54076,"parent_record_offset":54052,"ancestor_record_offset":53796,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":54932,"parent_record_offset":54908,"ancestor_record_offset":53796,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":54956,"parent_record_offset":54932,"ancestor_record_offset":53796,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":54100,"parent_record_offset":54076,"ancestor_record_offset":53796,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":54980,"parent_record_offset":54956,"ancestor_record_offset":53796,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":54028,"parent_record_offset":53996,"ancestor_record_offset":53796,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":579,"record_offset":54908,"parent_record_offset":54884,"ancestor_record_offset":53796,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":579,"record_offset":53996,"parent_record_offset":53968,"ancestor_record_offset":53796,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":579,"record_offset":54884,"parent_record_offset":54860,"ancestor_record_offset":53796,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":54380,"parent_record_offset":54352,"ancestor_record_offset":53796,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":54440,"parent_record_offset":54408,"ancestor_record_offset":53796,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":579,"record_offset":54408,"parent_record_offset":54380,"ancestor_record_offset":53796,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":579,"record_offset":54800,"parent_record_offset":54776,"ancestor_record_offset":53796,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":579,"record_offset":54260,"parent_record_offset":54232,"ancestor_record_offset":53796,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":579,"record_offset":54288,"parent_record_offset":54260,"ancestor_record_offset":53796,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":579,"record_offset":54316,"parent_record_offset":54288,"ancestor_record_offset":53796,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":579,"record_offset":54232,"parent_record_offset":54204,"ancestor_record_offset":53796,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":579,"record_offset":54204,"parent_record_offset":54176,"ancestor_record_offset":53796,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":579,"record_offset":54176,"parent_record_offset":54148,"ancestor_record_offset":53796,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":54824,"parent_record_offset":54800,"ancestor_record_offset":53796,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":579,"record_offset":54352,"parent_record_offset":54316,"ancestor_record_offset":53796,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":579,"record_offset":54708,"parent_record_offset":54680,"ancestor_record_offset":53796,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":579,"record_offset":54596,"parent_record_offset":54316,"ancestor_record_offset":53796,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":579,"record_offset":54620,"parent_record_offset":54596,"ancestor_record_offset":53796,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00579.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 368
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89B6
// 首条位置: {"module":561,"inlinee":35254}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":62304,"parent_record_offset":62260,"ancestor_record_offset":62260,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":561,"record_offset":62464,"parent_record_offset":62260,"ancestor_record_offset":62260,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":62416,"parent_record_offset":62388,"ancestor_record_offset":62260,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":62388,"parent_record_offset":62360,"ancestor_record_offset":62260,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":62332,"parent_record_offset":62304,"ancestor_record_offset":62260,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":62360,"parent_record_offset":62332,"ancestor_record_offset":62260,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":62520,"parent_record_offset":62492,"ancestor_record_offset":62260,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":561,"record_offset":62492,"parent_record_offset":62464,"ancestor_record_offset":62260,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 378
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D29
// 首条位置: {"module":1643,"inlinee":81193}
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
// 完整结构及原始类型记录: wall_splitter.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_splitter.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_splitter.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 28; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)
// 候选参数类型: ["ref$<tg_mods::overlay::Mod>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] country_core::systems::wall::structure_outline::WallOutline (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] relative_path::relative_path_buf::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles
// 信息 ID: ad597bd79f99eeb31271ce48
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls; module=561, inline_record=62260; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=579, inline_record=33068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure$0
// 信息 ID: 9c1da484186eb60365a72347
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0 <- core::iter::adapters::map::impl$2::next<tuple$<f32,f32>,core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0>,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$0<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$1<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$2<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0 <- core::iter::adapters::map::impl$2::next<tuple$<f32,f32>,core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0>,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$0<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$1<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$2<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0 <- core::iter::adapters::map::impl$2::next<tuple$<f32,f32>,core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0>,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$0<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$1<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$2<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0 <- core::iter::adapters::map::impl$2::next<tuple$<f32,f32>,core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0>,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$0<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$1<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$2<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,f32>,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0>,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$0<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$1<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$2<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,f32>,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0>,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$0<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$1<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$2<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<f32,f32>,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0>,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$0<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$1<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > >,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$2<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 13 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0>; module=1643, inline_record=116148; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<array$<utils::geometry::intersect::Triangle,2>,itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0,tuple$<>,core::iter::adapters::flatten::impl$18::iter_try_fold::flatten::closure_env$0<array$<utils::geometry::intersect::Triangle,2>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<f32,f32>,tuple$<> > >,core::iter::adapters::flatten::impl$20::try_fold::flatten::closure_env$0<core::array::iter::IntoIter<utils::geometry::intersect::Triangle,2>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<f32,f32>,tuple$<> > >,core::iter::adapters::filter_map::filter_map_try_fold::closure_env$0<utils::geometry::intersect::Triangle,tuple$<f32,f32>,tuple$<>,enum2$<core::ops::control_flow::ControlFlow<tuple$<f32,f32>,tuple$<> > >,country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$0<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> >,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<f32,f32>,ref_mut$<country_core::systems::ui_wall_transform_gizmos::declipped_freehand_move_gizmo_distance::closure_env$1<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > > >,array$<utils::geometry::intersect::Triangle,2>,country_core::systems::wall::wall_splitter::freehand_wall_interaction_triangles::closure_env$0> > > > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<f32,f32>,tuple$<> > > >; module=1682, inline_record=183852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_splitter::wall_splitter
// 信息 ID: 3aa5cfbf171286e1b0584d8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 25fd0a414dbbce449cd41bd8: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >)> > @ 0x28c1e0 -> 0xbb5560 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::WoodenWallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetWoodenWallColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio> >)> @ 0x391c03 -> 0xbb5560 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_splitter::wall_splitter::closure$0
// 信息 ID: 510389ec0d6ed95eef6816cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::wall_splitter::wall_splitter", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::wall_splitter::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::wall_splitter::wall_splitter::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::wall_splitter::wall_splitter::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xbbdd20 -> 0xbb81f0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::wall_splitter::wall_splitter::closure$1
// 信息 ID: 6af733c6f5fbb5df3ce2a8d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::wall_splitter::wall_splitter", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::wall_splitter::wall_splitter::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::wall_splitter::wall_splitter::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::wall_splitter::wall_splitter::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xbbdd10 -> 0xbb83b0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
