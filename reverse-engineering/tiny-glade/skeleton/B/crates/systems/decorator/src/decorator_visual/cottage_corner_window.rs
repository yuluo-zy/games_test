// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/decorator_visual/cottage_corner_window.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 6dd9807078ddca871a72182e30bcc7584d54240530df0e01db1089b7a67049c4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cottage_corner_window.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 434, 442, 445
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":445,"record_offset":1924,"rva":8349360,"va":5377058480}
// 编译实例 RVAs: 0x7f66b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2382972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e884a3dbff","size":5,"soft_fail":false,"target_va":5374676688,"va":5377059655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1978526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e862cfe1ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377060009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1983317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8abbce1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377064816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=4
//     target_aliases=14 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t246943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e89fc40300","size":5,"soft_fail":false,"target_va":5377307216,"va":5377060268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408393b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t268661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e875190400","size":5,"soft_fail":false,"target_va":5377332144,"va":5377063478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bc520
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t808021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e855540c00","size":5,"soft_fail":false,"target_va":5377869088,"va":5377061062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1b100
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2246520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e878472200","size":5,"soft_fail":false,"target_va":5379305728,"va":5377059203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3337934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8ceee3200","size":5,"soft_fail":false,"target_va":5380398224,"va":5377060285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b26a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3342597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e805013300","size":5,"soft_fail":false,"target_va":5380401776,"va":5377059174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3553597","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e83d393600","size":5,"soft_fail":false,"target_va":5380621856,"va":5377068254,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91eb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4827278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e88ea84900","size":5,"soft_fail":false,"target_va":5381889712,"va":5377062429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4874067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8535f4a00","size":5,"soft_fail":false,"target_va":5381934528,"va":5377060456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4874118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8865f4a00","size":5,"soft_fail":false,"target_va":5381934560,"va":5377060437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f7dc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10488030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8de08a000","size":5,"soft_fail":false,"target_va":5387550144,"va":5377062109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125cf90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10903987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8b361a600","size":5,"soft_fail":false,"target_va":5387964304,"va":5377060312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14128ca70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_corner_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11094266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8fa48a900","size":5,"soft_fail":false,"target_va":5388159600,"va":5377065329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34474265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e819090e02","size":5,"soft_fail":false,"target_va":5411540992,"va":5377066722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t214276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e804450300","size":5,"soft_fail":false,"target_va":5377274992,"va":5377060711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34474551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8370a0e02","size":5,"soft_fail":false,"target_va":5411540963,"va":5377066407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34475339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e84b0d0e02","size":5,"soft_fail":false,"target_va":5411541584,"va":5377066240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34475872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8600f0e02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377065899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t34475836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e83c0f0e02","size":5,"soft_fail":false,"target_va":5411542064,"va":5377066223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t34475887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e86f0f0e02","size":5,"soft_fail":false,"target_va":5411541904,"va":5377066012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8ddfaffff","size":5,"soft_fail":false,"target_va":5377057328,"va":5377058638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34484640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8a0310e02","size":5,"soft_fail":false,"target_va":5411543360,"va":5377058715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2244008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8a83d2200","size":5,"soft_fail":false,"target_va":5379303968,"va":5377059955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t242953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e809b50300","size":5,"soft_fail":false,"target_va":5377307232,"va":5377064274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::iter_modified
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2246143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8ff452200","size":5,"soft_fail":false,"target_va":5379305584,"va":5377059436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3341668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e864fd3200","size":5,"soft_fail":false,"target_va":5380401520,"va":5377059847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3357351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8a73a3300","size":5,"soft_fail":false,"target_va":5380417424,"va":5377060068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3354952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e848313300","size":5,"soft_fail":false,"target_va":5380417344,"va":5377062387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2732735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8bfb22900","size":5,"soft_fail":false,"target_va":5379792624,"va":5377059884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3336622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8aee93200","size":5,"soft_fail":false,"target_va":5380396784,"va":5377060157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4326389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8f5034200","size":5,"soft_fail":false,"target_va":5381385536,"va":5377059142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4331578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e83a184200","size":5,"soft_fail":false,"target_va":5381390640,"va":5377059057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4320696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8b8ed4100","size":5,"soft_fail":false,"target_va":5381386480,"va":5377065779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4326091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8cb024200","size":5,"soft_fail":false,"target_va":5381384704,"va":5377058608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34578646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8d6a00f02","size":5,"soft_fail":false,"target_va":5411644800,"va":5377066149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34470900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8f4fb0d02","size":5,"soft_fail":false,"target_va":5411537104,"va":5377066199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t186588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8dcd80200","size":5,"soft_fail":false,"target_va":5377249968,"va":5377063375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t190400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8c0e70200","size":5,"soft_fail":false,"target_va":5377251504,"va":5377061099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_wall_window::add_lintels
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-33961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8577bffff","size":5,"soft_fail":false,"target_va":5377030544,"va":5377064500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-34933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e88b77ffff","size":5,"soft_fail":false,"target_va":5377030064,"va":5377064992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4862951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8e7334a00","size":5,"soft_fail":false,"target_va":5381923136,"va":5377060180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4863130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e89a344a00","size":5,"soft_fail":false,"target_va":5381923344,"va":5377060209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4842163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e8b3e24900","size":5,"soft_fail":false,"target_va":5381907008,"va":5377064840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4827655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e807aa4900","size":5,"soft_fail":false,"target_va":5381890192,"va":5377062532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation_bad
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4825936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377058480,"raw_hex":"e850a34900","size":5,"soft_fail":false,"target_va":5381886416,"va":5377060475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":9724,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":7460,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":445,"record_offset":10276,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":445,"record_offset":10332,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":445,"record_offset":15496,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":445,"record_offset":4720,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":445,"record_offset":4692,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":445,"record_offset":7424,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":16084,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":445,"record_offset":8972,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25099,"source_line":291,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":445,"record_offset":10848,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24795,"source_line":298,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":445,"record_offset":12268,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24802,"source_line":309,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":445,"record_offset":4636,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24514,"source_line":273,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":445,"record_offset":13644,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":445,"record_offset":15352,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25181,"source_line":237,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":445,"record_offset":15800,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":7382,"source_line":237,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":445,"record_offset":10360,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":9360,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":9696,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":9136,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":9192,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":445,"record_offset":15264,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":445,"record_offset":3652,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":445,"record_offset":2228,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":445,"record_offset":2676,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::add_cloth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":445,"record_offset":14388,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25395,"source_line":250,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":445,"record_offset":10724,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24805,"source_line":262,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_corner_window::get_corner_info
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_corner_window.rs
//     evidence={"module":445,"record_offset":7856,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25425,"source_line":300,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_corner_window::has_cloth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_corner_window.rs
//     evidence={"module":445,"record_offset":14332,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25180,"source_line":348,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":445,"record_offset":10500,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24808,"source_line":84,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":445,"record_offset":13700,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":25100,"source_line":170,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":445,"record_offset":10472,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":24807,"source_line":54,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":445,"record_offset":14084,"parent_record_offset":1924,"ancestor_record_offset":1924,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":7776,"parent_record_offset":7752,"ancestor_record_offset":1924,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":7800,"parent_record_offset":7776,"ancestor_record_offset":1924,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":14712,"parent_record_offset":14688,"ancestor_record_offset":1924,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":14460,"parent_record_offset":14424,"ancestor_record_offset":1924,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":7752,"parent_record_offset":7576,"ancestor_record_offset":1924,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":14688,"parent_record_offset":14664,"ancestor_record_offset":1924,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":445,"record_offset":14424,"parent_record_offset":14388,"ancestor_record_offset":1924,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":445,"record_offset":14664,"parent_record_offset":14640,"ancestor_record_offset":1924,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":445,"record_offset":9796,"parent_record_offset":9764,"ancestor_record_offset":1924,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":445,"record_offset":7516,"parent_record_offset":7488,"ancestor_record_offset":1924,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":445,"record_offset":16184,"parent_record_offset":16156,"ancestor_record_offset":1924,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":445,"record_offset":16156,"parent_record_offset":16132,"ancestor_record_offset":1924,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":445,"record_offset":9844,"parent_record_offset":9820,"ancestor_record_offset":1924,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":445,"record_offset":9820,"parent_record_offset":9796,"ancestor_record_offset":1924,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":445,"record_offset":7576,"parent_record_offset":7544,"ancestor_record_offset":1924,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":445,"record_offset":7544,"parent_record_offset":7516,"ancestor_record_offset":1924,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":445,"record_offset":16132,"parent_record_offset":16108,"ancestor_record_offset":1924,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":445,"record_offset":7612,"parent_record_offset":7576,"ancestor_record_offset":1924,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":445,"record_offset":12088,"parent_record_offset":10888,"ancestor_record_offset":1924,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":445,"record_offset":18940,"parent_record_offset":18916,"ancestor_record_offset":1924,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":9764,"parent_record_offset":9724,"ancestor_record_offset":1924,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":445,"record_offset":7488,"parent_record_offset":7460,"ancestor_record_offset":1924,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":445,"record_offset":10300,"parent_record_offset":10276,"ancestor_record_offset":1924,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":445,"record_offset":7636,"parent_record_offset":7612,"ancestor_record_offset":1924,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":445,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":1924,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":445,"record_offset":3988,"parent_record_offset":3960,"ancestor_record_offset":1924,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":445,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":1924,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":445,"record_offset":4016,"parent_record_offset":3988,"ancestor_record_offset":1924,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":445,"record_offset":3936,"parent_record_offset":3908,"ancestor_record_offset":1924,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":445,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":1924,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":445,"record_offset":4132,"parent_record_offset":3960,"ancestor_record_offset":1924,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":445,"record_offset":4244,"parent_record_offset":4220,"ancestor_record_offset":1924,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":445,"record_offset":7136,"parent_record_offset":7112,"ancestor_record_offset":1924,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":445,"record_offset":4660,"parent_record_offset":4636,"ancestor_record_offset":1924,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":445,"record_offset":5128,"parent_record_offset":5104,"ancestor_record_offset":1924,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":445,"record_offset":6644,"parent_record_offset":6620,"ancestor_record_offset":1924,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":445,"record_offset":5224,"parent_record_offset":5188,"ancestor_record_offset":1924,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":445,"record_offset":15200,"parent_record_offset":14784,"ancestor_record_offset":1924,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":445,"record_offset":15108,"parent_record_offset":15052,"ancestor_record_offset":1924,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":445,"record_offset":10432,"parent_record_offset":10408,"ancestor_record_offset":1924,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":445,"record_offset":14936,"parent_record_offset":14820,"ancestor_record_offset":1924,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":445,"record_offset":14908,"parent_record_offset":14820,"ancestor_record_offset":1924,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":445,"record_offset":10816,"parent_record_offset":10724,"ancestor_record_offset":1924,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":445,"record_offset":9092,"parent_record_offset":9068,"ancestor_record_offset":1924,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":445,"record_offset":6668,"parent_record_offset":6644,"ancestor_record_offset":1924,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":445,"record_offset":7204,"parent_record_offset":6508,"ancestor_record_offset":1924,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":445,"record_offset":4064,"parent_record_offset":4040,"ancestor_record_offset":1924,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":1924,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":7708,"parent_record_offset":7684,"ancestor_record_offset":1924,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":16212,"parent_record_offset":16184,"ancestor_record_offset":1924,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":445,"record_offset":2376,"parent_record_offset":2344,"ancestor_record_offset":1924,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":14356,"parent_record_offset":14332,"ancestor_record_offset":1924,"inlinee":9282,"source_line":1980,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":15452,"parent_record_offset":15424,"ancestor_record_offset":1924,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":15960,"parent_record_offset":15824,"ancestor_record_offset":1924,"inlinee":6324,"source_line":2065,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":445,"record_offset":5032,"parent_record_offset":5004,"ancestor_record_offset":1924,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":445,"record_offset":5004,"parent_record_offset":4976,"ancestor_record_offset":1924,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":445,"record_offset":9216,"parent_record_offset":9192,"ancestor_record_offset":1924,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":445,"record_offset":8512,"parent_record_offset":8488,"ancestor_record_offset":1924,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":445,"record_offset":15292,"parent_record_offset":15264,"ancestor_record_offset":1924,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":445,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":1924,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":445,"record_offset":3804,"parent_record_offset":3760,"ancestor_record_offset":1924,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":445,"record_offset":5440,"parent_record_offset":5416,"ancestor_record_offset":1924,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":445,"record_offset":10756,"parent_record_offset":10724,"ancestor_record_offset":1924,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":445,"record_offset":10780,"parent_record_offset":10756,"ancestor_record_offset":1924,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":445,"record_offset":5416,"parent_record_offset":4720,"ancestor_record_offset":1924,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":445,"record_offset":7380,"parent_record_offset":6324,"ancestor_record_offset":1924,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":445,"record_offset":9036,"parent_record_offset":9008,"ancestor_record_offset":1924,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":445,"record_offset":2624,"parent_record_offset":2596,"ancestor_record_offset":1924,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":445,"record_offset":4592,"parent_record_offset":3760,"ancestor_record_offset":1924,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":4196,"parent_record_offset":3908,"ancestor_record_offset":1924,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":445,"record_offset":5284,"parent_record_offset":5260,"ancestor_record_offset":1924,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":1924,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":3064,"parent_record_offset":3040,"ancestor_record_offset":1924,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":445,"record_offset":3272,"parent_record_offset":3244,"ancestor_record_offset":1924,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":14568,"parent_record_offset":14540,"ancestor_record_offset":1924,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":1924,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":1924,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":3040,"parent_record_offset":3016,"ancestor_record_offset":1924,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":3344,"parent_record_offset":3316,"ancestor_record_offset":1924,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":15524,"parent_record_offset":15496,"ancestor_record_offset":1924,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":2992,"parent_record_offset":2876,"ancestor_record_offset":1924,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":445,"record_offset":2344,"parent_record_offset":2316,"ancestor_record_offset":1924,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":445,"record_offset":2704,"parent_record_offset":2676,"ancestor_record_offset":1924,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":445,"record_offset":2844,"parent_record_offset":2812,"ancestor_record_offset":1924,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":445,"record_offset":2812,"parent_record_offset":2776,"ancestor_record_offset":1924,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":445,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":1924,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":445,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":1924,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":445,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":1924,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":445,"record_offset":3316,"parent_record_offset":2776,"ancestor_record_offset":1924,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":445,"record_offset":2316,"parent_record_offset":2284,"ancestor_record_offset":1924,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":445,"record_offset":2284,"parent_record_offset":2260,"ancestor_record_offset":1924,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":445,"record_offset":2476,"parent_record_offset":2444,"ancestor_record_offset":1924,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":445,"record_offset":2572,"parent_record_offset":2476,"ancestor_record_offset":1924,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":445,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":1924,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":445,"record_offset":2260,"parent_record_offset":2228,"ancestor_record_offset":1924,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":445,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":1924,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":445,"record_offset":2876,"parent_record_offset":2844,"ancestor_record_offset":1924,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":445,"record_offset":3160,"parent_record_offset":2876,"ancestor_record_offset":1924,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":445,"record_offset":3132,"parent_record_offset":2876,"ancestor_record_offset":1924,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":445,"record_offset":2936,"parent_record_offset":2876,"ancestor_record_offset":1924,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":12116,"parent_record_offset":12088,"ancestor_record_offset":1924,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":18976,"parent_record_offset":18940,"ancestor_record_offset":1924,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":12140,"parent_record_offset":12116,"ancestor_record_offset":1924,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":19000,"parent_record_offset":18976,"ancestor_record_offset":1924,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":19032,"parent_record_offset":18940,"ancestor_record_offset":1924,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":445,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":1924,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$1::iter_modified::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":445,"record_offset":6324,"parent_record_offset":6292,"ancestor_record_offset":1924,"inlinee":24794,"source_line":77,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":445,"record_offset":10888,"parent_record_offset":10848,"ancestor_record_offset":1924,"inlinee":24797,"source_line":249,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":445,"record_offset":9008,"parent_record_offset":8972,"ancestor_record_offset":1924,"inlinee":25059,"source_line":223,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":445,"record_offset":12308,"parent_record_offset":12268,"ancestor_record_offset":1924,"inlinee":24803,"source_line":258,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$22::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":445,"record_offset":11860,"parent_record_offset":11832,"ancestor_record_offset":1924,"inlinee":24799,"source_line":94,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":445,"record_offset":12200,"parent_record_offset":12176,"ancestor_record_offset":1924,"inlinee":24813,"source_line":252,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":445,"record_offset":9068,"parent_record_offset":9036,"ancestor_record_offset":1924,"inlinee":25042,"source_line":226,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":445,"record_offset":13576,"parent_record_offset":13552,"ancestor_record_offset":1924,"inlinee":24815,"source_line":261,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":445,"record_offset":15376,"parent_record_offset":15352,"ancestor_record_offset":1924,"inlinee":25182,"source_line":143,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":445,"record_offset":15824,"parent_record_offset":15800,"ancestor_record_offset":1924,"inlinee":7383,"source_line":161,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":445,"record_offset":15400,"parent_record_offset":15376,"ancestor_record_offset":1924,"inlinee":25183,"source_line":122,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":445,"record_offset":15424,"parent_record_offset":15400,"ancestor_record_offset":1924,"inlinee":7384,"source_line":128,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::mul_high_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":445,"record_offset":15932,"parent_record_offset":15824,"ancestor_record_offset":1924,"inlinee":7386,"source_line":214,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":445,"record_offset":10384,"parent_record_offset":10360,"ancestor_record_offset":1924,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":445,"record_offset":10408,"parent_record_offset":10384,"ancestor_record_offset":1924,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":8196,"parent_record_offset":8164,"ancestor_record_offset":1924,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":1924,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":8140,"parent_record_offset":8112,"ancestor_record_offset":1924,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":8940,"parent_record_offset":7856,"ancestor_record_offset":1924,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::try_normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":8112,"parent_record_offset":7856,"ancestor_record_offset":1924,"inlinee":25426,"source_line":481,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":8488,"parent_record_offset":8328,"ancestor_record_offset":1924,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":9468,"parent_record_offset":9360,"ancestor_record_offset":1924,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":8056,"parent_record_offset":7856,"ancestor_record_offset":1924,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":445,"record_offset":15052,"parent_record_offset":15028,"ancestor_record_offset":1924,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":445,"record_offset":15028,"parent_record_offset":14784,"ancestor_record_offset":1924,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":445,"record_offset":4940,"parent_record_offset":4904,"ancestor_record_offset":1924,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":445,"record_offset":4976,"parent_record_offset":4940,"ancestor_record_offset":1924,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":445,"record_offset":5072,"parent_record_offset":4904,"ancestor_record_offset":1924,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":445,"record_offset":4904,"parent_record_offset":4852,"ancestor_record_offset":1924,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":445,"record_offset":6620,"parent_record_offset":6508,"ancestor_record_offset":1924,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":445,"record_offset":5104,"parent_record_offset":4852,"ancestor_record_offset":1924,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":445,"record_offset":7232,"parent_record_offset":6508,"ancestor_record_offset":1924,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":445,"record_offset":5188,"parent_record_offset":5160,"ancestor_record_offset":1924,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":445,"record_offset":5160,"parent_record_offset":4852,"ancestor_record_offset":1924,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":445,"record_offset":6704,"parent_record_offset":6508,"ancestor_record_offset":1924,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":445,"record_offset":6592,"parent_record_offset":6508,"ancestor_record_offset":1924,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":445,"record_offset":7112,"parent_record_offset":7088,"ancestor_record_offset":1924,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":445,"record_offset":6376,"parent_record_offset":6352,"ancestor_record_offset":1924,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":445,"record_offset":6400,"parent_record_offset":6376,"ancestor_record_offset":1924,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":445,"record_offset":7088,"parent_record_offset":6920,"ancestor_record_offset":1924,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":445,"record_offset":4776,"parent_record_offset":4748,"ancestor_record_offset":1924,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":445,"record_offset":5584,"parent_record_offset":5560,"ancestor_record_offset":1924,"inlinee":24792,"source_line":3317,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":445,"record_offset":6432,"parent_record_offset":6400,"ancestor_record_offset":1924,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":445,"record_offset":7028,"parent_record_offset":6944,"ancestor_record_offset":1924,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":445,"record_offset":6972,"parent_record_offset":6944,"ancestor_record_offset":1924,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":445,"record_offset":5260,"parent_record_offset":4852,"ancestor_record_offset":1924,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":445,"record_offset":7324,"parent_record_offset":6508,"ancestor_record_offset":1924,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":445,"record_offset":4852,"parent_record_offset":4812,"ancestor_record_offset":1924,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":445,"record_offset":6944,"parent_record_offset":6920,"ancestor_record_offset":1924,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":445,"record_offset":6484,"parent_record_offset":6460,"ancestor_record_offset":1924,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":445,"record_offset":6460,"parent_record_offset":6400,"ancestor_record_offset":1924,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":445,"record_offset":6508,"parent_record_offset":6484,"ancestor_record_offset":1924,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":445,"record_offset":6564,"parent_record_offset":6508,"ancestor_record_offset":1924,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":445,"record_offset":4812,"parent_record_offset":4776,"ancestor_record_offset":1924,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":445,"record_offset":6920,"parent_record_offset":6508,"ancestor_record_offset":1924,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":445,"record_offset":5560,"parent_record_offset":5536,"ancestor_record_offset":1924,"inlinee":24790,"source_line":1801,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":445,"record_offset":3688,"parent_record_offset":3652,"ancestor_record_offset":1924,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":445,"record_offset":16616,"parent_record_offset":16584,"ancestor_record_offset":1924,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":445,"record_offset":17132,"parent_record_offset":16872,"ancestor_record_offset":1924,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":445,"record_offset":3832,"parent_record_offset":3804,"ancestor_record_offset":1924,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":445,"record_offset":3908,"parent_record_offset":3760,"ancestor_record_offset":1924,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":445,"record_offset":4168,"parent_record_offset":3908,"ancestor_record_offset":1924,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":445,"record_offset":6352,"parent_record_offset":6324,"ancestor_record_offset":1924,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":445,"record_offset":4748,"parent_record_offset":4720,"ancestor_record_offset":1924,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":445,"record_offset":5536,"parent_record_offset":5504,"ancestor_record_offset":1924,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":445,"record_offset":8228,"parent_record_offset":8196,"ancestor_record_offset":1924,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":445,"record_offset":3860,"parent_record_offset":3832,"ancestor_record_offset":1924,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":445,"record_offset":3760,"parent_record_offset":3724,"ancestor_record_offset":1924,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":445,"record_offset":3724,"parent_record_offset":3688,"ancestor_record_offset":1924,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::offset_x_clamped
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":445,"record_offset":8000,"parent_record_offset":7856,"ancestor_record_offset":1924,"inlinee":25386,"source_line":221,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_corner_window::get_corner_info
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 300
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6351
// 首条位置: {"module":445,"inlinee":25425}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":445,"record_offset":7964,"parent_record_offset":7856,"ancestor_record_offset":7856,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":8940,"parent_record_offset":7856,"ancestor_record_offset":7856,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::try_normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":8112,"parent_record_offset":7856,"ancestor_record_offset":7856,"inlinee":25426,"source_line":481,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":8056,"parent_record_offset":7856,"ancestor_record_offset":7856,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::offset_x_clamped
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":445,"record_offset":8000,"parent_record_offset":7856,"ancestor_record_offset":7856,"inlinee":25386,"source_line":221,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":445,"record_offset":8512,"parent_record_offset":8488,"ancestor_record_offset":7856,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":445,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":7856,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":8196,"parent_record_offset":8164,"ancestor_record_offset":7856,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":7856,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":8140,"parent_record_offset":8112,"ancestor_record_offset":7856,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":8488,"parent_record_offset":8328,"ancestor_record_offset":7856,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":445,"record_offset":8228,"parent_record_offset":8196,"ancestor_record_offset":7856,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_corner_window::has_cloth
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 348
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x625C
// 首条位置: {"module":434,"inlinee":25180}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":434,"record_offset":19232,"parent_record_offset":19208,"ancestor_record_offset":19208,"inlinee":9282,"source_line":1980,"archive":"_pdb/B/modules/00434.jsonl.gz"}
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
// 完整结构及原始类型记录: cottage_corner_window.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cottage_corner_window.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: cottage_corner_window.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows
// 信息 ID: d7b4e1dcbc0c8d6b8d950986
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 99c78b7d2b54b0f43bad25b1: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)> > @ 0x402aa -> 0x7f66b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>)> > > @ 0x46965a -> 0x7f66b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_corner_window::get_corner_info
// 信息 ID: dca8bd4180e0d7fd3005b4fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows; module=445, inline_record=7856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_corner_window::has_cloth
// 信息 ID: 263beea5785b6d8addd45229
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=434, inline_record=19208; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows; module=442, inline_record=19076; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows; module=445, inline_record=14332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
