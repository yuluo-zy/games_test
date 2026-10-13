// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 4fe1ca37fadf9b785756b67190af5a9ce92aadfbaf77237ae2737b12ea78a248
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: find_overlap_segments.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1455, 1474, 1482, 1500, 1504, 1535, 3159, 4370, 4371, 4375
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
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
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
// 首条位置: {"module":1535,"record_offset":3904,"rva":21549792,"va":5390258912}
// 编译实例 RVAs: 0x148d2e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14581910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e86a7f21ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5390260593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=15
//     evidence={"assembly":"\tcallq\t-14583489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e83f7921ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390262188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21538388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8ac59b7fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390260575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=8
//     target_aliases=11 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21026276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e81c2abffe","size":5,"soft_fail":false,"target_va":5369240528,"va":5390266799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a620a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10664870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e85a445dff","size":5,"soft_fail":false,"target_va":5379596448,"va":5390261313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fc700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-594195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8edeef6ff","size":5,"soft_fail":false,"target_va":5389666048,"va":5390260238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405920
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-558057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8177cf7ff","size":5,"soft_fail":false,"target_va":5389703456,"va":5390261508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405de0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-556551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8f981f7ff","size":5,"soft_fail":false,"target_va":5389704672,"va":5390261218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141420c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-446189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e81331f9ff","size":5,"soft_fail":false,"target_va":5389814848,"va":5390261032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-135242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8b6effdff","size":5,"soft_fail":false,"target_va":5390127040,"va":5390262277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141473d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-108847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8d156feff","size":5,"soft_fail":false,"target_va":5390155024,"va":5390263866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26091323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e83b1f8e01","size":5,"soft_fail":false,"target_va":5416351056,"va":5390259728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26796420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e884e19801","size":5,"soft_fail":false,"target_va":5417057056,"va":5390260631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26797328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e810e59801","size":5,"soft_fail":false,"target_va":5417057840,"va":5390260507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t26121389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8ad948e01","size":5,"soft_fail":false,"target_va":5416381792,"va":5390260398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26355873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8a1289201","size":5,"soft_fail":false,"target_va":5416620794,"va":5390264916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26356220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8fc299201","size":5,"soft_fail":false,"target_va":5416621408,"va":5390265183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26356544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8402b9201","size":5,"soft_fail":false,"target_va":5416621600,"va":5390265051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t26357230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8ee2d9201","size":5,"soft_fail":false,"target_va":5416622208,"va":5390264973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26356509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e81d2b9201","size":5,"soft_fail":false,"target_va":5416621762,"va":5390265248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26357202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8d22d9201","size":5,"soft_fail":false,"target_va":5416622240,"va":5390265033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-115308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8943dfeff","size":5,"soft_fail":false,"target_va":5390146992,"va":5390262295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8d6270000","size":5,"soft_fail":false,"target_va":5390269280,"va":5390259077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26356067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e863299201","size":5,"soft_fail":false,"target_va":5416621376,"va":5390265304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::gable_sides
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9067543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8e9a375ff","size":5,"soft_fail":false,"target_va":5381194960,"va":5390262498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9066362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e886a875ff","size":5,"soft_fail":false,"target_va":5381196128,"va":5390262485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9030793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8773376ff","size":5,"soft_fail":false,"target_va":5381231904,"va":5390262692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9025414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e87a4876ff","size":5,"soft_fail":false,"target_va":5381234608,"va":5390260017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9356442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8663b71ff","size":5,"soft_fail":false,"target_va":5380903536,"va":5390259973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::utils_intersect::line_x_shape_overlap
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9074633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8378875ff","size":5,"soft_fail":false,"target_va":5381188096,"va":5390262724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6295623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8b9ef9fff","size":5,"soft_fail":false,"target_va":5383963792,"va":5390259410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6287116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8f410a0ff","size":5,"soft_fail":false,"target_va":5383972224,"va":5390259335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6299884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e814df9fff","size":5,"soft_fail":false,"target_va":5383964912,"va":5390264791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6296241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e84fed9fff","size":5,"soft_fail":false,"target_va":5383962816,"va":5390259052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26431810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e842519301","size":5,"soft_fail":false,"target_va":5416697008,"va":5390265193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26354120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8c8219201","size":5,"soft_fail":false,"target_va":5416619360,"va":5390265235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve2
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6953674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e836e595ff","size":5,"soft_fail":false,"target_va":5383306512,"va":5390260181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::contains_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6959396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8dcce95ff","size":5,"soft_fail":false,"target_va":5383304336,"va":5390263727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6957371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8c5d695ff","size":5,"soft_fail":false,"target_va":5383307120,"va":5390264486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve2
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6938517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e86b2096ff","size":5,"soft_fail":false,"target_va":5383321632,"va":5390260144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6950408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8f8f195ff","size":5,"soft_fail":false,"target_va":5383313216,"va":5390263619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6939381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e80b1d96ff","size":5,"soft_fail":false,"target_va":5383323216,"va":5390262592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6951250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8aeee95ff","size":5,"soft_fail":false,"target_va":5383309120,"va":5390260365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6939335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8391d96ff","size":5,"soft_fail":false,"target_va":5383323424,"va":5390262754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6944088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258912,"raw_hex":"e8a80a96ff","size":5,"soft_fail":false,"target_va":5383320304,"va":5390264387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1535,"record_offset":8560,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":6444,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":6764,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":7712,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":14596,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1535,"record_offset":7768,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1535,"record_offset":14824,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1535,"record_offset":11772,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1535,"record_offset":17704,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1535,"record_offset":8444,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":11646,"source_line":1259,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1535,"record_offset":16572,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1535,"record_offset":10716,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1535,"record_offset":8144,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1535,"record_offset":7948,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1535,"record_offset":7884,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1535,"record_offset":13320,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1535,"record_offset":8060,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1535,"record_offset":14708,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1535,"record_offset":7508,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1535,"record_offset":7920,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1535,"record_offset":17732,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$10::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1535,"record_offset":10744,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":9887,"source_line":276,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1535,"record_offset":5636,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1535,"record_offset":4232,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1535,"record_offset":4624,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$35::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1535,"record_offset":6472,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":8500,"source_line":2025,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1535,"record_offset":6976,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1535,"record_offset":11984,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1535,"record_offset":12620,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1535,"record_offset":12648,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":27120,"source_line":41,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::distance
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1535,"record_offset":12592,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":79567,"source_line":65,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1535,"record_offset":8720,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::subtract
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1535,"record_offset":12740,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":27119,"source_line":48,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1535,"record_offset":11344,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":79541,"source_line":55,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1535,"record_offset":8844,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1535,"record_offset":9352,"parent_record_offset":9328,"ancestor_record_offset":3904,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1535,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":3904,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1535,"record_offset":10524,"parent_record_offset":10500,"ancestor_record_offset":3904,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1535,"record_offset":10548,"parent_record_offset":10524,"ancestor_record_offset":3904,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1535,"record_offset":9400,"parent_record_offset":9376,"ancestor_record_offset":3904,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1535,"record_offset":10572,"parent_record_offset":10548,"ancestor_record_offset":3904,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1535,"record_offset":9328,"parent_record_offset":9148,"ancestor_record_offset":3904,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1535,"record_offset":10500,"parent_record_offset":10412,"ancestor_record_offset":3904,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":6852,"parent_record_offset":6820,"ancestor_record_offset":3904,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":9088,"parent_record_offset":9060,"ancestor_record_offset":3904,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":10440,"parent_record_offset":10412,"ancestor_record_offset":3904,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":10412,"parent_record_offset":10388,"ancestor_record_offset":3904,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":6900,"parent_record_offset":6876,"ancestor_record_offset":3904,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":6876,"parent_record_offset":6852,"ancestor_record_offset":3904,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":9148,"parent_record_offset":9116,"ancestor_record_offset":3904,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":9116,"parent_record_offset":9088,"ancestor_record_offset":3904,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":10388,"parent_record_offset":10364,"ancestor_record_offset":3904,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":9184,"parent_record_offset":9148,"ancestor_record_offset":3904,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1535,"record_offset":9632,"parent_record_offset":9560,"ancestor_record_offset":3904,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1535,"record_offset":9016,"parent_record_offset":8944,"ancestor_record_offset":3904,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1535,"record_offset":8588,"parent_record_offset":8560,"ancestor_record_offset":3904,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":6820,"parent_record_offset":6788,"ancestor_record_offset":3904,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":8988,"parent_record_offset":8944,"ancestor_record_offset":3904,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":3904,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":9060,"parent_record_offset":9016,"ancestor_record_offset":3904,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":8908,"parent_record_offset":8844,"ancestor_record_offset":3904,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":12072,"parent_record_offset":12012,"ancestor_record_offset":3904,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":3904,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":19668,"parent_record_offset":19640,"ancestor_record_offset":3904,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":8944,"parent_record_offset":8908,"ancestor_record_offset":3904,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1535,"record_offset":14248,"parent_record_offset":14224,"ancestor_record_offset":3904,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1535,"record_offset":14296,"parent_record_offset":14272,"ancestor_record_offset":3904,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1535,"record_offset":9208,"parent_record_offset":9184,"ancestor_record_offset":3904,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1535,"record_offset":14848,"parent_record_offset":14824,"ancestor_record_offset":3904,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1535,"record_offset":14964,"parent_record_offset":14872,"ancestor_record_offset":3904,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1535,"record_offset":5964,"parent_record_offset":5936,"ancestor_record_offset":3904,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1535,"record_offset":5992,"parent_record_offset":5964,"ancestor_record_offset":3904,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1535,"record_offset":5912,"parent_record_offset":5884,"ancestor_record_offset":3904,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1535,"record_offset":5936,"parent_record_offset":5912,"ancestor_record_offset":3904,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1535,"record_offset":6052,"parent_record_offset":5936,"ancestor_record_offset":3904,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1535,"record_offset":6164,"parent_record_offset":6140,"ancestor_record_offset":3904,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1535,"record_offset":7996,"parent_record_offset":7972,"ancestor_record_offset":3904,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1535,"record_offset":8020,"parent_record_offset":7996,"ancestor_record_offset":3904,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1535,"record_offset":8524,"parent_record_offset":8468,"ancestor_record_offset":3904,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1535,"record_offset":6680,"parent_record_offset":6656,"ancestor_record_offset":3904,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1535,"record_offset":8756,"parent_record_offset":8720,"ancestor_record_offset":3904,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1535,"record_offset":17868,"parent_record_offset":17844,"ancestor_record_offset":3904,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1535,"record_offset":8652,"parent_record_offset":8616,"ancestor_record_offset":3904,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1535,"record_offset":9288,"parent_record_offset":9232,"ancestor_record_offset":3904,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1535,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":3904,"inlinee":8137,"source_line":36,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cycle::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\cycle.rs
//     evidence={"module":1535,"record_offset":10832,"parent_record_offset":10784,"ancestor_record_offset":3904,"inlinee":9870,"source_line":34,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1535,"record_offset":10144,"parent_record_offset":10088,"ancestor_record_offset":3904,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1535,"record_offset":10868,"parent_record_offset":10832,"ancestor_record_offset":3904,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":1535,"record_offset":10088,"parent_record_offset":9632,"ancestor_record_offset":3904,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1535,"record_offset":8468,"parent_record_offset":8444,"ancestor_record_offset":3904,"inlinee":11648,"source_line":1164,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1535,"record_offset":16596,"parent_record_offset":16572,"ancestor_record_offset":3904,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1535,"record_offset":8192,"parent_record_offset":8168,"ancestor_record_offset":3904,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1535,"record_offset":11252,"parent_record_offset":10784,"ancestor_record_offset":3904,"inlinee":8135,"source_line":507,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1535,"record_offset":14988,"parent_record_offset":14964,"ancestor_record_offset":3904,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1535,"record_offset":10296,"parent_record_offset":9632,"ancestor_record_offset":3904,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1535,"record_offset":6016,"parent_record_offset":5992,"ancestor_record_offset":3904,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1535,"record_offset":9232,"parent_record_offset":9208,"ancestor_record_offset":3904,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1535,"record_offset":9260,"parent_record_offset":9232,"ancestor_record_offset":3904,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1535,"record_offset":14932,"parent_record_offset":14908,"ancestor_record_offset":3904,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1535,"record_offset":10468,"parent_record_offset":10440,"ancestor_record_offset":3904,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1535,"record_offset":7224,"parent_record_offset":7200,"ancestor_record_offset":3904,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1535,"record_offset":7200,"parent_record_offset":7176,"ancestor_record_offset":3904,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1535,"record_offset":11444,"parent_record_offset":11392,"ancestor_record_offset":3904,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1535,"record_offset":11416,"parent_record_offset":11392,"ancestor_record_offset":3904,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1535,"record_offset":17784,"parent_record_offset":17756,"ancestor_record_offset":3904,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1535,"record_offset":5784,"parent_record_offset":5744,"ancestor_record_offset":3904,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1535,"record_offset":11124,"parent_record_offset":11064,"ancestor_record_offset":3904,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1535,"record_offset":14908,"parent_record_offset":14872,"ancestor_record_offset":3904,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1535,"record_offset":14872,"parent_record_offset":14848,"ancestor_record_offset":3904,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1535,"record_offset":7972,"parent_record_offset":7948,"ancestor_record_offset":3904,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1535,"record_offset":11860,"parent_record_offset":11828,"ancestor_record_offset":3904,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1535,"record_offset":8496,"parent_record_offset":8468,"ancestor_record_offset":3904,"inlinee":10957,"source_line":559,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1535,"record_offset":11308,"parent_record_offset":10784,"ancestor_record_offset":3904,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1535,"record_offset":4572,"parent_record_offset":4544,"ancestor_record_offset":3904,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1535,"record_offset":6400,"parent_record_offset":5744,"ancestor_record_offset":3904,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1535,"record_offset":9456,"parent_record_offset":9016,"ancestor_record_offset":3904,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1535,"record_offset":15012,"parent_record_offset":14988,"ancestor_record_offset":3904,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1535,"record_offset":9480,"parent_record_offset":9456,"ancestor_record_offset":3904,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=41
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1535,"record_offset":6116,"parent_record_offset":5884,"ancestor_record_offset":3904,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1535,"record_offset":7736,"parent_record_offset":7712,"ancestor_record_offset":3904,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1535,"record_offset":7376,"parent_record_offset":7352,"ancestor_record_offset":3904,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1535,"record_offset":10256,"parent_record_offset":10188,"ancestor_record_offset":3904,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1535,"record_offset":5044,"parent_record_offset":5020,"ancestor_record_offset":3904,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1535,"record_offset":7824,"parent_record_offset":7768,"ancestor_record_offset":3904,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1535,"record_offset":5020,"parent_record_offset":4996,"ancestor_record_offset":3904,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1535,"record_offset":5228,"parent_record_offset":5200,"ancestor_record_offset":3904,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1535,"record_offset":7848,"parent_record_offset":7824,"ancestor_record_offset":3904,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1535,"record_offset":6940,"parent_record_offset":6788,"ancestor_record_offset":3904,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1535,"record_offset":5328,"parent_record_offset":5300,"ancestor_record_offset":3904,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1535,"record_offset":5300,"parent_record_offset":5272,"ancestor_record_offset":3904,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1535,"record_offset":14620,"parent_record_offset":14596,"ancestor_record_offset":3904,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1535,"record_offset":12100,"parent_record_offset":12072,"ancestor_record_offset":3904,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1535,"record_offset":12128,"parent_record_offset":12100,"ancestor_record_offset":3904,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1535,"record_offset":8084,"parent_record_offset":8060,"ancestor_record_offset":3904,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1535,"record_offset":4972,"parent_record_offset":4948,"ancestor_record_offset":3904,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1535,"record_offset":4996,"parent_record_offset":4972,"ancestor_record_offset":3904,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1535,"record_offset":5356,"parent_record_offset":5328,"ancestor_record_offset":3904,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1535,"record_offset":10188,"parent_record_offset":10144,"ancestor_record_offset":3904,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1535,"record_offset":4948,"parent_record_offset":4832,"ancestor_record_offset":3904,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1535,"record_offset":8616,"parent_record_offset":8588,"ancestor_record_offset":3904,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1535,"record_offset":4360,"parent_record_offset":4336,"ancestor_record_offset":3904,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1535,"record_offset":4660,"parent_record_offset":4624,"ancestor_record_offset":3904,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1535,"record_offset":4800,"parent_record_offset":4768,"ancestor_record_offset":3904,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1535,"record_offset":4768,"parent_record_offset":4732,"ancestor_record_offset":3904,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1535,"record_offset":4708,"parent_record_offset":4684,"ancestor_record_offset":3904,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1535,"record_offset":4732,"parent_record_offset":4708,"ancestor_record_offset":3904,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1535,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":3904,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1535,"record_offset":5272,"parent_record_offset":4732,"ancestor_record_offset":3904,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1535,"record_offset":4336,"parent_record_offset":4304,"ancestor_record_offset":3904,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1535,"record_offset":4304,"parent_record_offset":4276,"ancestor_record_offset":3904,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1535,"record_offset":4464,"parent_record_offset":4432,"ancestor_record_offset":3904,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1535,"record_offset":4520,"parent_record_offset":4464,"ancestor_record_offset":3904,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1535,"record_offset":4432,"parent_record_offset":4400,"ancestor_record_offset":3904,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1535,"record_offset":4276,"parent_record_offset":4232,"ancestor_record_offset":3904,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1535,"record_offset":4544,"parent_record_offset":4520,"ancestor_record_offset":3904,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1535,"record_offset":4832,"parent_record_offset":4800,"ancestor_record_offset":3904,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1535,"record_offset":5116,"parent_record_offset":4832,"ancestor_record_offset":3904,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1535,"record_offset":5088,"parent_record_offset":4832,"ancestor_record_offset":3904,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1535,"record_offset":4892,"parent_record_offset":4832,"ancestor_record_offset":3904,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1535,"record_offset":17812,"parent_record_offset":17756,"ancestor_record_offset":3904,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1535,"record_offset":12316,"parent_record_offset":12292,"ancestor_record_offset":3904,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1535,"record_offset":12340,"parent_record_offset":12316,"ancestor_record_offset":3904,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1535,"record_offset":12488,"parent_record_offset":12316,"ancestor_record_offset":3904,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1535,"record_offset":17756,"parent_record_offset":17732,"ancestor_record_offset":3904,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1535,"record_offset":7144,"parent_record_offset":7112,"ancestor_record_offset":3904,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1535,"record_offset":7176,"parent_record_offset":7144,"ancestor_record_offset":3904,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1535,"record_offset":7264,"parent_record_offset":7112,"ancestor_record_offset":3904,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1535,"record_offset":7112,"parent_record_offset":7072,"ancestor_record_offset":3904,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1535,"record_offset":6656,"parent_record_offset":6628,"ancestor_record_offset":3904,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1535,"record_offset":6520,"parent_record_offset":6496,"ancestor_record_offset":3904,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1535,"record_offset":7004,"parent_record_offset":6976,"ancestor_record_offset":3904,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1535,"record_offset":7352,"parent_record_offset":7072,"ancestor_record_offset":3904,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1535,"record_offset":6628,"parent_record_offset":6568,"ancestor_record_offset":3904,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1535,"record_offset":7072,"parent_record_offset":7032,"ancestor_record_offset":3904,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1535,"record_offset":6544,"parent_record_offset":6520,"ancestor_record_offset":3904,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1535,"record_offset":6600,"parent_record_offset":6568,"ancestor_record_offset":3904,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1535,"record_offset":6568,"parent_record_offset":6544,"ancestor_record_offset":3904,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1535,"record_offset":7032,"parent_record_offset":7004,"ancestor_record_offset":3904,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1535,"record_offset":11164,"parent_record_offset":10784,"ancestor_record_offset":3904,"inlinee":7465,"source_line":388,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1535,"record_offset":10784,"parent_record_offset":10744,"ancestor_record_offset":3904,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1535,"record_offset":11392,"parent_record_offset":11344,"ancestor_record_offset":3904,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1535,"record_offset":5672,"parent_record_offset":5636,"ancestor_record_offset":3904,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1535,"record_offset":18104,"parent_record_offset":18072,"ancestor_record_offset":3904,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1535,"record_offset":18556,"parent_record_offset":18352,"ancestor_record_offset":3904,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1535,"record_offset":5812,"parent_record_offset":5784,"ancestor_record_offset":3904,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1535,"record_offset":5884,"parent_record_offset":5744,"ancestor_record_offset":3904,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1535,"record_offset":6088,"parent_record_offset":5884,"ancestor_record_offset":3904,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1535,"record_offset":6496,"parent_record_offset":6472,"ancestor_record_offset":3904,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1535,"record_offset":17844,"parent_record_offset":17812,"ancestor_record_offset":3904,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1535,"record_offset":5840,"parent_record_offset":5812,"ancestor_record_offset":3904,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1535,"record_offset":5744,"parent_record_offset":5708,"ancestor_record_offset":3904,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1535,"record_offset":5708,"parent_record_offset":5672,"ancestor_record_offset":3904,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1535,"record_offset":8808,"parent_record_offset":8784,"ancestor_record_offset":3904,"inlinee":11236,"source_line":154,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1535,"record_offset":12012,"parent_record_offset":11984,"ancestor_record_offset":3904,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1535,"record_offset":12044,"parent_record_offset":12012,"ancestor_record_offset":3904,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1535,"record_offset":12168,"parent_record_offset":12012,"ancestor_record_offset":3904,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1535,"record_offset":11828,"parent_record_offset":11800,"ancestor_record_offset":3904,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1535,"record_offset":12764,"parent_record_offset":12740,"ancestor_record_offset":3904,"inlinee":27120,"source_line":41,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1535,"record_offset":11476,"parent_record_offset":11344,"ancestor_record_offset":3904,"inlinee":26127,"source_line":31,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1535,"record_offset":8784,"parent_record_offset":8720,"ancestor_record_offset":3904,"inlinee":26128,"source_line":90,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1535,"record_offset":11800,"parent_record_offset":11772,"ancestor_record_offset":3904,"inlinee":27128,"source_line":115,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1535,"record_offset":12292,"parent_record_offset":12012,"ancestor_record_offset":3904,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/01535.jsonl.gz"}
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
// 首条位置: {"module":1535,"record_offset":280,"rva":21548992,"va":5390258112}
// 编译实例 RVAs: 0x148cfc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-14580152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258112,"raw_hex":"e8488621ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390258851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26092431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258112,"raw_hex":"e88f238e01","size":5,"soft_fail":false,"target_va":5416351056,"va":5390258620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9061125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258112,"raw_hex":"e8fbbc75ff","size":5,"soft_fail":false,"target_va":5381197472,"va":5390258592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9355933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390258112,"raw_hex":"e8633d71ff","size":5,"soft_fail":false,"target_va":5380902448,"va":5390258376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":504,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":1344,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1535,"record_offset":1316,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1535,"record_offset":1568,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1535,"record_offset":3316,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1535,"record_offset":3288,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1535,"record_offset":560,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1535,"record_offset":852,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1535,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1535,"record_offset":3528,"parent_record_offset":3504,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1535,"record_offset":3552,"parent_record_offset":3528,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1535,"record_offset":3480,"parent_record_offset":3388,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":1444,"parent_record_offset":1412,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":3416,"parent_record_offset":3388,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":1492,"parent_record_offset":1468,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":1468,"parent_record_offset":1444,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":3364,"parent_record_offset":3340,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":1412,"parent_record_offset":1372,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":280,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":2172,"parent_record_offset":2140,"ancestor_record_offset":280,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":1372,"parent_record_offset":1344,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":2140,"parent_record_offset":2116,"ancestor_record_offset":280,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":1535,"record_offset":1928,"parent_record_offset":1644,"ancestor_record_offset":280,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1535,"record_offset":1692,"parent_record_offset":1644,"ancestor_record_offset":280,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1535,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":280,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1535,"record_offset":1832,"parent_record_offset":1800,"ancestor_record_offset":280,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1535,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":280,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1535,"record_offset":2904,"parent_record_offset":2880,"ancestor_record_offset":280,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1535,"record_offset":2348,"parent_record_offset":1644,"ancestor_record_offset":280,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1535,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":280,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1535,"record_offset":2880,"parent_record_offset":1644,"ancestor_record_offset":280,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1535,"record_offset":2928,"parent_record_offset":2904,"ancestor_record_offset":280,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":1535,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":280,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1535,"record_offset":2472,"parent_record_offset":2448,"ancestor_record_offset":280,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1535,"record_offset":2448,"parent_record_offset":2424,"ancestor_record_offset":280,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1535,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":280,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1535,"record_offset":2424,"parent_record_offset":2396,"ancestor_record_offset":280,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1535,"record_offset":2396,"parent_record_offset":2372,"ancestor_record_offset":280,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1535,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":280,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1535,"record_offset":2116,"parent_record_offset":1644,"ancestor_record_offset":280,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":1535,"record_offset":2804,"parent_record_offset":2780,"ancestor_record_offset":280,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1535,"record_offset":2780,"parent_record_offset":2756,"ancestor_record_offset":280,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1535,"record_offset":1620,"parent_record_offset":1596,"ancestor_record_offset":280,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1535,"record_offset":1644,"parent_record_offset":1620,"ancestor_record_offset":280,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":1535,"record_offset":1964,"parent_record_offset":1928,"ancestor_record_offset":280,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1535,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":280,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1535,"record_offset":3448,"parent_record_offset":3416,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1535,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":280,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1535,"record_offset":2564,"parent_record_offset":2500,"ancestor_record_offset":280,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1535,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":280,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1535,"record_offset":1800,"parent_record_offset":1716,"ancestor_record_offset":280,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1535,"record_offset":2628,"parent_record_offset":2448,"ancestor_record_offset":280,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1535,"record_offset":1900,"parent_record_offset":1644,"ancestor_record_offset":280,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1535,"record_offset":804,"parent_record_offset":720,"ancestor_record_offset":280,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1535,"record_offset":3340,"parent_record_offset":3316,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1535,"record_offset":2068,"parent_record_offset":2036,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1535,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":280,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1535,"record_offset":1532,"parent_record_offset":1372,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1535,"record_offset":1744,"parent_record_offset":1716,"ancestor_record_offset":280,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1535,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":280,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1535,"record_offset":2708,"parent_record_offset":2676,"ancestor_record_offset":280,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1535,"record_offset":2012,"parent_record_offset":1988,"ancestor_record_offset":280,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1535,"record_offset":2036,"parent_record_offset":2012,"ancestor_record_offset":280,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1535,"record_offset":1988,"parent_record_offset":1964,"ancestor_record_offset":280,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1535,"record_offset":1008,"parent_record_offset":980,"ancestor_record_offset":280,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1535,"record_offset":1040,"parent_record_offset":1008,"ancestor_record_offset":280,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1535,"record_offset":980,"parent_record_offset":940,"ancestor_record_offset":280,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1535,"record_offset":748,"parent_record_offset":720,"ancestor_record_offset":280,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1535,"record_offset":592,"parent_record_offset":560,"ancestor_record_offset":280,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1535,"record_offset":880,"parent_record_offset":852,"ancestor_record_offset":280,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1535,"record_offset":1160,"parent_record_offset":940,"ancestor_record_offset":280,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1535,"record_offset":720,"parent_record_offset":656,"ancestor_record_offset":280,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1535,"record_offset":940,"parent_record_offset":908,"ancestor_record_offset":280,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1535,"record_offset":624,"parent_record_offset":592,"ancestor_record_offset":280,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1535,"record_offset":692,"parent_record_offset":656,"ancestor_record_offset":280,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1535,"record_offset":656,"parent_record_offset":624,"ancestor_record_offset":280,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1535,"record_offset":908,"parent_record_offset":880,"ancestor_record_offset":280,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1535,"record_offset":2984,"parent_record_offset":2952,"ancestor_record_offset":280,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1535,"record_offset":2676,"parent_record_offset":2652,"ancestor_record_offset":280,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::find_overlap_segments::impl$0::new::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs
//     evidence={"module":1535,"record_offset":1596,"parent_record_offset":1568,"ancestor_record_offset":280,"inlinee":80104,"source_line":43,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::impl$0::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x138E8
// 首条位置: {"module":1535,"inlinee":80104}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1535,"record_offset":1620,"parent_record_offset":1596,"ancestor_record_offset":1596,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":1596,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":2268,"parent_record_offset":2244,"ancestor_record_offset":1596,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1535,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":1596,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":1596,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":2172,"parent_record_offset":2140,"ancestor_record_offset":1596,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1535,"record_offset":2140,"parent_record_offset":2116,"ancestor_record_offset":1596,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":1535,"record_offset":1928,"parent_record_offset":1644,"ancestor_record_offset":1596,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1535,"record_offset":1692,"parent_record_offset":1644,"ancestor_record_offset":1596,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1535,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":1596,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1535,"record_offset":1832,"parent_record_offset":1800,"ancestor_record_offset":1596,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":1535,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":1596,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1535,"record_offset":2904,"parent_record_offset":2880,"ancestor_record_offset":1596,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1535,"record_offset":2348,"parent_record_offset":1644,"ancestor_record_offset":1596,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1535,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":1596,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1535,"record_offset":2880,"parent_record_offset":1644,"ancestor_record_offset":1596,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":1535,"record_offset":2928,"parent_record_offset":2904,"ancestor_record_offset":1596,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":1535,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":1596,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1535,"record_offset":2472,"parent_record_offset":2448,"ancestor_record_offset":1596,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1535,"record_offset":2448,"parent_record_offset":2424,"ancestor_record_offset":1596,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":1535,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":1596,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1535,"record_offset":2424,"parent_record_offset":2396,"ancestor_record_offset":1596,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1535,"record_offset":2396,"parent_record_offset":2372,"ancestor_record_offset":1596,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1535,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":1596,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1535,"record_offset":2116,"parent_record_offset":1644,"ancestor_record_offset":1596,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":1535,"record_offset":2804,"parent_record_offset":2780,"ancestor_record_offset":1596,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":1535,"record_offset":2780,"parent_record_offset":2756,"ancestor_record_offset":1596,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":1535,"record_offset":1644,"parent_record_offset":1620,"ancestor_record_offset":1596,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":1535,"record_offset":1964,"parent_record_offset":1928,"ancestor_record_offset":1596,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1535,"record_offset":2564,"parent_record_offset":2500,"ancestor_record_offset":1596,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1535,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":1596,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1535,"record_offset":1800,"parent_record_offset":1716,"ancestor_record_offset":1596,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1535,"record_offset":2628,"parent_record_offset":2448,"ancestor_record_offset":1596,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1535,"record_offset":1900,"parent_record_offset":1644,"ancestor_record_offset":1596,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1535,"record_offset":2068,"parent_record_offset":2036,"ancestor_record_offset":1596,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1535,"record_offset":1744,"parent_record_offset":1716,"ancestor_record_offset":1596,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1535,"record_offset":1768,"parent_record_offset":1744,"ancestor_record_offset":1596,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1535,"record_offset":2708,"parent_record_offset":2676,"ancestor_record_offset":1596,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1535,"record_offset":2012,"parent_record_offset":1988,"ancestor_record_offset":1596,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1535,"record_offset":2036,"parent_record_offset":2012,"ancestor_record_offset":1596,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1535,"record_offset":1988,"parent_record_offset":1964,"ancestor_record_offset":1596,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1535,"record_offset":2984,"parent_record_offset":2952,"ancestor_record_offset":1596,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1535,"record_offset":2676,"parent_record_offset":2652,"ancestor_record_offset":1596,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/01535.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13611
// 首条位置: {"module":1455,"inlinee":79377}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1455,"record_offset":169400,"parent_record_offset":169376,"ancestor_record_offset":169376,"inlinee":26089,"source_line":207,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1455,"record_offset":169424,"parent_record_offset":169400,"ancestor_record_offset":169376,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=46
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137FE
// 首条位置: {"module":1500,"inlinee":79870}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1500,"record_offset":18348,"parent_record_offset":18280,"ancestor_record_offset":18280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01500.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=53
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1500,"record_offset":18300,"parent_record_offset":18280,"ancestor_record_offset":18280,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/01500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 327
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x136C9
// 首条位置: {"module":1474,"inlinee":79561}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<f32>,ref$<f32>)
// 候选参数类型: ["ref$<f32>", "ref$<f32>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>,ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>)
// 候选参数类型: ["ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>", "ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache::new
// 信息 ID: aef8cbaf8bd5de32f83830ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x8382bd -> 0x148cfc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments
// 信息 ID: 80af51f041ef2da3ff8d323f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x838818 -> 0x148d2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x838917 -> 0x148d2e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$0
// 信息 ID: 4f6b9aa35b1a9f03ff116237
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$0> >; module=1455, inline_record=169376; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$1
// 信息 ID: 020e503c320bad7454becc95
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::shared::smallsort::sort4_stable<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::stable::driftsort_main<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>,alloc::vec::Vec<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::stable::drift::sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::stable::quicksort::quicksort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::shared::pivot::median3_rec<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 40 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1500, inline_record=18280; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1500, inline_record=18428; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1500, inline_record=18592; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1500, inline_record=18764; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1500, inline_record=19048; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1500, inline_record=19440; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1500, inline_record=89492; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1500, inline_record=89692; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$3
// 信息 ID: 54fb2889bf1c68573f84e641
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3 <- itertools::tuple_impl::impl$10::next<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> >,tuple$<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > > >; module=1474, inline_record=3260; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > > >; module=1474, inline_record=3604; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > > >; module=1474, inline_record=4252; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > > >; module=1474, inline_record=4592; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$10::next<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> >,tuple$<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >; module=1504, inline_record=19644; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$10::next<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> >,tuple$<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >; module=1504, inline_record=20012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::impl$0::new::closure$0
// 信息 ID: 8bf772ab74829fe97c944d9d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::find_overlap_segments::impl$0::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache::new; module=1535, inline_record=1596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
