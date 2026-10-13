// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 4fe1ca37fadf9b785756b67190af5a9ce92aadfbaf77237ae2737b12ea78a248
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: find_overlap_segments.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1415, 1417, 1444, 1452, 1472, 1476, 3024, 4061, 4067, 4068
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
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
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1476,"record_offset":5800,"rva":19170848,"va":5387879968}
// 编译实例 RVAs: 0x1248620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=8
//     target_aliases=12 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13074612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e84c7f38ff","size":5,"soft_fail":false,"target_va":5374813424,"va":5387888031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12800457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e837ae3cff","size":5,"soft_fail":false,"target_va":5375081488,"va":5387881940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12801720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e848a93cff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387883219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ad2950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7824967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8b99988ff","size":5,"soft_fail":false,"target_va":5380057424,"va":5387882386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8750
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-330015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8e1f6faff","size":5,"soft_fail":false,"target_va":5387552592,"va":5387882602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8c10
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-328483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8ddfcfaff","size":5,"soft_fail":false,"target_va":5387553808,"va":5387882286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411fd940
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-313796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e83c36fbff","size":5,"soft_fail":false,"target_va":5387573568,"va":5387887359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412042a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-281037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e833b6fbff","size":5,"soft_fail":false,"target_va":5387600544,"va":5387881576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14122cb50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-116717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e81338feff","size":5,"soft_fail":false,"target_va":5387766608,"va":5387883320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141247300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e869d8ffff","size":5,"soft_fail":false,"target_va":5387875072,"va":5387885202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23964370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8d2aa6d01","size":5,"soft_fail":false,"target_va":5411847168,"va":5387882793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23966106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e89ab16d01","size":5,"soft_fail":false,"target_va":5411847952,"va":5387881841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-13526002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e80e9c31ff","size":5,"soft_fail":false,"target_va":5374354992,"va":5387880989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23654200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e838ef6801","size":5,"soft_fail":false,"target_va":5411540963,"va":5387886758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23654913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e801f26801","size":5,"soft_fail":false,"target_va":5411541584,"va":5387886666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23655062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e896f26801","size":5,"soft_fail":false,"target_va":5411541776,"va":5387886709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t23655523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e863f46801","size":5,"soft_fail":false,"target_va":5411542064,"va":5387886536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23655613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8bdf46801","size":5,"soft_fail":false,"target_va":5411542296,"va":5387886678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23655392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8e0f36801","size":5,"soft_fail":false,"target_va":5411541904,"va":5387886507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-105854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e88262feff","size":5,"soft_fail":false,"target_va":5387777488,"va":5387883337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e802f8ffff","size":5,"soft_fail":false,"target_va":5387878096,"va":5387880137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23663141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e825126901","size":5,"soft_fail":false,"target_va":5411543360,"va":5387880214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::gable_sides
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9855003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8e59f69ff","size":5,"soft_fail":false,"target_va":5378028512,"va":5387883510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9853903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e831a469ff","size":5,"soft_fail":false,"target_va":5378029600,"va":5387883498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7466747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e805118eff","size":5,"soft_fail":false,"target_va":5380416976,"va":5387883718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7464284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8a41a8eff","size":5,"soft_fail":false,"target_va":5380417056,"va":5387881335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8088666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8a69384ff","size":5,"soft_fail":false,"target_va":5379792624,"va":5387881285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::utils_intersect::line_x_shape_overlap
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5314738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8b2185100","size":5,"soft_fail":false,"target_va":5393198496,"va":5387883753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6495115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e875e49cff","size":5,"soft_fail":false,"target_va":5381385536,"va":5387880646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6489926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8baf89cff","size":5,"soft_fail":false,"target_va":5381390640,"va":5387880561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6499758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e852d29cff","size":5,"soft_fail":false,"target_va":5381386480,"va":5387886233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6495409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e84fe39cff","size":5,"soft_fail":false,"target_va":5381384704,"va":5387880108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23754407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8a7766a01","size":5,"soft_fail":false,"target_va":5411640768,"va":5387886356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-322718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e86213fbff","size":5,"soft_fail":false,"target_va":5387564432,"va":5387887145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23650693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e885e16801","size":5,"soft_fail":false,"target_va":5411537104,"va":5387886406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve2
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5967675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8c5f0a4ff","size":5,"soft_fail":false,"target_va":5381913840,"va":5387881510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::contains_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5969891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e81de8a4ff","size":5,"soft_fail":false,"target_va":5381914864,"va":5387884750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5972974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e812dca4ff","size":5,"soft_fail":false,"target_va":5381912944,"va":5387885913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve2
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5963540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8ec00a5ff","size":5,"soft_fail":false,"target_va":5381917936,"va":5387881471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5960263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8b90da5ff","size":5,"soft_fail":false,"target_va":5381924400,"va":5387884658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5963371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e89501a5ff","size":5,"soft_fail":false,"target_va":5381920256,"va":5387883622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5961239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8e909a5ff","size":5,"soft_fail":false,"target_va":5381920464,"va":5387881698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5963739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e82500a5ff","size":5,"soft_fail":false,"target_va":5381920048,"va":5387883782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5960908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879968,"raw_hex":"e8340ba5ff","size":5,"soft_fail":false,"target_va":5381924896,"va":5387885799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1476,"record_offset":10756,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":8544,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":8932,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":9940,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":16528,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1476,"record_offset":9996,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1476,"record_offset":16968,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1476,"record_offset":14116,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1476,"record_offset":20700,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1476,"record_offset":10640,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":13250,"source_line":1251,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1476,"record_offset":18656,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1476,"record_offset":12652,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1476,"record_offset":10372,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1476,"record_offset":10176,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1476,"record_offset":10112,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1476,"record_offset":15476,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1476,"record_offset":10288,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1476,"record_offset":16788,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1476,"record_offset":9792,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1476,"record_offset":10148,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1476,"record_offset":20728,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$10::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1476,"record_offset":12880,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":14447,"source_line":276,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1476,"record_offset":7560,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1476,"record_offset":6128,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1476,"record_offset":6584,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$44::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1476,"record_offset":8572,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":5192,"source_line":2340,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1476,"record_offset":9116,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1476,"record_offset":14400,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1476,"record_offset":15036,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1476,"record_offset":15064,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":25176,"source_line":41,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::distance
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1476,"record_offset":15008,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":70935,"source_line":65,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1476,"record_offset":10916,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":24685,"source_line":31,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::subtract
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1476,"record_offset":15156,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":25187,"source_line":48,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1476,"record_offset":13680,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":24689,"source_line":55,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1476,"record_offset":11040,"parent_record_offset":5800,"ancestor_record_offset":5800,"inlinee":4621,"source_line":61,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1476,"record_offset":11572,"parent_record_offset":11548,"ancestor_record_offset":5800,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1476,"record_offset":11596,"parent_record_offset":11572,"ancestor_record_offset":5800,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1476,"record_offset":12516,"parent_record_offset":12492,"ancestor_record_offset":5800,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1476,"record_offset":11548,"parent_record_offset":11372,"ancestor_record_offset":5800,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1476,"record_offset":12492,"parent_record_offset":12404,"ancestor_record_offset":5800,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":8996,"parent_record_offset":8964,"ancestor_record_offset":5800,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":11312,"parent_record_offset":11284,"ancestor_record_offset":5800,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":12432,"parent_record_offset":12404,"ancestor_record_offset":5800,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":12404,"parent_record_offset":12380,"ancestor_record_offset":5800,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":9044,"parent_record_offset":9020,"ancestor_record_offset":5800,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":9020,"parent_record_offset":8996,"ancestor_record_offset":5800,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":11372,"parent_record_offset":11340,"ancestor_record_offset":5800,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":11340,"parent_record_offset":11312,"ancestor_record_offset":5800,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":12380,"parent_record_offset":12356,"ancestor_record_offset":5800,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":11408,"parent_record_offset":11372,"ancestor_record_offset":5800,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1476,"record_offset":11244,"parent_record_offset":11208,"ancestor_record_offset":5800,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1476,"record_offset":11208,"parent_record_offset":11136,"ancestor_record_offset":5800,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1476,"record_offset":10784,"parent_record_offset":10756,"ancestor_record_offset":5800,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":8964,"parent_record_offset":8932,"ancestor_record_offset":5800,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":16608,"parent_record_offset":16584,"ancestor_record_offset":5800,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":11180,"parent_record_offset":11136,"ancestor_record_offset":5800,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":11284,"parent_record_offset":11244,"ancestor_record_offset":5800,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":11100,"parent_record_offset":11040,"ancestor_record_offset":5800,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":14488,"parent_record_offset":14428,"ancestor_record_offset":5800,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":10396,"parent_record_offset":10372,"ancestor_record_offset":5800,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":11136,"parent_record_offset":11100,"ancestor_record_offset":5800,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1476,"record_offset":16236,"parent_record_offset":16212,"ancestor_record_offset":5800,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1476,"record_offset":16284,"parent_record_offset":16260,"ancestor_record_offset":5800,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1476,"record_offset":11432,"parent_record_offset":11408,"ancestor_record_offset":5800,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1476,"record_offset":11456,"parent_record_offset":11432,"ancestor_record_offset":5800,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1476,"record_offset":17084,"parent_record_offset":16992,"ancestor_record_offset":5800,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1476,"record_offset":7896,"parent_record_offset":7868,"ancestor_record_offset":5800,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1476,"record_offset":7948,"parent_record_offset":7924,"ancestor_record_offset":5800,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1476,"record_offset":7924,"parent_record_offset":7896,"ancestor_record_offset":5800,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1476,"record_offset":7844,"parent_record_offset":7816,"ancestor_record_offset":5800,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1476,"record_offset":7868,"parent_record_offset":7844,"ancestor_record_offset":5800,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1476,"record_offset":8040,"parent_record_offset":7868,"ancestor_record_offset":5800,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1476,"record_offset":8152,"parent_record_offset":8128,"ancestor_record_offset":5800,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1476,"record_offset":10224,"parent_record_offset":10200,"ancestor_record_offset":5800,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1476,"record_offset":10248,"parent_record_offset":10224,"ancestor_record_offset":5800,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1476,"record_offset":10720,"parent_record_offset":10664,"ancestor_record_offset":5800,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1476,"record_offset":8784,"parent_record_offset":8756,"ancestor_record_offset":5800,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1476,"record_offset":8844,"parent_record_offset":8812,"ancestor_record_offset":5800,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1476,"record_offset":10952,"parent_record_offset":10916,"ancestor_record_offset":5800,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1476,"record_offset":14360,"parent_record_offset":14180,"ancestor_record_offset":5800,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1476,"record_offset":11672,"parent_record_offset":11648,"ancestor_record_offset":5800,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1476,"record_offset":10848,"parent_record_offset":10812,"ancestor_record_offset":5800,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1476,"record_offset":19840,"parent_record_offset":19816,"ancestor_record_offset":5800,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1476,"record_offset":13260,"parent_record_offset":13236,"ancestor_record_offset":5800,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1476,"record_offset":13308,"parent_record_offset":13284,"ancestor_record_offset":5800,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cycle::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\cycle.rs
//     evidence={"module":1476,"record_offset":12940,"parent_record_offset":12908,"ancestor_record_offset":5800,"inlinee":12166,"source_line":34,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1476,"record_offset":19292,"parent_record_offset":18932,"ancestor_record_offset":5800,"inlinee":71056,"source_line":14,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1476,"record_offset":18972,"parent_record_offset":18932,"ancestor_record_offset":5800,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$13::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1476,"record_offset":19320,"parent_record_offset":19292,"ancestor_record_offset":5800,"inlinee":58506,"source_line":60,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1476,"record_offset":12968,"parent_record_offset":12940,"ancestor_record_offset":5800,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1476,"record_offset":10664,"parent_record_offset":10640,"ancestor_record_offset":5800,"inlinee":13252,"source_line":1156,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1476,"record_offset":19100,"parent_record_offset":19044,"ancestor_record_offset":5800,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1476,"record_offset":18680,"parent_record_offset":18656,"ancestor_record_offset":5800,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1476,"record_offset":19020,"parent_record_offset":18996,"ancestor_record_offset":5800,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1476,"record_offset":10420,"parent_record_offset":10396,"ancestor_record_offset":5800,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1476,"record_offset":19816,"parent_record_offset":18888,"ancestor_record_offset":5800,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1476,"record_offset":13332,"parent_record_offset":13308,"ancestor_record_offset":5800,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1476,"record_offset":17108,"parent_record_offset":17084,"ancestor_record_offset":5800,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1476,"record_offset":7972,"parent_record_offset":7948,"ancestor_record_offset":5800,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1476,"record_offset":11480,"parent_record_offset":11456,"ancestor_record_offset":5800,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1476,"record_offset":11504,"parent_record_offset":11480,"ancestor_record_offset":5800,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1476,"record_offset":17052,"parent_record_offset":17028,"ancestor_record_offset":5800,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1476,"record_offset":12460,"parent_record_offset":12432,"ancestor_record_offset":5800,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1476,"record_offset":6284,"parent_record_offset":6252,"ancestor_record_offset":5800,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1476,"record_offset":9400,"parent_record_offset":9368,"ancestor_record_offset":5800,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1476,"record_offset":9368,"parent_record_offset":9336,"ancestor_record_offset":5800,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1476,"record_offset":13788,"parent_record_offset":13728,"ancestor_record_offset":5800,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1476,"record_offset":13756,"parent_record_offset":13728,"ancestor_record_offset":5800,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1476,"record_offset":20780,"parent_record_offset":20752,"ancestor_record_offset":5800,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1476,"record_offset":7712,"parent_record_offset":7668,"ancestor_record_offset":5800,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1476,"record_offset":13112,"parent_record_offset":13080,"ancestor_record_offset":5800,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1476,"record_offset":17028,"parent_record_offset":16992,"ancestor_record_offset":5800,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1476,"record_offset":16992,"parent_record_offset":16968,"ancestor_record_offset":5800,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1476,"record_offset":10200,"parent_record_offset":10176,"ancestor_record_offset":5800,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1476,"record_offset":14220,"parent_record_offset":14180,"ancestor_record_offset":5800,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1476,"record_offset":10692,"parent_record_offset":10664,"ancestor_record_offset":5800,"inlinee":13253,"source_line":545,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1476,"record_offset":13080,"parent_record_offset":13056,"ancestor_record_offset":5800,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1476,"record_offset":13284,"parent_record_offset":13260,"ancestor_record_offset":5800,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1476,"record_offset":19872,"parent_record_offset":18888,"ancestor_record_offset":5800,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1476,"record_offset":6532,"parent_record_offset":6504,"ancestor_record_offset":5800,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1476,"record_offset":8500,"parent_record_offset":7668,"ancestor_record_offset":5800,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1476,"record_offset":11648,"parent_record_offset":11244,"ancestor_record_offset":5800,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1476,"record_offset":17132,"parent_record_offset":17108,"ancestor_record_offset":5800,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1476,"record_offset":8104,"parent_record_offset":7816,"ancestor_record_offset":5800,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1476,"record_offset":9964,"parent_record_offset":9940,"ancestor_record_offset":5800,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1476,"record_offset":9660,"parent_record_offset":9636,"ancestor_record_offset":5800,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1476,"record_offset":13188,"parent_record_offset":13164,"ancestor_record_offset":5800,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1476,"record_offset":6996,"parent_record_offset":6972,"ancestor_record_offset":5800,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1476,"record_offset":10052,"parent_record_offset":9996,"ancestor_record_offset":5800,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1476,"record_offset":6972,"parent_record_offset":6948,"ancestor_record_offset":5800,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1476,"record_offset":7180,"parent_record_offset":7152,"ancestor_record_offset":5800,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1476,"record_offset":10076,"parent_record_offset":10052,"ancestor_record_offset":5800,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1476,"record_offset":7996,"parent_record_offset":7972,"ancestor_record_offset":5800,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1476,"record_offset":14516,"parent_record_offset":14488,"ancestor_record_offset":5800,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1476,"record_offset":14544,"parent_record_offset":14516,"ancestor_record_offset":5800,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1476,"record_offset":10312,"parent_record_offset":10288,"ancestor_record_offset":5800,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1476,"record_offset":6924,"parent_record_offset":6900,"ancestor_record_offset":5800,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1476,"record_offset":13164,"parent_record_offset":13140,"ancestor_record_offset":5800,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1476,"record_offset":6948,"parent_record_offset":6924,"ancestor_record_offset":5800,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1476,"record_offset":7252,"parent_record_offset":7224,"ancestor_record_offset":5800,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1476,"record_offset":12992,"parent_record_offset":12968,"ancestor_record_offset":5800,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1476,"record_offset":6900,"parent_record_offset":6784,"ancestor_record_offset":5800,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1476,"record_offset":13140,"parent_record_offset":13080,"ancestor_record_offset":5800,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1476,"record_offset":10812,"parent_record_offset":10784,"ancestor_record_offset":5800,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1476,"record_offset":6252,"parent_record_offset":6224,"ancestor_record_offset":5800,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1476,"record_offset":6612,"parent_record_offset":6584,"ancestor_record_offset":5800,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1476,"record_offset":6752,"parent_record_offset":6720,"ancestor_record_offset":5800,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1476,"record_offset":6720,"parent_record_offset":6684,"ancestor_record_offset":5800,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1476,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":5800,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1476,"record_offset":6684,"parent_record_offset":6660,"ancestor_record_offset":5800,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1476,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":5800,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1476,"record_offset":7224,"parent_record_offset":6684,"ancestor_record_offset":5800,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1476,"record_offset":6224,"parent_record_offset":6192,"ancestor_record_offset":5800,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1476,"record_offset":6192,"parent_record_offset":6164,"ancestor_record_offset":5800,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1476,"record_offset":6384,"parent_record_offset":6352,"ancestor_record_offset":5800,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1476,"record_offset":6480,"parent_record_offset":6384,"ancestor_record_offset":5800,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1476,"record_offset":6352,"parent_record_offset":6328,"ancestor_record_offset":5800,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1476,"record_offset":6164,"parent_record_offset":6128,"ancestor_record_offset":5800,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1476,"record_offset":6504,"parent_record_offset":6480,"ancestor_record_offset":5800,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1476,"record_offset":6784,"parent_record_offset":6752,"ancestor_record_offset":5800,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1476,"record_offset":7068,"parent_record_offset":6784,"ancestor_record_offset":5800,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1476,"record_offset":7040,"parent_record_offset":6784,"ancestor_record_offset":5800,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1476,"record_offset":6844,"parent_record_offset":6784,"ancestor_record_offset":5800,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1476,"record_offset":20808,"parent_record_offset":20752,"ancestor_record_offset":5800,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1476,"record_offset":14732,"parent_record_offset":14708,"ancestor_record_offset":5800,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1476,"record_offset":14756,"parent_record_offset":14732,"ancestor_record_offset":5800,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1476,"record_offset":14904,"parent_record_offset":14732,"ancestor_record_offset":5800,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1476,"record_offset":20752,"parent_record_offset":20728,"ancestor_record_offset":5800,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1476,"record_offset":9296,"parent_record_offset":9256,"ancestor_record_offset":5800,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1476,"record_offset":9336,"parent_record_offset":9296,"ancestor_record_offset":5800,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1476,"record_offset":9444,"parent_record_offset":9256,"ancestor_record_offset":5800,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1476,"record_offset":9256,"parent_record_offset":9208,"ancestor_record_offset":5800,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1476,"record_offset":8756,"parent_record_offset":8728,"ancestor_record_offset":5800,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1476,"record_offset":8812,"parent_record_offset":8756,"ancestor_record_offset":5800,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1476,"record_offset":9536,"parent_record_offset":9208,"ancestor_record_offset":5800,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1476,"record_offset":8620,"parent_record_offset":8596,"ancestor_record_offset":5800,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1476,"record_offset":9144,"parent_record_offset":9116,"ancestor_record_offset":5800,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1476,"record_offset":9636,"parent_record_offset":9208,"ancestor_record_offset":5800,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1476,"record_offset":8728,"parent_record_offset":8668,"ancestor_record_offset":5800,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1476,"record_offset":9208,"parent_record_offset":9172,"ancestor_record_offset":5800,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1476,"record_offset":8644,"parent_record_offset":8620,"ancestor_record_offset":5800,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1476,"record_offset":8700,"parent_record_offset":8668,"ancestor_record_offset":5800,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1476,"record_offset":8668,"parent_record_offset":8644,"ancestor_record_offset":5800,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1476,"record_offset":9172,"parent_record_offset":9144,"ancestor_record_offset":5800,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1476,"record_offset":13236,"parent_record_offset":12908,"ancestor_record_offset":5800,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1476,"record_offset":19644,"parent_record_offset":18888,"ancestor_record_offset":5800,"inlinee":4542,"source_line":388,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1476,"record_offset":12908,"parent_record_offset":12880,"ancestor_record_offset":5800,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1476,"record_offset":13728,"parent_record_offset":13680,"ancestor_record_offset":5800,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1476,"record_offset":7596,"parent_record_offset":7560,"ancestor_record_offset":5800,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1476,"record_offset":21072,"parent_record_offset":21040,"ancestor_record_offset":5800,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1476,"record_offset":21588,"parent_record_offset":21328,"ancestor_record_offset":5800,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1476,"record_offset":7740,"parent_record_offset":7712,"ancestor_record_offset":5800,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1476,"record_offset":7816,"parent_record_offset":7668,"ancestor_record_offset":5800,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1476,"record_offset":8076,"parent_record_offset":7816,"ancestor_record_offset":5800,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1476,"record_offset":8596,"parent_record_offset":8572,"ancestor_record_offset":5800,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1476,"record_offset":20840,"parent_record_offset":20808,"ancestor_record_offset":5800,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1476,"record_offset":7768,"parent_record_offset":7740,"ancestor_record_offset":5800,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1476,"record_offset":7668,"parent_record_offset":7632,"ancestor_record_offset":5800,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1476,"record_offset":7632,"parent_record_offset":7596,"ancestor_record_offset":5800,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$3
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs
//     evidence={"module":1476,"record_offset":19212,"parent_record_offset":19188,"ancestor_record_offset":5800,"inlinee":71054,"source_line":327,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1476,"record_offset":11004,"parent_record_offset":10980,"ancestor_record_offset":5800,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1476,"record_offset":14428,"parent_record_offset":14400,"ancestor_record_offset":5800,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1476,"record_offset":14460,"parent_record_offset":14428,"ancestor_record_offset":5800,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1476,"record_offset":14584,"parent_record_offset":14428,"ancestor_record_offset":5800,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1476,"record_offset":14180,"parent_record_offset":14148,"ancestor_record_offset":5800,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1476,"record_offset":15180,"parent_record_offset":15156,"ancestor_record_offset":5800,"inlinee":25176,"source_line":41,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1476,"record_offset":13904,"parent_record_offset":13680,"ancestor_record_offset":5800,"inlinee":24685,"source_line":31,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1476,"record_offset":10980,"parent_record_offset":10916,"ancestor_record_offset":5800,"inlinee":24686,"source_line":90,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1476,"record_offset":14148,"parent_record_offset":14116,"ancestor_record_offset":5800,"inlinee":25178,"source_line":115,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1476,"record_offset":14708,"parent_record_offset":14428,"ancestor_record_offset":5800,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1476,"record_offset":2052,"rva":19170048,"va":5387879168}
// 编译实例 RVAs: 0x1248300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12798408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879168,"raw_hex":"e838b63cff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387879907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-13524702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879168,"raw_hex":"e822a131ff","size":5,"soft_fail":false,"target_va":5374354992,"va":5387879689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9851196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879168,"raw_hex":"e8c4ae69ff","size":5,"soft_fail":false,"target_va":5378028464,"va":5387879655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8085867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387879168,"raw_hex":"e8959e84ff","size":5,"soft_fail":false,"target_va":5379793584,"va":5387879446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":2272,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":3340,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1476,"record_offset":3532,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1476,"record_offset":5324,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1476,"record_offset":5296,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1476,"record_offset":2328,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1476,"record_offset":2668,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1476,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":2052,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1476,"record_offset":5488,"parent_record_offset":5396,"ancestor_record_offset":2052,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":3412,"parent_record_offset":3380,"ancestor_record_offset":2052,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":5424,"parent_record_offset":5396,"ancestor_record_offset":2052,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":5396,"parent_record_offset":5372,"ancestor_record_offset":2052,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":3460,"parent_record_offset":3436,"ancestor_record_offset":2052,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":3436,"parent_record_offset":3412,"ancestor_record_offset":2052,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":2052,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":3380,"parent_record_offset":3340,"ancestor_record_offset":2052,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":3808,"parent_record_offset":3784,"ancestor_record_offset":2052,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":3784,"parent_record_offset":3684,"ancestor_record_offset":2052,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":2052,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":1476,"record_offset":3984,"parent_record_offset":3652,"ancestor_record_offset":2052,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":1476,"record_offset":3652,"parent_record_offset":3608,"ancestor_record_offset":2052,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1476,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":2052,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1476,"record_offset":4456,"parent_record_offset":3608,"ancestor_record_offset":2052,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1476,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":2052,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1476,"record_offset":4952,"parent_record_offset":3608,"ancestor_record_offset":2052,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1476,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":2052,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":1476,"record_offset":4900,"parent_record_offset":4876,"ancestor_record_offset":2052,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1476,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":2052,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1476,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":2052,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1476,"record_offset":4760,"parent_record_offset":4736,"ancestor_record_offset":2052,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1476,"record_offset":4532,"parent_record_offset":4504,"ancestor_record_offset":2052,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1476,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":2052,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1476,"record_offset":4196,"parent_record_offset":3608,"ancestor_record_offset":2052,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":1476,"record_offset":4876,"parent_record_offset":4852,"ancestor_record_offset":2052,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1476,"record_offset":4852,"parent_record_offset":4828,"ancestor_record_offset":2052,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1476,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":2052,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1476,"record_offset":3608,"parent_record_offset":3584,"ancestor_record_offset":2052,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":1476,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":2052,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":1476,"record_offset":4044,"parent_record_offset":4016,"ancestor_record_offset":2052,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1476,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":2052,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1476,"record_offset":2552,"parent_record_offset":2524,"ancestor_record_offset":2052,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1476,"record_offset":2612,"parent_record_offset":2580,"ancestor_record_offset":2052,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1476,"record_offset":5456,"parent_record_offset":5424,"ancestor_record_offset":2052,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1476,"record_offset":2948,"parent_record_offset":2916,"ancestor_record_offset":2052,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1476,"record_offset":2916,"parent_record_offset":2884,"ancestor_record_offset":2052,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1476,"record_offset":3928,"parent_record_offset":3652,"ancestor_record_offset":2052,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1476,"record_offset":4672,"parent_record_offset":4608,"ancestor_record_offset":2052,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1476,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":2052,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1476,"record_offset":4736,"parent_record_offset":4556,"ancestor_record_offset":2052,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1476,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":2052,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1476,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":2052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1476,"record_offset":4148,"parent_record_offset":4116,"ancestor_record_offset":2052,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1476,"record_offset":3208,"parent_record_offset":3184,"ancestor_record_offset":2052,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1476,"record_offset":3500,"parent_record_offset":3340,"ancestor_record_offset":2052,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1476,"record_offset":3684,"parent_record_offset":3652,"ancestor_record_offset":2052,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1476,"record_offset":3752,"parent_record_offset":3724,"ancestor_record_offset":2052,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1476,"record_offset":3724,"parent_record_offset":3684,"ancestor_record_offset":2052,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1476,"record_offset":4016,"parent_record_offset":3608,"ancestor_record_offset":2052,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1476,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":2052,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1476,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":2052,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1476,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":2052,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1476,"record_offset":2844,"parent_record_offset":2804,"ancestor_record_offset":2052,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1476,"record_offset":2884,"parent_record_offset":2844,"ancestor_record_offset":2052,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1476,"record_offset":2992,"parent_record_offset":2804,"ancestor_record_offset":2052,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1476,"record_offset":2804,"parent_record_offset":2756,"ancestor_record_offset":2052,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1476,"record_offset":2524,"parent_record_offset":2472,"ancestor_record_offset":2052,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1476,"record_offset":2580,"parent_record_offset":2524,"ancestor_record_offset":2052,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1476,"record_offset":3084,"parent_record_offset":2756,"ancestor_record_offset":2052,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1476,"record_offset":2356,"parent_record_offset":2328,"ancestor_record_offset":2052,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1476,"record_offset":2696,"parent_record_offset":2668,"ancestor_record_offset":2052,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1476,"record_offset":3184,"parent_record_offset":2756,"ancestor_record_offset":2052,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1476,"record_offset":2472,"parent_record_offset":2412,"ancestor_record_offset":2052,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1476,"record_offset":2756,"parent_record_offset":2724,"ancestor_record_offset":2052,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1476,"record_offset":2384,"parent_record_offset":2356,"ancestor_record_offset":2052,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1476,"record_offset":2444,"parent_record_offset":2412,"ancestor_record_offset":2052,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1476,"record_offset":2412,"parent_record_offset":2384,"ancestor_record_offset":2052,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1476,"record_offset":2724,"parent_record_offset":2696,"ancestor_record_offset":2052,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1476,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":2052,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::find_overlap_segments::impl$0::new::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs
//     evidence={"module":1476,"record_offset":3560,"parent_record_offset":3532,"ancestor_record_offset":2052,"inlinee":71246,"source_line":43,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::impl$0::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1164E
// 首条位置: {"module":1476,"inlinee":71246}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1476,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":3560,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":3832,"parent_record_offset":3808,"ancestor_record_offset":3560,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":3880,"parent_record_offset":3856,"ancestor_record_offset":3560,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1476,"record_offset":3856,"parent_record_offset":3832,"ancestor_record_offset":3560,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":3808,"parent_record_offset":3784,"ancestor_record_offset":3560,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":3784,"parent_record_offset":3684,"ancestor_record_offset":3560,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1476,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":3560,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":1476,"record_offset":3984,"parent_record_offset":3652,"ancestor_record_offset":3560,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":1476,"record_offset":3652,"parent_record_offset":3608,"ancestor_record_offset":3560,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1476,"record_offset":4976,"parent_record_offset":4952,"ancestor_record_offset":3560,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1476,"record_offset":4456,"parent_record_offset":3608,"ancestor_record_offset":3560,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1476,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":3560,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1476,"record_offset":4952,"parent_record_offset":3608,"ancestor_record_offset":3560,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1476,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":3560,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":1476,"record_offset":4900,"parent_record_offset":4876,"ancestor_record_offset":3560,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1476,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":3560,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1476,"record_offset":4556,"parent_record_offset":4532,"ancestor_record_offset":3560,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1476,"record_offset":4760,"parent_record_offset":4736,"ancestor_record_offset":3560,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1476,"record_offset":4532,"parent_record_offset":4504,"ancestor_record_offset":3560,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1476,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":3560,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1476,"record_offset":4196,"parent_record_offset":3608,"ancestor_record_offset":3560,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":1476,"record_offset":4876,"parent_record_offset":4852,"ancestor_record_offset":3560,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1476,"record_offset":4852,"parent_record_offset":4828,"ancestor_record_offset":3560,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1476,"record_offset":3608,"parent_record_offset":3584,"ancestor_record_offset":3560,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":1476,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":3560,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":1476,"record_offset":4044,"parent_record_offset":4016,"ancestor_record_offset":3560,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1476,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":3560,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1476,"record_offset":3928,"parent_record_offset":3652,"ancestor_record_offset":3560,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1476,"record_offset":4672,"parent_record_offset":4608,"ancestor_record_offset":3560,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1476,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":3560,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1476,"record_offset":4736,"parent_record_offset":4556,"ancestor_record_offset":3560,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1476,"record_offset":4148,"parent_record_offset":4116,"ancestor_record_offset":3560,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1476,"record_offset":3684,"parent_record_offset":3652,"ancestor_record_offset":3560,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1476,"record_offset":3752,"parent_record_offset":3724,"ancestor_record_offset":3560,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1476,"record_offset":3724,"parent_record_offset":3684,"ancestor_record_offset":3560,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1476,"record_offset":4016,"parent_record_offset":3608,"ancestor_record_offset":3560,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1476,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":3560,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1476,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":3560,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1476,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":3560,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1476,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":3560,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/01476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11484
// 首条位置: {"module":1415,"inlinee":70788}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1415,"record_offset":54616,"parent_record_offset":54592,"ancestor_record_offset":54592,"inlinee":24561,"source_line":207,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1415,"record_offset":54640,"parent_record_offset":54616,"ancestor_record_offset":54592,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01415.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=48
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11541
// 首条位置: {"module":1444,"inlinee":70977}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1444,"record_offset":5912,"parent_record_offset":5844,"ancestor_record_offset":5844,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01444.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=54
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1444,"record_offset":5864,"parent_record_offset":5844,"ancestor_record_offset":5844,"inlinee":4695,"source_line":125,"archive":"_pdb/B/modules/01444.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 327
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1158E
// 首条位置: {"module":1452,"inlinee":71054}
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
// 完整结构及原始类型记录: find_overlap_segments.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: find_overlap_segments.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: find_overlap_segments.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::WallId>,ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>", "ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache::new
// 信息 ID: 05c95f40c765ec34f20055c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x786f99 -> 0x1248300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments
// 信息 ID: 9f9a94af278ee0d6f7642585
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x7874d7 -> 0x1248620 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x7875de -> 0x1248620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$0
// 信息 ID: 1023ea185eca7775a0f9ee49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$0> >; module=1415, inline_record=54592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$1
// 信息 ID: 8681f7e3c25aa2d0b1dd2cab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::shared::smallsort::sort4_stable<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::stable::driftsort_main<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>,alloc::vec::Vec<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::stable::drift::sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::shared::pivot::median3_rec<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::stable::quicksort::quicksort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 41 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1444, inline_record=5844; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1444, inline_record=5992; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1444, inline_record=6156; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1444, inline_record=6328; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1444, inline_record=6564; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1444, inline_record=6956; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1444, inline_record=43504; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1444, inline_record=43704; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$3
// 信息 ID: f11b621e74e9d234a3b7c90b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > > >; module=1472, inline_record=1456; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > > >; module=1472, inline_record=1864; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > > >; module=1472, inline_record=2668; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > > >; module=1472, inline_record=2812; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1476, inline_record=19212; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1476, inline_record=19596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::impl$0::new::closure$0
// 信息 ID: a3f17dedd598f56ae6ff56c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::find_overlap_segments::impl$0::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache::new; module=1476, inline_record=3560; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
