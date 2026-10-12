// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/flag.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d5c635e918ccda6f8e5a6f2b921a84f63c4e943e9e573ef3db25460f3dc48f24
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: flag.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 476, 1557
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
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use smallvec; // namespace candidate
//   package identities: [{"name":"smallvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.13.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag::generate_flags
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":476,"record_offset":552,"rva":9234944,"va":5377944064}
// 编译实例 RVAs: 0x8cea00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2273000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e81851ddff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377951683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9226281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8d73773ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377948468,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ba370
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8485284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e85c867eff","size":5,"soft_fail":false,"target_va":5369471856,"va":5377957135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4465916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e804dbbbff","size":5,"soft_fail":false,"target_va":5373479776,"va":5377945687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t174377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e829a90200","size":5,"soft_fail":false,"target_va":5378120480,"va":5377946098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=5
//     target_aliases=20 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t180313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e859c00200","size":5,"soft_fail":false,"target_va":5378126944,"va":5377946626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb5e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t175493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e885ad0200","size":5,"soft_fail":false,"target_va":5378127328,"va":5377951830,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fbce0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t180729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8f9c10200","size":5,"soft_fail":false,"target_va":5378129120,"va":5377948386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a4f060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1573295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8af011800","size":5,"soft_fail":false,"target_va":5379518560,"va":5377945260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c5a060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3715928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e858b33800","size":5,"soft_fail":false,"target_va":5381660768,"va":5377944835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c915a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3929573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8e5f53b00","size":5,"soft_fail":false,"target_va":5381887392,"va":5377957814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de6350
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5334954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8aa675100","size":5,"soft_fail":false,"target_va":5383283536,"va":5377948577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fd3a0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11722588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e85cdfb200","size":5,"soft_fail":false,"target_va":5389669280,"va":5377946687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141498890
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12359243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e84b96bc00","size":5,"soft_fail":false,"target_va":5390305424,"va":5377946176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414988e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12351044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e84476bc00","size":5,"soft_fail":false,"target_va":5390305504,"va":5377954455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414991f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12361623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8979fbc00","size":5,"soft_fail":false,"target_va":5390307824,"va":5377946196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141499e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12363433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8a9a6bc00","size":5,"soft_fail":false,"target_va":5390311024,"va":5377947586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a11e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12392142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8ce16bd00","size":5,"soft_fail":false,"target_va":5390340576,"va":5377948429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414a3040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12394086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8661ebd00","size":5,"soft_fail":false,"target_va":5390348352,"va":5377954261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39105318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e826b35402","size":5,"soft_fail":false,"target_va":5417057056,"va":5377951733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39103118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e88eaa5402","size":5,"soft_fail":false,"target_va":5417057600,"va":5377954477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39103183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8cfaa5402","size":5,"soft_fail":false,"target_va":5417057696,"va":5377954508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39103531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e82bac5402","size":5,"soft_fail":false,"target_va":5417058032,"va":5377954496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38664364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8acf84d02","size":5,"soft_fail":false,"target_va":5416620769,"va":5377956400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t38431956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8d46c4a02","size":5,"soft_fail":false,"target_va":5416382400,"va":5377950439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::array::from_fn<f32,2,system_decorator::decorator_visual::flag::stair_banner_attachment::closure_env$2>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e86d160000","size":5,"soft_fail":false,"target_va":5377960384,"va":5377954638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38665216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e800fc4d02","size":5,"soft_fail":false,"target_va":5416621408,"va":5377956187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38665807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e84ffe4d02","size":5,"soft_fail":false,"target_va":5416621600,"va":5377955788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38665824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e860fe4d02","size":5,"soft_fail":false,"target_va":5416622208,"va":5377956379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t38665163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8cbfb4d02","size":5,"soft_fail":false,"target_va":5416621762,"va":5377956594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t38666387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e893004e02","size":5,"soft_fail":false,"target_va":5416622240,"va":5377955848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38665894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8a6fe4d02","size":5,"soft_fail":false,"target_va":5416622320,"va":5377956421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t15107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8033b0000","size":5,"soft_fail":false,"target_va":5377959328,"va":5377944216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38664909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8cdfa4d02","size":5,"soft_fail":false,"target_va":5416621376,"va":5377956462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3709819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e87b9b3800","size":5,"soft_fail":false,"target_va":5381655760,"va":5377945936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_stair
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t181417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8a9c40200","size":5,"soft_fail":false,"target_va":5378127488,"va":5377946066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t179078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e886bb0200","size":5,"soft_fail":false,"target_va":5378126544,"va":5377947461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1562497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e881d71700","size":5,"soft_fail":false,"target_va":5379507392,"va":5377944890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1556943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8cfc11700","size":5,"soft_fail":false,"target_va":5379502992,"va":5377946044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1561997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e88dd51700","size":5,"soft_fail":false,"target_va":5379509504,"va":5377947502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::iter_modified
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1529653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e835571700","size":5,"soft_fail":false,"target_va":5379475184,"va":5377945526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3704222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e89e853800","size":5,"soft_fail":false,"target_va":5381650864,"va":5377946637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3708363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8cb953800","size":5,"soft_fail":false,"target_va":5381654320,"va":5377945952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3284189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8dd1c3200","size":5,"soft_fail":false,"target_va":5381231744,"va":5377947550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2956009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8e91a2d00","size":5,"soft_fail":false,"target_va":5380903536,"va":5377947522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_cuboid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1218313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e809971200","size":5,"soft_fail":false,"target_va":5379167328,"va":5377949010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_sphere
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1212234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e84a7f1200","size":5,"soft_fail":false,"target_va":5379167632,"va":5377955393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_raycast_w_params
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1216118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8768e1200","size":5,"soft_fail":false,"target_va":5379164880,"va":5377948757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6019178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e86ad85b00","size":5,"soft_fail":false,"target_va":5383963792,"va":5377944609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6027685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8a5f95b00","size":5,"soft_fail":false,"target_va":5383972224,"va":5377944534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6020189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e85ddc5b00","size":5,"soft_fail":false,"target_va":5383964912,"va":5377944718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6018619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e83bd65b00","size":5,"soft_fail":false,"target_va":5383962816,"va":5377944192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t38761494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e816744f02","size":5,"soft_fail":false,"target_va":5416717696,"va":5377956197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12411073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8c160bd00","size":5,"soft_fail":false,"target_va":5390367248,"va":5377956170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38663116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e8ccf34d02","size":5,"soft_fail":false,"target_va":5416619360,"va":5377956239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t119897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e859d40100","size":5,"soft_fail":false,"target_va":5378070256,"va":5377950354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t125326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377944064,"raw_hex":"e88ee90100","size":5,"soft_fail":false,"target_va":5378071840,"va":5377946509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":476,"record_offset":20876,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":476,"record_offset":24516,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":19092,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":476,"record_offset":8456,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":476,"record_offset":12924,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":476,"record_offset":14648,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":476,"record_offset":4008,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":476,"record_offset":5288,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":476,"record_offset":3980,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":476,"record_offset":12036,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":3044,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":476,"record_offset":13496,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":476,"record_offset":8608,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":26677,"source_line":298,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":476,"record_offset":14236,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":27083,"source_line":285,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":476,"record_offset":16276,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":26686,"source_line":309,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":476,"record_offset":3924,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":26241,"source_line":273,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":476,"record_offset":13524,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":476,"record_offset":13552,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$31::mul_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":476,"record_offset":10720,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":11818,"source_line":1154,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$33::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":476,"record_offset":9912,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":11718,"source_line":1174,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":476,"record_offset":14208,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":476,"record_offset":14760,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":476,"record_offset":10596,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":476,"record_offset":2236,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":476,"record_offset":836,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":476,"record_offset":1224,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":476,"record_offset":12660,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":476,"record_offset":12980,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":476,"record_offset":8512,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":26690,"source_line":270,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::flag::stair_banner_attachment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/flag.rs
//     evidence={"module":476,"record_offset":6920,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":27661,"source_line":31,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::albedo_map
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":476,"record_offset":21212,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":26694,"source_line":138,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_roof_cloth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":476,"record_offset":19336,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":27665,"source_line":165,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_wall_cloth
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":476,"record_offset":22796,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":27666,"source_line":171,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":476,"record_offset":19064,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":26693,"source_line":85,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":476,"record_offset":19036,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":26692,"source_line":55,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":476,"record_offset":12072,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":476,"record_offset":14460,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":27088,"source_line":70,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":21024,"parent_record_offset":21000,"ancestor_record_offset":552,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":21048,"parent_record_offset":21024,"ancestor_record_offset":552,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":21072,"parent_record_offset":21048,"ancestor_record_offset":552,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":21000,"parent_record_offset":20960,"ancestor_record_offset":552,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":476,"record_offset":20960,"parent_record_offset":20912,"ancestor_record_offset":552,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":476,"record_offset":20912,"parent_record_offset":20876,"ancestor_record_offset":552,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":476,"record_offset":19184,"parent_record_offset":19152,"ancestor_record_offset":552,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":476,"record_offset":19232,"parent_record_offset":19208,"ancestor_record_offset":552,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":476,"record_offset":19208,"parent_record_offset":19184,"ancestor_record_offset":552,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":476,"record_offset":9760,"parent_record_offset":8648,"ancestor_record_offset":552,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":476,"record_offset":24740,"parent_record_offset":24716,"ancestor_record_offset":552,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":19152,"parent_record_offset":19116,"ancestor_record_offset":552,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":19116,"parent_record_offset":19092,"ancestor_record_offset":552,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":7456,"parent_record_offset":6920,"ancestor_record_offset":552,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":476,"record_offset":8480,"parent_record_offset":8456,"ancestor_record_offset":552,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":476,"record_offset":2564,"parent_record_offset":2536,"ancestor_record_offset":552,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":476,"record_offset":2592,"parent_record_offset":2564,"ancestor_record_offset":552,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":476,"record_offset":2512,"parent_record_offset":2484,"ancestor_record_offset":552,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":476,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":552,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":476,"record_offset":2652,"parent_record_offset":2536,"ancestor_record_offset":552,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":476,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":552,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":476,"record_offset":7400,"parent_record_offset":6920,"ancestor_record_offset":552,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":476,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":552,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":476,"record_offset":3948,"parent_record_offset":3924,"ancestor_record_offset":552,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":476,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":552,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":476,"record_offset":6204,"parent_record_offset":6180,"ancestor_record_offset":552,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":476,"record_offset":6288,"parent_record_offset":6264,"ancestor_record_offset":552,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":476,"record_offset":10380,"parent_record_offset":9960,"ancestor_record_offset":552,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":476,"record_offset":10288,"parent_record_offset":10232,"ancestor_record_offset":552,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":476,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":552,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":476,"record_offset":10088,"parent_record_offset":10004,"ancestor_record_offset":552,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":476,"record_offset":10152,"parent_record_offset":9960,"ancestor_record_offset":552,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":476,"record_offset":14548,"parent_record_offset":14524,"ancestor_record_offset":552,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":476,"record_offset":8224,"parent_record_offset":8200,"ancestor_record_offset":552,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":476,"record_offset":6704,"parent_record_offset":6068,"ancestor_record_offset":552,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":476,"record_offset":4040,"parent_record_offset":4008,"ancestor_record_offset":552,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":476,"record_offset":4132,"parent_record_offset":4104,"ancestor_record_offset":552,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":476,"record_offset":5040,"parent_record_offset":5008,"ancestor_record_offset":552,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":476,"record_offset":4256,"parent_record_offset":4228,"ancestor_record_offset":552,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":476,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":552,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":476,"record_offset":7560,"parent_record_offset":7520,"ancestor_record_offset":552,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":476,"record_offset":6436,"parent_record_offset":6408,"ancestor_record_offset":552,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":476,"record_offset":4492,"parent_record_offset":4468,"ancestor_record_offset":552,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":476,"record_offset":12624,"parent_record_offset":12600,"ancestor_record_offset":552,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":476,"record_offset":12568,"parent_record_offset":12544,"ancestor_record_offset":552,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":476,"record_offset":10628,"parent_record_offset":10596,"ancestor_record_offset":552,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":476,"record_offset":12512,"parent_record_offset":12488,"ancestor_record_offset":552,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":476,"record_offset":2384,"parent_record_offset":2344,"ancestor_record_offset":552,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":476,"record_offset":4852,"parent_record_offset":4828,"ancestor_record_offset":552,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":476,"record_offset":4072,"parent_record_offset":4040,"ancestor_record_offset":552,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":476,"record_offset":7744,"parent_record_offset":6920,"ancestor_record_offset":552,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":476,"record_offset":4948,"parent_record_offset":4164,"ancestor_record_offset":552,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":476,"record_offset":4972,"parent_record_offset":4948,"ancestor_record_offset":552,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":476,"record_offset":4828,"parent_record_offset":4256,"ancestor_record_offset":552,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":476,"record_offset":5008,"parent_record_offset":4132,"ancestor_record_offset":552,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":476,"record_offset":6876,"parent_record_offset":5900,"ancestor_record_offset":552,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":476,"record_offset":9848,"parent_record_offset":8648,"ancestor_record_offset":552,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":476,"record_offset":1172,"parent_record_offset":1144,"ancestor_record_offset":552,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":476,"record_offset":3000,"parent_record_offset":2344,"ancestor_record_offset":552,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":6228,"parent_record_offset":6204,"ancestor_record_offset":552,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":2716,"parent_record_offset":2484,"ancestor_record_offset":552,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":476,"record_offset":4612,"parent_record_offset":4588,"ancestor_record_offset":552,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":476,"record_offset":1644,"parent_record_offset":1620,"ancestor_record_offset":552,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":476,"record_offset":1620,"parent_record_offset":1596,"ancestor_record_offset":552,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":476,"record_offset":1828,"parent_record_offset":1800,"ancestor_record_offset":552,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":19272,"parent_record_offset":19116,"ancestor_record_offset":552,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":476,"record_offset":1928,"parent_record_offset":1900,"ancestor_record_offset":552,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":476,"record_offset":1900,"parent_record_offset":1872,"ancestor_record_offset":552,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":476,"record_offset":7588,"parent_record_offset":7520,"ancestor_record_offset":552,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":476,"record_offset":7488,"parent_record_offset":7456,"ancestor_record_offset":552,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":476,"record_offset":7520,"parent_record_offset":7488,"ancestor_record_offset":552,"inlinee":11671,"source_line":696,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":476,"record_offset":7828,"parent_record_offset":7800,"ancestor_record_offset":552,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":476,"record_offset":1572,"parent_record_offset":1548,"ancestor_record_offset":552,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":476,"record_offset":1596,"parent_record_offset":1572,"ancestor_record_offset":552,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":476,"record_offset":7628,"parent_record_offset":6920,"ancestor_record_offset":552,"inlinee":11954,"source_line":309,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":476,"record_offset":1956,"parent_record_offset":1928,"ancestor_record_offset":552,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":476,"record_offset":7052,"parent_record_offset":6920,"ancestor_record_offset":552,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":476,"record_offset":7080,"parent_record_offset":7052,"ancestor_record_offset":552,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":476,"record_offset":1548,"parent_record_offset":1432,"ancestor_record_offset":552,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":476,"record_offset":964,"parent_record_offset":940,"ancestor_record_offset":552,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":476,"record_offset":1260,"parent_record_offset":1224,"ancestor_record_offset":552,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":476,"record_offset":1400,"parent_record_offset":1368,"ancestor_record_offset":552,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":476,"record_offset":1368,"parent_record_offset":1332,"ancestor_record_offset":552,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":476,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":552,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":476,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":552,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":476,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":552,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":476,"record_offset":1872,"parent_record_offset":1332,"ancestor_record_offset":552,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":476,"record_offset":940,"parent_record_offset":908,"ancestor_record_offset":552,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":476,"record_offset":908,"parent_record_offset":880,"ancestor_record_offset":552,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":476,"record_offset":1064,"parent_record_offset":1032,"ancestor_record_offset":552,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":476,"record_offset":1120,"parent_record_offset":1064,"ancestor_record_offset":552,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":476,"record_offset":1032,"parent_record_offset":1004,"ancestor_record_offset":552,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":476,"record_offset":880,"parent_record_offset":836,"ancestor_record_offset":552,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":476,"record_offset":1144,"parent_record_offset":1120,"ancestor_record_offset":552,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":476,"record_offset":1432,"parent_record_offset":1400,"ancestor_record_offset":552,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":476,"record_offset":1716,"parent_record_offset":1432,"ancestor_record_offset":552,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":476,"record_offset":1688,"parent_record_offset":1432,"ancestor_record_offset":552,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":476,"record_offset":1492,"parent_record_offset":1432,"ancestor_record_offset":552,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":9788,"parent_record_offset":9760,"ancestor_record_offset":552,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":24776,"parent_record_offset":24740,"ancestor_record_offset":552,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":9812,"parent_record_offset":9788,"ancestor_record_offset":552,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":24800,"parent_record_offset":24776,"ancestor_record_offset":552,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":24832,"parent_record_offset":24740,"ancestor_record_offset":552,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_assembly_params::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_assembly_params.rs
//     evidence={"module":476,"record_offset":7424,"parent_record_offset":7400,"ancestor_record_offset":552,"inlinee":12092,"source_line":110,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":476,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":552,"inlinee":8701,"source_line":38,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":476,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":552,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$17::iter::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":476,"record_offset":4876,"parent_record_offset":4852,"ancestor_record_offset":552,"inlinee":26674,"source_line":746,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$1::iter_modified::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":476,"record_offset":5900,"parent_record_offset":5864,"ancestor_record_offset":552,"inlinee":26676,"source_line":77,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":476,"record_offset":14368,"parent_record_offset":14268,"ancestor_record_offset":552,"inlinee":27086,"source_line":118,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":476,"record_offset":8648,"parent_record_offset":8608,"ancestor_record_offset":552,"inlinee":26679,"source_line":249,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":476,"record_offset":14268,"parent_record_offset":14236,"ancestor_record_offset":552,"inlinee":27084,"source_line":241,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":476,"record_offset":16316,"parent_record_offset":16276,"ancestor_record_offset":552,"inlinee":26687,"source_line":258,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$24::eq
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":476,"record_offset":9536,"parent_record_offset":9508,"ancestor_record_offset":552,"inlinee":26681,"source_line":94,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":476,"record_offset":9872,"parent_record_offset":9848,"ancestor_record_offset":552,"inlinee":26703,"source_line":252,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":476,"record_offset":14336,"parent_record_offset":14304,"ancestor_record_offset":552,"inlinee":27106,"source_line":244,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":476,"record_offset":17540,"parent_record_offset":17516,"ancestor_record_offset":552,"inlinee":26705,"source_line":261,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":476,"record_offset":8332,"parent_record_offset":8304,"ancestor_record_offset":552,"inlinee":11738,"source_line":182,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_y
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":476,"record_offset":8304,"parent_record_offset":6920,"ancestor_record_offset":552,"inlinee":11736,"source_line":178,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":476,"record_offset":8420,"parent_record_offset":8304,"ancestor_record_offset":552,"inlinee":11856,"source_line":81,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":476,"record_offset":10768,"parent_record_offset":10744,"ancestor_record_offset":552,"inlinee":11715,"source_line":804,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":476,"record_offset":9936,"parent_record_offset":9912,"ancestor_record_offset":552,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":476,"record_offset":9960,"parent_record_offset":9936,"ancestor_record_offset":552,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::impl$27::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":476,"record_offset":10744,"parent_record_offset":10720,"ancestor_record_offset":552,"inlinee":11714,"source_line":1123,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":476,"record_offset":10408,"parent_record_offset":9960,"ancestor_record_offset":552,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":476,"record_offset":8168,"parent_record_offset":8136,"ancestor_record_offset":552,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":476,"record_offset":8136,"parent_record_offset":8112,"ancestor_record_offset":552,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":476,"record_offset":8112,"parent_record_offset":8084,"ancestor_record_offset":552,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":476,"record_offset":8084,"parent_record_offset":8060,"ancestor_record_offset":552,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":476,"record_offset":8060,"parent_record_offset":6920,"ancestor_record_offset":552,"inlinee":9810,"source_line":581,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":476,"record_offset":12544,"parent_record_offset":12296,"ancestor_record_offset":552,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":476,"record_offset":12600,"parent_record_offset":12296,"ancestor_record_offset":552,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":476,"record_offset":12488,"parent_record_offset":12296,"ancestor_record_offset":552,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":476,"record_offset":10032,"parent_record_offset":10004,"ancestor_record_offset":552,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":476,"record_offset":10004,"parent_record_offset":9960,"ancestor_record_offset":552,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":476,"record_offset":4436,"parent_record_offset":4408,"ancestor_record_offset":552,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":476,"record_offset":4468,"parent_record_offset":4436,"ancestor_record_offset":552,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":476,"record_offset":13252,"parent_record_offset":13100,"ancestor_record_offset":552,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":476,"record_offset":4408,"parent_record_offset":4368,"ancestor_record_offset":552,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":476,"record_offset":6180,"parent_record_offset":6068,"ancestor_record_offset":552,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":476,"record_offset":4532,"parent_record_offset":4368,"ancestor_record_offset":552,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":476,"record_offset":6732,"parent_record_offset":6068,"ancestor_record_offset":552,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":476,"record_offset":6264,"parent_record_offset":6068,"ancestor_record_offset":552,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":476,"record_offset":6124,"parent_record_offset":6068,"ancestor_record_offset":552,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":476,"record_offset":6612,"parent_record_offset":6588,"ancestor_record_offset":552,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":476,"record_offset":5960,"parent_record_offset":5932,"ancestor_record_offset":552,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":476,"record_offset":12684,"parent_record_offset":12660,"ancestor_record_offset":552,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":476,"record_offset":6588,"parent_record_offset":6480,"ancestor_record_offset":552,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":476,"record_offset":4312,"parent_record_offset":4288,"ancestor_record_offset":552,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":476,"record_offset":5368,"parent_record_offset":5344,"ancestor_record_offset":552,"inlinee":9034,"source_line":3365,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":476,"record_offset":5992,"parent_record_offset":5960,"ancestor_record_offset":552,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":4760,"parent_record_offset":4736,"ancestor_record_offset":552,"inlinee":8504,"source_line":435,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":4736,"parent_record_offset":4312,"ancestor_record_offset":552,"inlinee":8503,"source_line":573,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":6528,"parent_record_offset":6504,"ancestor_record_offset":552,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":4588,"parent_record_offset":4368,"ancestor_record_offset":552,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":6824,"parent_record_offset":6068,"ancestor_record_offset":552,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":12792,"parent_record_offset":12732,"ancestor_record_offset":552,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":4368,"parent_record_offset":4340,"ancestor_record_offset":552,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":6504,"parent_record_offset":6480,"ancestor_record_offset":552,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":552,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":6020,"parent_record_offset":5960,"ancestor_record_offset":552,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":12708,"parent_record_offset":12684,"ancestor_record_offset":552,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":12764,"parent_record_offset":12732,"ancestor_record_offset":552,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":552,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":12732,"parent_record_offset":12708,"ancestor_record_offset":552,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":6152,"parent_record_offset":6068,"ancestor_record_offset":552,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":4340,"parent_record_offset":4312,"ancestor_record_offset":552,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":476,"record_offset":6480,"parent_record_offset":6068,"ancestor_record_offset":552,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":476,"record_offset":5344,"parent_record_offset":5320,"ancestor_record_offset":552,"inlinee":11498,"source_line":1798,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":476,"record_offset":2272,"parent_record_offset":2236,"ancestor_record_offset":552,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":476,"record_offset":3108,"parent_record_offset":3076,"ancestor_record_offset":552,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":476,"record_offset":3560,"parent_record_offset":3356,"ancestor_record_offset":552,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":476,"record_offset":2412,"parent_record_offset":2384,"ancestor_record_offset":552,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":476,"record_offset":2484,"parent_record_offset":2344,"ancestor_record_offset":552,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":476,"record_offset":2688,"parent_record_offset":2484,"ancestor_record_offset":552,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":476,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":552,"inlinee":4822,"source_line":1001,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":476,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":552,"inlinee":12072,"source_line":971,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$17::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":476,"record_offset":7248,"parent_record_offset":7224,"ancestor_record_offset":552,"inlinee":12071,"source_line":1846,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$50::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":476,"record_offset":7224,"parent_record_offset":6920,"ancestor_record_offset":552,"inlinee":12069,"source_line":2307,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":476,"record_offset":5932,"parent_record_offset":5900,"ancestor_record_offset":552,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":476,"record_offset":4288,"parent_record_offset":4256,"ancestor_record_offset":552,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$23::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":476,"record_offset":5320,"parent_record_offset":5288,"ancestor_record_offset":552,"inlinee":16842,"source_line":1676,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":476,"record_offset":8276,"parent_record_offset":6920,"ancestor_record_offset":552,"inlinee":11245,"source_line":880,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":476,"record_offset":8380,"parent_record_offset":8356,"ancestor_record_offset":552,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":476,"record_offset":8356,"parent_record_offset":8332,"ancestor_record_offset":552,"inlinee":11739,"source_line":910,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":476,"record_offset":8200,"parent_record_offset":8168,"ancestor_record_offset":552,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":476,"record_offset":2440,"parent_record_offset":2412,"ancestor_record_offset":552,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":476,"record_offset":2344,"parent_record_offset":2308,"ancestor_record_offset":552,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":476,"record_offset":2308,"parent_record_offset":2272,"ancestor_record_offset":552,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::flag::stair_banner_attachment::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/flag.rs
//     evidence={"module":476,"record_offset":7136,"parent_record_offset":7052,"ancestor_record_offset":552,"inlinee":27663,"source_line":41,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::flag::stair_banner_attachment::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/flag.rs
//     evidence={"module":476,"record_offset":7712,"parent_record_offset":7628,"ancestor_record_offset":552,"inlinee":27664,"source_line":49,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":476,"record_offset":14500,"parent_record_offset":14460,"ancestor_record_offset":552,"inlinee":27090,"source_line":39,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":476,"record_offset":14524,"parent_record_offset":14500,"ancestor_record_offset":552,"inlinee":27092,"source_line":58,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":476,"record_offset":12108,"parent_record_offset":12072,"ancestor_record_offset":552,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":476,"record_offset":7868,"parent_record_offset":6920,"ancestor_record_offset":552,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":476,"record_offset":8004,"parent_record_offset":6920,"ancestor_record_offset":552,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":476,"record_offset":14420,"parent_record_offset":14368,"ancestor_record_offset":552,"inlinee":11330,"source_line":79,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":476,"record_offset":14392,"parent_record_offset":14368,"ancestor_record_offset":552,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag::stair_banner_attachment
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6C0D
// 首条位置: {"module":476,"inlinee":27661}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":7456,"parent_record_offset":6920,"ancestor_record_offset":6920,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":476,"record_offset":7400,"parent_record_offset":6920,"ancestor_record_offset":6920,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":476,"record_offset":7744,"parent_record_offset":6920,"ancestor_record_offset":6920,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::all
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":476,"record_offset":7628,"parent_record_offset":6920,"ancestor_record_offset":6920,"inlinee":11954,"source_line":309,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":476,"record_offset":7052,"parent_record_offset":6920,"ancestor_record_offset":6920,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":476,"record_offset":7336,"parent_record_offset":6920,"ancestor_record_offset":6920,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_rotation_y
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":476,"record_offset":8304,"parent_record_offset":6920,"ancestor_record_offset":6920,"inlinee":11736,"source_line":178,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":476,"record_offset":8060,"parent_record_offset":6920,"ancestor_record_offset":6920,"inlinee":9810,"source_line":581,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$50::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":476,"record_offset":7224,"parent_record_offset":6920,"ancestor_record_offset":6920,"inlinee":12069,"source_line":2307,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":476,"record_offset":8276,"parent_record_offset":6920,"ancestor_record_offset":6920,"inlinee":11245,"source_line":880,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":476,"record_offset":7868,"parent_record_offset":6920,"ancestor_record_offset":6920,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":476,"record_offset":8004,"parent_record_offset":6920,"ancestor_record_offset":6920,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":476,"record_offset":8224,"parent_record_offset":8200,"ancestor_record_offset":6920,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":476,"record_offset":7560,"parent_record_offset":7520,"ancestor_record_offset":6920,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":476,"record_offset":7104,"parent_record_offset":7080,"ancestor_record_offset":6920,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":476,"record_offset":7588,"parent_record_offset":7520,"ancestor_record_offset":6920,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":476,"record_offset":7488,"parent_record_offset":7456,"ancestor_record_offset":6920,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":476,"record_offset":7520,"parent_record_offset":7488,"ancestor_record_offset":6920,"inlinee":11671,"source_line":696,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":476,"record_offset":7828,"parent_record_offset":7800,"ancestor_record_offset":6920,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":476,"record_offset":7080,"parent_record_offset":7052,"ancestor_record_offset":6920,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_assembly_params::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_assembly_params.rs
//     evidence={"module":476,"record_offset":7424,"parent_record_offset":7400,"ancestor_record_offset":6920,"inlinee":12092,"source_line":110,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":476,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":6920,"inlinee":8701,"source_line":38,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":476,"record_offset":7184,"parent_record_offset":7160,"ancestor_record_offset":6920,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":476,"record_offset":8332,"parent_record_offset":8304,"ancestor_record_offset":6920,"inlinee":11738,"source_line":182,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":476,"record_offset":8420,"parent_record_offset":8304,"ancestor_record_offset":6920,"inlinee":11856,"source_line":81,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":476,"record_offset":8168,"parent_record_offset":8136,"ancestor_record_offset":6920,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":476,"record_offset":8136,"parent_record_offset":8112,"ancestor_record_offset":6920,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":476,"record_offset":8112,"parent_record_offset":8084,"ancestor_record_offset":6920,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":476,"record_offset":8084,"parent_record_offset":8060,"ancestor_record_offset":6920,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":476,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":6920,"inlinee":4822,"source_line":1001,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":476,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":6920,"inlinee":12072,"source_line":971,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$17::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":476,"record_offset":7248,"parent_record_offset":7224,"ancestor_record_offset":6920,"inlinee":12071,"source_line":1846,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":476,"record_offset":8380,"parent_record_offset":8356,"ancestor_record_offset":6920,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":476,"record_offset":8356,"parent_record_offset":8332,"ancestor_record_offset":6920,"inlinee":11739,"source_line":910,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":476,"record_offset":8200,"parent_record_offset":8168,"ancestor_record_offset":6920,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::flag::stair_banner_attachment::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/flag.rs
//     evidence={"module":476,"record_offset":7136,"parent_record_offset":7052,"ancestor_record_offset":6920,"inlinee":27663,"source_line":41,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::flag::stair_banner_attachment::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/flag.rs
//     evidence={"module":476,"record_offset":7712,"parent_record_offset":7628,"ancestor_record_offset":6920,"inlinee":27664,"source_line":49,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag::stair_banner_attachment::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6C0F
// 首条位置: {"module":476,"inlinee":27663}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":476,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":7136,"inlinee":8701,"source_line":38,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":476,"record_offset":7184,"parent_record_offset":7160,"ancestor_record_offset":7136,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/00476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag::stair_banner_attachment::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6C10
// 首条位置: {"module":476,"inlinee":27664}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::flag::stair_banner_attachment::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13A11
// 首条位置: {"module":1557,"inlinee":80401}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1557,"record_offset":668,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1557,"record_offset":572,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1557,"record_offset":840,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01557.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1557,"record_offset":776,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1557,"record_offset":628,"parent_record_offset":600,"ancestor_record_offset":488,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1557,"record_offset":600,"parent_record_offset":572,"ancestor_record_offset":488,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1557,"record_offset":804,"parent_record_offset":776,"ancestor_record_offset":488,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1557,"record_offset":700,"parent_record_offset":668,"ancestor_record_offset":488,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1557,"record_offset":732,"parent_record_offset":700,"ancestor_record_offset":488,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01557.jsonl.gz"}
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
// 完整结构及原始类型记录: flag.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: flag.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: flag.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag::generate_flags
// 信息 ID: 3e5b418ad73af3f439f3d727
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 28e4c499a79488e0d373583e: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)> > @ 0x290b41 -> 0x8cea00 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)> @ 0x43ac38 -> 0x8cea00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag::stair_banner_attachment
// 信息 ID: df8b384c2e233f8164b7a1d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=476, inline_record=6920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag::stair_banner_attachment::closure$0
// 信息 ID: 8aef598823b7276ec46e51d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::flag::stair_banner_attachment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=476, inline_record=7136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag::stair_banner_attachment::closure$1
// 信息 ID: 2370f95e4b12f75073d055bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::flag::stair_banner_attachment", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::flag::generate_flags; module=476, inline_record=7712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag::stair_banner_attachment::closure$2
// 信息 ID: d751a6ffc1b6c198c08535a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::decorator_visual::flag::stair_banner_attachment", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::flag::stair_banner_attachment::closure_env$2 <- core::array::from_fn<f32,2,system_decorator::decorator_visual::flag::stair_banner_attachment::closure_env$2>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::flag::stair_banner_attachment::closure_env$2 <- core::ops::try_trait::impl$1::call_mut<f32,usize,system_decorator::decorator_visual::flag::stair_banner_attachment::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::try_trait::impl$1::call_mut<f32,usize,system_decorator::decorator_visual::flag::stair_banner_attachment::closure_env$2>; module=1557, inline_record=488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
