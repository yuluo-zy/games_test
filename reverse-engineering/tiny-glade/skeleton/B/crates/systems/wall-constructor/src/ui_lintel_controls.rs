// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/ui_lintel_controls.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a56bc0e921f0c90ffb660f8618af9ddba5655afb6a5b643fabb13155088c890e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_lintel_controls.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 404, 1417
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
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
// 函数符号: system_wall_constructor::ui_lintel_controls::ui_lintel_controls
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":404,"record_offset":3668,"rva":7805536,"va":5376514656}
// 编译实例 RVAs: 0x771a60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1437185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8ff11eaff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376518668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1435834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e84617eaff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376517333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14085e430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t967482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e83ac30e00","size":5,"soft_fail":false,"target_va":5377483824,"va":5376516337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898470
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1200332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8cc501200","size":5,"soft_fail":false,"target_va":5377721456,"va":5376521119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14092df10
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1817413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e845bb1b00","size":5,"soft_fail":false,"target_va":5378334480,"va":5376517062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a82b20
//     owner=unresolved_namespace; count=2
//     target_aliases=20 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3212195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8a3033100","size":5,"soft_fail":false,"target_va":5379730208,"va":5376518008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14123b320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11308918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8768fac00","size":5,"soft_fail":false,"target_va":5387825952,"va":5376517029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35328437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8b5111b02","size":5,"soft_fail":false,"target_va":5411847168,"va":5376518726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35020781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8ed5f1602","size":5,"soft_fail":false,"target_va":5411540992,"va":5376520206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6323291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e85b7c6000","size":5,"soft_fail":false,"target_va":5382840208,"va":5376516912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4119032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8f8d93e00","size":5,"soft_fail":false,"target_va":5380635696,"va":5376516659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34820285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8bd501302","size":5,"soft_fail":false,"target_va":5411339744,"va":5376519454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35020691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8935f1602","size":5,"soft_fail":false,"target_va":5411540963,"va":5376520267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35021402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e85a621602","size":5,"soft_fail":false,"target_va":5411541584,"va":5376520177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t35021480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8a8621602","size":5,"soft_fail":false,"target_va":5411541776,"va":5376520291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35022057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8e9641602","size":5,"soft_fail":false,"target_va":5411542296,"va":5376520234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e80ffbffff","size":5,"soft_fail":false,"target_va":5376513536,"va":5376514796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35028482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8027e1602","size":5,"soft_fail":false,"target_va":5411543360,"va":5376514873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3221160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8a8263100","size":5,"soft_fail":false,"target_va":5379739328,"va":5376518163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::build
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3212645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e865053100","size":5,"soft_fail":false,"target_va":5379730768,"va":5376518118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3212169","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e889033100","size":5,"soft_fail":false,"target_va":5379730240,"va":5376518066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t950196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8b47f0e00","size":5,"soft_fail":false,"target_va":5377467744,"va":5376517543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_arch_height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t952090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e81a870e00","size":5,"soft_fail":false,"target_va":5377469728,"va":5376517633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_arch_type
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t951762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8d2850e00","size":5,"soft_fail":false,"target_va":5377469760,"va":5376517993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3276665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e879ff3100","size":5,"soft_fail":false,"target_va":5379792624,"va":5376515954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1819193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e839c21b00","size":5,"soft_fail":false,"target_va":5378336320,"va":5376517122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1817878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e816bd1b00","size":5,"soft_fail":false,"target_va":5378336384,"va":5376518501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2321020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e87c6a2300","size":5,"soft_fail":false,"target_va":5378838032,"va":5376517007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1223692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e80cac1200","size":5,"soft_fail":false,"target_va":5377743024,"va":5376519327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3908280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8b8a23b00","size":5,"soft_fail":false,"target_va":5380427664,"va":5376519379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3497774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e82e5f3500","size":5,"soft_fail":false,"target_va":5380015136,"va":5376517357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3497794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8425f3500","size":5,"soft_fail":false,"target_va":5380015184,"va":5376517385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3497765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8255f3500","size":5,"soft_fail":false,"target_va":5380015264,"va":5376517494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4870185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e829504a00","size":5,"soft_fail":false,"target_va":5381385536,"va":5376515346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4875374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e86e644a00","size":5,"soft_fail":false,"target_va":5381390640,"va":5376515261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4871016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e868534a00","size":5,"soft_fail":false,"target_va":5381386480,"va":5376515459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4869923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8234f4a00","size":5,"soft_fail":false,"target_va":5381384704,"va":5376514776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35120674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e822e61702","size":5,"soft_fail":false,"target_va":5411640768,"va":5376520089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11041252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8e479a800","size":5,"soft_fail":false,"target_va":5387564432,"va":5376523175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35016934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8e6501602","size":5,"soft_fail":false,"target_va":5411537104,"va":5376520165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::intersect::Triangle::new
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11307874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376514656,"raw_hex":"e8628bac00","size":5,"soft_fail":false,"target_va":5387824496,"va":5376516617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":404,"record_offset":10732,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":404,"record_offset":14656,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":10704,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":8352,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":9508,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":12144,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":14260,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":8380,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":13092,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":404,"record_offset":11252,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":404,"record_offset":11308,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":404,"record_offset":11668,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":404,"record_offset":11220,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":404,"record_offset":10816,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":404,"record_offset":12056,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":404,"record_offset":9052,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":404,"record_offset":11156,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":7576,"source_line":378,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":404,"record_offset":8296,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":404,"record_offset":10844,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":6356,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":404,"record_offset":11972,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":8640,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":404,"record_offset":10216,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":404,"record_offset":10272,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/lib.rs
//     evidence={"module":404,"record_offset":9452,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4532,"source_line":777,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":404,"record_offset":9700,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":5372,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":3936,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":4396,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":404,"record_offset":7216,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":404,"record_offset":7548,"parent_record_offset":3668,"ancestor_record_offset":3668,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":12604,"parent_record_offset":12580,"ancestor_record_offset":3668,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":12628,"parent_record_offset":12604,"ancestor_record_offset":3668,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":11108,"parent_record_offset":11080,"ancestor_record_offset":3668,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":14728,"parent_record_offset":14692,"ancestor_record_offset":3668,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":12580,"parent_record_offset":12404,"ancestor_record_offset":3668,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":11080,"parent_record_offset":10988,"ancestor_record_offset":3668,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":404,"record_offset":14692,"parent_record_offset":14656,"ancestor_record_offset":3668,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":404,"record_offset":10780,"parent_record_offset":10756,"ancestor_record_offset":3668,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":8484,"parent_record_offset":8460,"ancestor_record_offset":3668,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":13228,"parent_record_offset":13200,"ancestor_record_offset":3668,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":12344,"parent_record_offset":12316,"ancestor_record_offset":3668,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":11016,"parent_record_offset":10988,"ancestor_record_offset":3668,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":10988,"parent_record_offset":10964,"ancestor_record_offset":3668,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":13288,"parent_record_offset":13256,"ancestor_record_offset":3668,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":8532,"parent_record_offset":8508,"ancestor_record_offset":3668,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":8508,"parent_record_offset":8484,"ancestor_record_offset":3668,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":13256,"parent_record_offset":13228,"ancestor_record_offset":3668,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":12404,"parent_record_offset":12372,"ancestor_record_offset":3668,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":12372,"parent_record_offset":12344,"ancestor_record_offset":3668,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":10964,"parent_record_offset":10940,"ancestor_record_offset":3668,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":12440,"parent_record_offset":12404,"ancestor_record_offset":3668,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":404,"record_offset":12276,"parent_record_offset":12244,"ancestor_record_offset":3668,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":404,"record_offset":12244,"parent_record_offset":12176,"ancestor_record_offset":3668,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":9548,"parent_record_offset":9508,"ancestor_record_offset":3668,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":8460,"parent_record_offset":8436,"ancestor_record_offset":3668,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":8436,"parent_record_offset":8404,"ancestor_record_offset":3668,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":13156,"parent_record_offset":13124,"ancestor_record_offset":3668,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":11428,"parent_record_offset":11392,"ancestor_record_offset":3668,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":13200,"parent_record_offset":13156,"ancestor_record_offset":3668,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":12316,"parent_record_offset":12276,"ancestor_record_offset":3668,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":9076,"parent_record_offset":9052,"ancestor_record_offset":3668,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":13880,"parent_record_offset":13852,"ancestor_record_offset":3668,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":8404,"parent_record_offset":8380,"ancestor_record_offset":3668,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":404,"record_offset":14220,"parent_record_offset":13156,"ancestor_record_offset":3668,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":404,"record_offset":14048,"parent_record_offset":14024,"ancestor_record_offset":3668,"inlinee":6529,"source_line":30,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":404,"record_offset":13124,"parent_record_offset":13092,"ancestor_record_offset":3668,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":404,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":3668,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":404,"record_offset":11332,"parent_record_offset":11308,"ancestor_record_offset":3668,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":404,"record_offset":11392,"parent_record_offset":11332,"ancestor_record_offset":3668,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":404,"record_offset":12464,"parent_record_offset":12440,"ancestor_record_offset":3668,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":404,"record_offset":12488,"parent_record_offset":12464,"ancestor_record_offset":3668,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":404,"record_offset":5708,"parent_record_offset":5680,"ancestor_record_offset":3668,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":404,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":3668,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":404,"record_offset":5736,"parent_record_offset":5708,"ancestor_record_offset":3668,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":404,"record_offset":5656,"parent_record_offset":5628,"ancestor_record_offset":3668,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":404,"record_offset":5680,"parent_record_offset":5656,"ancestor_record_offset":3668,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":404,"record_offset":5852,"parent_record_offset":5680,"ancestor_record_offset":3668,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":404,"record_offset":5964,"parent_record_offset":5940,"ancestor_record_offset":3668,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::Clone::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\clone.rs
//     evidence={"module":404,"record_offset":10180,"parent_record_offset":10152,"ancestor_record_offset":3668,"inlinee":4549,"source_line":165,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":404,"record_offset":9408,"parent_record_offset":9384,"ancestor_record_offset":3668,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":404,"record_offset":9384,"parent_record_offset":9360,"ancestor_record_offset":3668,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":404,"record_offset":7404,"parent_record_offset":7376,"ancestor_record_offset":3668,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":404,"record_offset":7464,"parent_record_offset":7432,"ancestor_record_offset":3668,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":404,"record_offset":12704,"parent_record_offset":12680,"ancestor_record_offset":3668,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":404,"record_offset":10120,"parent_record_offset":10096,"ancestor_record_offset":3668,"inlinee":4545,"source_line":36,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$3::rfold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":404,"record_offset":13420,"parent_record_offset":13388,"ancestor_record_offset":3668,"inlinee":24600,"source_line":161,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":404,"record_offset":13592,"parent_record_offset":13452,"ancestor_record_offset":3668,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":404,"record_offset":13388,"parent_record_offset":13356,"ancestor_record_offset":3668,"inlinee":24599,"source_line":60,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":404,"record_offset":9336,"parent_record_offset":9052,"ancestor_record_offset":3668,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":404,"record_offset":9748,"parent_record_offset":9700,"ancestor_record_offset":3668,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":404,"record_offset":13504,"parent_record_offset":13452,"ancestor_record_offset":3668,"inlinee":24603,"source_line":125,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":404,"record_offset":9360,"parent_record_offset":9336,"ancestor_record_offset":3668,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":404,"record_offset":9772,"parent_record_offset":9748,"ancestor_record_offset":3668,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":404,"record_offset":13544,"parent_record_offset":13504,"ancestor_record_offset":3668,"inlinee":24604,"source_line":369,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::double_ended::DoubleEndedIterator::rfold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\double_ended.rs
//     evidence={"module":404,"record_offset":13452,"parent_record_offset":13420,"ancestor_record_offset":3668,"inlinee":24601,"source_line":301,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":404,"record_offset":10096,"parent_record_offset":9700,"ancestor_record_offset":3668,"inlinee":4543,"source_line":468,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":404,"record_offset":13356,"parent_record_offset":13156,"ancestor_record_offset":3668,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":404,"record_offset":13852,"parent_record_offset":13592,"ancestor_record_offset":3668,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":404,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":3668,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":404,"record_offset":12512,"parent_record_offset":12488,"ancestor_record_offset":3668,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":404,"record_offset":12536,"parent_record_offset":12512,"ancestor_record_offset":3668,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":404,"record_offset":11048,"parent_record_offset":11016,"ancestor_record_offset":3668,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":404,"record_offset":13312,"parent_record_offset":13288,"ancestor_record_offset":3668,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":404,"record_offset":4096,"parent_record_offset":4064,"ancestor_record_offset":3668,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":404,"record_offset":7820,"parent_record_offset":7788,"ancestor_record_offset":3668,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":404,"record_offset":7788,"parent_record_offset":7756,"ancestor_record_offset":3668,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":404,"record_offset":10304,"parent_record_offset":10272,"ancestor_record_offset":3668,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":404,"record_offset":10240,"parent_record_offset":10216,"ancestor_record_offset":3668,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":404,"record_offset":5524,"parent_record_offset":5480,"ancestor_record_offset":3668,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":404,"record_offset":8320,"parent_record_offset":8296,"ancestor_record_offset":3668,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":404,"record_offset":10756,"parent_record_offset":10732,"ancestor_record_offset":3668,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":404,"record_offset":10152,"parent_record_offset":9700,"ancestor_record_offset":3668,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":404,"record_offset":4344,"parent_record_offset":4316,"ancestor_record_offset":3668,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":404,"record_offset":6312,"parent_record_offset":5480,"ancestor_record_offset":3668,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":404,"record_offset":12680,"parent_record_offset":12276,"ancestor_record_offset":3668,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":5916,"parent_record_offset":5628,"ancestor_record_offset":3668,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":404,"record_offset":8600,"parent_record_offset":8576,"ancestor_record_offset":3668,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":404,"record_offset":8080,"parent_record_offset":8056,"ancestor_record_offset":3668,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":8752,"parent_record_offset":8728,"ancestor_record_offset":3668,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":3668,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":3668,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":404,"record_offset":4992,"parent_record_offset":4964,"ancestor_record_offset":3668,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":9976,"parent_record_offset":9948,"ancestor_record_offset":3668,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":14836,"parent_record_offset":14808,"ancestor_record_offset":3668,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":5808,"parent_record_offset":5784,"ancestor_record_offset":3668,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":404,"record_offset":14288,"parent_record_offset":14260,"ancestor_record_offset":3668,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":404,"record_offset":14316,"parent_record_offset":14288,"ancestor_record_offset":3668,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":404,"record_offset":8576,"parent_record_offset":8404,"ancestor_record_offset":3668,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":4736,"parent_record_offset":4712,"ancestor_record_offset":3668,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":8728,"parent_record_offset":8704,"ancestor_record_offset":3668,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":4760,"parent_record_offset":4736,"ancestor_record_offset":3668,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::__iterator_get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":9804,"parent_record_offset":9772,"ancestor_record_offset":3668,"inlinee":4540,"source_line":372,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":5064,"parent_record_offset":5036,"ancestor_record_offset":3668,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":12080,"parent_record_offset":12056,"ancestor_record_offset":3668,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":4712,"parent_record_offset":4596,"ancestor_record_offset":3668,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":404,"record_offset":8704,"parent_record_offset":8640,"ancestor_record_offset":3668,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":404,"record_offset":4064,"parent_record_offset":4036,"ancestor_record_offset":3668,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":404,"record_offset":4424,"parent_record_offset":4396,"ancestor_record_offset":3668,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":404,"record_offset":4564,"parent_record_offset":4532,"ancestor_record_offset":3668,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":404,"record_offset":4532,"parent_record_offset":4496,"ancestor_record_offset":3668,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":404,"record_offset":4472,"parent_record_offset":4448,"ancestor_record_offset":3668,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":404,"record_offset":4496,"parent_record_offset":4472,"ancestor_record_offset":3668,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":404,"record_offset":4448,"parent_record_offset":4424,"ancestor_record_offset":3668,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":404,"record_offset":5036,"parent_record_offset":4496,"ancestor_record_offset":3668,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":4036,"parent_record_offset":4000,"ancestor_record_offset":3668,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":4000,"parent_record_offset":3972,"ancestor_record_offset":3668,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":4196,"parent_record_offset":4164,"ancestor_record_offset":3668,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":4292,"parent_record_offset":4196,"ancestor_record_offset":3668,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3668,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":3972,"parent_record_offset":3936,"ancestor_record_offset":3668,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":404,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":3668,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":404,"record_offset":4596,"parent_record_offset":4564,"ancestor_record_offset":3668,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":404,"record_offset":4880,"parent_record_offset":4596,"ancestor_record_offset":3668,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":404,"record_offset":4852,"parent_record_offset":4596,"ancestor_record_offset":3668,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":404,"record_offset":4656,"parent_record_offset":4596,"ancestor_record_offset":3668,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":404,"record_offset":13648,"parent_record_offset":13620,"ancestor_record_offset":3668,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":404,"record_offset":13712,"parent_record_offset":13620,"ancestor_record_offset":3668,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":404,"record_offset":7716,"parent_record_offset":7676,"ancestor_record_offset":3668,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":404,"record_offset":7756,"parent_record_offset":7716,"ancestor_record_offset":3668,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":404,"record_offset":7864,"parent_record_offset":7676,"ancestor_record_offset":3668,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":404,"record_offset":7676,"parent_record_offset":7628,"ancestor_record_offset":3668,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":404,"record_offset":7376,"parent_record_offset":7348,"ancestor_record_offset":3668,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":404,"record_offset":7432,"parent_record_offset":7376,"ancestor_record_offset":3668,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":404,"record_offset":7956,"parent_record_offset":7628,"ancestor_record_offset":3668,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":404,"record_offset":7240,"parent_record_offset":7216,"ancestor_record_offset":3668,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":404,"record_offset":7572,"parent_record_offset":7548,"ancestor_record_offset":3668,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":404,"record_offset":8228,"parent_record_offset":8204,"ancestor_record_offset":3668,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":404,"record_offset":8204,"parent_record_offset":7572,"ancestor_record_offset":3668,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":404,"record_offset":8056,"parent_record_offset":7628,"ancestor_record_offset":3668,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":404,"record_offset":7348,"parent_record_offset":7288,"ancestor_record_offset":3668,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":404,"record_offset":7628,"parent_record_offset":7600,"ancestor_record_offset":3668,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":404,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":3668,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":404,"record_offset":7320,"parent_record_offset":7288,"ancestor_record_offset":3668,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":404,"record_offset":7288,"parent_record_offset":7264,"ancestor_record_offset":3668,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":404,"record_offset":7600,"parent_record_offset":7572,"ancestor_record_offset":3668,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":404,"record_offset":9924,"parent_record_offset":9700,"ancestor_record_offset":3668,"inlinee":4542,"source_line":388,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":404,"record_offset":9476,"parent_record_offset":9452,"ancestor_record_offset":3668,"inlinee":4534,"source_line":176,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":5408,"parent_record_offset":5372,"ancestor_record_offset":3668,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":6420,"parent_record_offset":6388,"ancestor_record_offset":3668,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":6936,"parent_record_offset":6676,"ancestor_record_offset":3668,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":404,"record_offset":5552,"parent_record_offset":5524,"ancestor_record_offset":3668,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":5628,"parent_record_offset":5480,"ancestor_record_offset":3668,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":404,"record_offset":5888,"parent_record_offset":5628,"ancestor_record_offset":3668,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":404,"record_offset":5580,"parent_record_offset":5552,"ancestor_record_offset":3668,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":404,"record_offset":5480,"parent_record_offset":5444,"ancestor_record_offset":3668,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":404,"record_offset":5444,"parent_record_offset":5408,"ancestor_record_offset":3668,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/ui_lintel_controls.rs
//     evidence={"module":404,"record_offset":13620,"parent_record_offset":13592,"ancestor_record_offset":3668,"inlinee":24668,"source_line":183,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x605C
// 首条位置: {"module":404,"inlinee":24668}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":404,"record_offset":13648,"parent_record_offset":13620,"ancestor_record_offset":13620,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":404,"record_offset":13712,"parent_record_offset":13620,"ancestor_record_offset":13620,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":404,"record_offset":13752,"parent_record_offset":13712,"ancestor_record_offset":13620,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":404,"record_offset":13676,"parent_record_offset":13648,"ancestor_record_offset":13620,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":404,"record_offset":17728,"rva":7770320,"va":5376479440}
// 编译实例 RVAs: 0x7690d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1398024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376479440,"raw_hex":"e8f8aaeaff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376479507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1398515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376479440,"raw_hex":"e80da9eaff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376480014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_lintel_controls.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35367511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376479440,"raw_hex":"e857aa1b02","size":5,"soft_fail":false,"target_va":5411847168,"va":5376479652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35061028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376479440,"raw_hex":"e824fd1602","size":5,"soft_fail":false,"target_va":5411540992,"va":5376479959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34859820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376479440,"raw_hex":"e82ceb1302","size":5,"soft_fail":false,"target_va":5411339744,"va":5376479919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35061016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376479440,"raw_hex":"e818fd1602","size":5,"soft_fail":false,"target_va":5411540963,"va":5376479942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1264304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376479440,"raw_hex":"e8b04a1300","size":5,"soft_fail":false,"target_va":5377744016,"va":5376479707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1264461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376479440,"raw_hex":"e84d4b1300","size":5,"soft_fail":false,"target_va":5377744208,"va":5376479742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1263584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376479440,"raw_hex":"e8e0471300","size":5,"soft_fail":false,"target_va":5377743280,"va":5376479691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":17904,"parent_record_offset":17728,"ancestor_record_offset":17728,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":404,"record_offset":18948,"parent_record_offset":17728,"ancestor_record_offset":17728,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":18052,"parent_record_offset":17728,"ancestor_record_offset":17728,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":404,"record_offset":18644,"parent_record_offset":17728,"ancestor_record_offset":17728,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":19032,"parent_record_offset":17728,"ancestor_record_offset":17728,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":17960,"parent_record_offset":17936,"ancestor_record_offset":17728,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":17984,"parent_record_offset":17960,"ancestor_record_offset":17728,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":19104,"parent_record_offset":19080,"ancestor_record_offset":17728,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":17936,"parent_record_offset":17904,"ancestor_record_offset":17728,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":404,"record_offset":19080,"parent_record_offset":19056,"ancestor_record_offset":17728,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":404,"record_offset":19056,"parent_record_offset":19032,"ancestor_record_offset":17728,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":18796,"parent_record_offset":18768,"ancestor_record_offset":17728,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":18236,"parent_record_offset":18208,"ancestor_record_offset":17728,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":18880,"parent_record_offset":18856,"ancestor_record_offset":17728,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":18856,"parent_record_offset":18824,"ancestor_record_offset":17728,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":18824,"parent_record_offset":18796,"ancestor_record_offset":17728,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":18296,"parent_record_offset":18264,"ancestor_record_offset":17728,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":18264,"parent_record_offset":18236,"ancestor_record_offset":17728,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":404,"record_offset":18332,"parent_record_offset":18296,"ancestor_record_offset":17728,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":404,"record_offset":18172,"parent_record_offset":18144,"ancestor_record_offset":17728,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":404,"record_offset":18972,"parent_record_offset":18948,"ancestor_record_offset":17728,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":404,"record_offset":18144,"parent_record_offset":18080,"ancestor_record_offset":17728,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":18116,"parent_record_offset":18080,"ancestor_record_offset":17728,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":18996,"parent_record_offset":18972,"ancestor_record_offset":17728,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":18768,"parent_record_offset":18732,"ancestor_record_offset":17728,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":18208,"parent_record_offset":18172,"ancestor_record_offset":17728,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":404,"record_offset":18080,"parent_record_offset":18052,"ancestor_record_offset":17728,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":404,"record_offset":18700,"parent_record_offset":18672,"ancestor_record_offset":17728,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":404,"record_offset":18732,"parent_record_offset":18700,"ancestor_record_offset":17728,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":404,"record_offset":18672,"parent_record_offset":18644,"ancestor_record_offset":17728,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":404,"record_offset":18356,"parent_record_offset":18332,"ancestor_record_offset":17728,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":404,"record_offset":18380,"parent_record_offset":18356,"ancestor_record_offset":17728,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":404,"record_offset":18596,"parent_record_offset":18572,"ancestor_record_offset":17728,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":404,"record_offset":18404,"parent_record_offset":18380,"ancestor_record_offset":17728,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":404,"record_offset":18428,"parent_record_offset":18404,"ancestor_record_offset":17728,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":404,"record_offset":18572,"parent_record_offset":18172,"ancestor_record_offset":17728,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00404.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":404,"record_offset":18008,"parent_record_offset":17984,"ancestor_record_offset":17728,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00404.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_lintel_controls.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_lintel_controls.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_lintel_controls.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::ui_lintel_controls::ui_lintel_controls
// 信息 ID: 291b741bd74354101c8e3d73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 232efeee8ef11b3a89a98b46: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>)> > > @ 0x465c69 -> 0x771a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::system_param::Local<system_wall_constructor::ui_arch_controls::ArchHeightControl>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>)> > @ 0x5af8f4 -> 0x771a60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure$0
// 信息 ID: 17618638fe42e85d24b557c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::ui_lintel_controls::ui_lintel_controls", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::ui_lintel_controls::ui_lintel_controls; module=404, inline_record=13620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure$1
// 信息 ID: 302ca29b985abbea9136f494
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::ui_lintel_controls::ui_lintel_controls", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure_env$1 <- core::ops::function::FnOnce::call_once<system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure_env$1 <- core::ptr::drop_in_place<system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_wall_constructor::ui_arch_controls::do_ui_arch_controls::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > | core::ops::function::FnOnce::call_once<system_wall_constructor::ui_lintel_controls::ui_lintel_controls::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x766963 -> 0x7690d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
