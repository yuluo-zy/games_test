// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/circle_turret.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a93523fa2d3b13cd7a0f3bb2d320d06822d2ed26fd749683ff6888b78f744178
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: circle_turret.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1480
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
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
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
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret
// 记录形式: compiled_procedure; instances=1
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1480,"record_offset":2052,"rva":19188784,"va":5387897904}
// 编译实例 RVAs: 0x124cc30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18632048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e890b2e3fe","size":5,"soft_fail":false,"target_va":5369267664,"va":5387899707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d24f0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13090961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e86f3f38ff","size":5,"soft_fail":false,"target_va":5374813424,"va":5387904380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dec30
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13040463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8b10439ff","size":5,"soft_fail":false,"target_va":5374864432,"va":5387904890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12819047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e899653cff","size":5,"soft_fail":false,"target_va":5375081488,"va":5387900530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12820402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e84e603cff","size":5,"soft_fail":false,"target_va":5375081504,"va":5387901901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140858790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10442162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e84eaa60ff","size":5,"soft_fail":false,"target_va":5377460112,"va":5387902269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408587c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10442126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e872aa60ff","size":5,"soft_fail":false,"target_va":5377460160,"va":5387902281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089d560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10160086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e82af864ff","size":5,"soft_fail":false,"target_va":5377742176,"va":5387902257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5670
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9993633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e85f8267ff","size":5,"soft_fail":false,"target_va":5377906288,"va":5387899916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ad2950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7841711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8515888ff","size":5,"soft_fail":false,"target_va":5380057424,"va":5387899130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5988178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8aea0a4ff","size":5,"soft_fail":false,"target_va":5381911904,"va":5387900077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5967116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8f4f2a4ff","size":5,"soft_fail":false,"target_va":5381934528,"va":5387901639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5965614","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8d2f8a4ff","size":5,"soft_fail":false,"target_va":5381934560,"va":5387900169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f8c10
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-345371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8e5bafaff","size":5,"soft_fail":false,"target_va":5387553808,"va":5387899174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141209c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-275960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e808cafbff","size":5,"soft_fail":false,"target_va":5387623488,"va":5387899443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14120c120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-269544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e818e3fbff","size":5,"soft_fail":false,"target_va":5387632928,"va":5387902467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23947628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e86c696d01","size":5,"soft_fail":false,"target_va":5411847152,"va":5387899519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23947402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e88a686d01","size":5,"soft_fail":false,"target_va":5411847776,"va":5387900369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f70
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23948090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e83a6b6d01","size":5,"soft_fail":false,"target_va":5411848048,"va":5387899953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-13543200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8e05831ff","size":5,"soft_fail":false,"target_va":5374356144,"va":5387899339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23637504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e800ae6801","size":5,"soft_fail":false,"target_va":5411540963,"va":5387903454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23638453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8b5b16801","size":5,"soft_fail":false,"target_va":5411541584,"va":5387903126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23638408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e888b16801","size":5,"soft_fail":false,"target_va":5411541776,"va":5387903363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23579247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e86fca6701","size":5,"soft_fail":false,"target_va":5411482433,"va":5387903181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23639154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e872b46801","size":5,"soft_fail":false,"target_va":5411542064,"va":5387902905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t23639022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8eeb36801","size":5,"soft_fail":false,"target_va":5411542296,"va":5387903269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23639023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8efb36801","size":5,"soft_fail":false,"target_va":5411541904,"va":5387902876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e830fbffff","size":5,"soft_fail":false,"target_va":5387896832,"va":5387898059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23645219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e823cc6801","size":5,"soft_fail":false,"target_va":5411543360,"va":5387898136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_curved_xy
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9166529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e83f2174ff","size":5,"soft_fail":false,"target_va":5378733712,"va":5387900236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7482117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8fbd48dff","size":5,"soft_fail":false,"target_va":5380416976,"va":5387899088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8106438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e83a4e84ff","size":5,"soft_fail":false,"target_va":5379792624,"va":5387899057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23589149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e81df16701","size":5,"soft_fail":false,"target_va":5411492368,"va":5387903214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23591549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e87dfa6701","size":5,"soft_fail":false,"target_va":5411494800,"va":5387903246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6076032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e88049a3ff","size":5,"soft_fail":false,"target_va":5381823600,"va":5387899627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6078540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8b43fa3ff","size":5,"soft_fail":false,"target_va":5381824016,"va":5387902551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6513037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8739e9cff","size":5,"soft_fail":false,"target_va":5381385536,"va":5387898568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6507848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8b8b29cff","size":5,"soft_fail":false,"target_va":5381390640,"va":5387898483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6516167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e839929cff","size":5,"soft_fail":false,"target_va":5381386480,"va":5387902642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6513341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8439d9cff","size":5,"soft_fail":false,"target_va":5381384704,"va":5387898040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23737699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e863356a01","size":5,"soft_fail":false,"target_va":5411640768,"va":5387903064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-339489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8dfd1faff","size":5,"soft_fail":false,"target_va":5387564432,"va":5387903916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23633985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e841a06801","size":5,"soft_fail":false,"target_va":5411537104,"va":5387903114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-55391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8a127ffff","size":5,"soft_fail":false,"target_va":5387845824,"va":5387901210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-54815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8e129ffff","size":5,"soft_fail":false,"target_va":5387846384,"va":5387901194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-54816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8e029ffff","size":5,"soft_fail":false,"target_va":5387846368,"va":5387901179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5986506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e836a7a4ff","size":5,"soft_fail":false,"target_va":5381913840,"va":5387900341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5987410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e8aea3a4ff","size":5,"soft_fail":false,"target_va":5381912944,"va":5387900349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6018712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387897904,"raw_hex":"e86829a4ff","size":5,"soft_fail":false,"target_va":5381881936,"va":5387900643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5387897904,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5387899557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1480,"record_offset":6964,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1480,"record_offset":4776,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1480,"record_offset":7712,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1480,"record_offset":8452,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1480,"record_offset":4860,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1480,"record_offset":13724,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":1480,"record_offset":6316,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1480,"record_offset":11104,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1480,"record_offset":5900,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1480,"record_offset":4832,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1480,"record_offset":7904,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1480,"record_offset":6252,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1480,"record_offset":11684,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1480,"record_offset":8856,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1480,"record_offset":5120,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1480,"record_offset":6992,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1480,"record_offset":7272,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":12277,"source_line":158,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1480,"record_offset":11600,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1480,"record_offset":5748,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1480,"record_offset":5932,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1480,"record_offset":11244,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1480,"record_offset":11516,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1480,"record_offset":10992,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1480,"record_offset":11188,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1480,"record_offset":11048,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1480,"record_offset":9136,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4536,"source_line":193,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1480,"record_offset":8016,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4713,"source_line":171,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1480,"record_offset":3792,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1480,"record_offset":2360,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1480,"record_offset":2816,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1480,"record_offset":8424,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":13632,"source_line":70,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1480,"record_offset":7988,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":11915,"source_line":339,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/circle_turret.rs
//     evidence={"module":1480,"record_offset":13908,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":71269,"source_line":207,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1480,"record_offset":5216,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1480,"record_offset":9736,"parent_record_offset":2052,"ancestor_record_offset":2052,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1480,"record_offset":8776,"parent_record_offset":8752,"ancestor_record_offset":2052,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1480,"record_offset":8800,"parent_record_offset":8776,"ancestor_record_offset":2052,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1480,"record_offset":6892,"parent_record_offset":6868,"ancestor_record_offset":2052,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1480,"record_offset":8752,"parent_record_offset":8576,"ancestor_record_offset":2052,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1480,"record_offset":6868,"parent_record_offset":6780,"ancestor_record_offset":2052,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1480,"record_offset":4964,"parent_record_offset":4940,"ancestor_record_offset":2052,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1480,"record_offset":8516,"parent_record_offset":8484,"ancestor_record_offset":2052,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1480,"record_offset":6808,"parent_record_offset":6780,"ancestor_record_offset":2052,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1480,"record_offset":6780,"parent_record_offset":6756,"ancestor_record_offset":2052,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1480,"record_offset":5012,"parent_record_offset":4988,"ancestor_record_offset":2052,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1480,"record_offset":4988,"parent_record_offset":4964,"ancestor_record_offset":2052,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1480,"record_offset":8576,"parent_record_offset":8544,"ancestor_record_offset":2052,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1480,"record_offset":8544,"parent_record_offset":8516,"ancestor_record_offset":2052,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1480,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":2052,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1480,"record_offset":8612,"parent_record_offset":8576,"ancestor_record_offset":2052,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1480,"record_offset":6512,"parent_record_offset":6460,"ancestor_record_offset":2052,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1480,"record_offset":4940,"parent_record_offset":4916,"ancestor_record_offset":2052,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1480,"record_offset":4916,"parent_record_offset":4884,"ancestor_record_offset":2052,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1480,"record_offset":6460,"parent_record_offset":6416,"ancestor_record_offset":2052,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1480,"record_offset":8484,"parent_record_offset":8452,"ancestor_record_offset":2052,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1480,"record_offset":5520,"parent_record_offset":5252,"ancestor_record_offset":2052,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1480,"record_offset":4884,"parent_record_offset":4860,"ancestor_record_offset":2052,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1480,"record_offset":13960,"parent_record_offset":13936,"ancestor_record_offset":2052,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1480,"record_offset":14008,"parent_record_offset":13984,"ancestor_record_offset":2052,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1480,"record_offset":6388,"parent_record_offset":6348,"ancestor_record_offset":2052,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1480,"record_offset":6348,"parent_record_offset":6316,"ancestor_record_offset":2052,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":1480,"record_offset":6416,"parent_record_offset":6348,"ancestor_record_offset":2052,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1480,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":2052,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1480,"record_offset":8660,"parent_record_offset":8636,"ancestor_record_offset":2052,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1480,"record_offset":4128,"parent_record_offset":4100,"ancestor_record_offset":2052,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1480,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":2052,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1480,"record_offset":4156,"parent_record_offset":4128,"ancestor_record_offset":2052,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1480,"record_offset":4076,"parent_record_offset":4048,"ancestor_record_offset":2052,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1480,"record_offset":4100,"parent_record_offset":4076,"ancestor_record_offset":2052,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1480,"record_offset":4272,"parent_record_offset":4100,"ancestor_record_offset":2052,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1480,"record_offset":4384,"parent_record_offset":4360,"ancestor_record_offset":2052,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1480,"record_offset":9824,"parent_record_offset":9800,"ancestor_record_offset":2052,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1480,"record_offset":9848,"parent_record_offset":9824,"ancestor_record_offset":2052,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1480,"record_offset":7952,"parent_record_offset":7928,"ancestor_record_offset":2052,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1480,"record_offset":7064,"parent_record_offset":7024,"ancestor_record_offset":2052,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1480,"record_offset":11652,"parent_record_offset":11600,"ancestor_record_offset":2052,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1480,"record_offset":11624,"parent_record_offset":11600,"ancestor_record_offset":2052,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1480,"record_offset":9888,"parent_record_offset":9736,"ancestor_record_offset":2052,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1480,"record_offset":9916,"parent_record_offset":9736,"ancestor_record_offset":2052,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1480,"record_offset":9384,"parent_record_offset":9356,"ancestor_record_offset":2052,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1480,"record_offset":9440,"parent_record_offset":9412,"ancestor_record_offset":2052,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1480,"record_offset":9172,"parent_record_offset":9136,"ancestor_record_offset":2052,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1480,"record_offset":7928,"parent_record_offset":7904,"ancestor_record_offset":2052,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1480,"record_offset":9468,"parent_record_offset":9440,"ancestor_record_offset":2052,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1480,"record_offset":4204,"parent_record_offset":4180,"ancestor_record_offset":2052,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1480,"record_offset":8684,"parent_record_offset":8660,"ancestor_record_offset":2052,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1480,"record_offset":7636,"parent_record_offset":7608,"ancestor_record_offset":2052,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1480,"record_offset":8708,"parent_record_offset":8684,"ancestor_record_offset":2052,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1480,"record_offset":6836,"parent_record_offset":6808,"ancestor_record_offset":2052,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1480,"record_offset":2516,"parent_record_offset":2484,"ancestor_record_offset":2052,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1480,"record_offset":5452,"parent_record_offset":5428,"ancestor_record_offset":2052,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1480,"record_offset":5368,"parent_record_offset":5344,"ancestor_record_offset":2052,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1480,"record_offset":11072,"parent_record_offset":11048,"ancestor_record_offset":2052,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1480,"record_offset":5800,"parent_record_offset":5772,"ancestor_record_offset":2052,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1480,"record_offset":3944,"parent_record_offset":3900,"ancestor_record_offset":2052,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1480,"record_offset":9800,"parent_record_offset":9776,"ancestor_record_offset":2052,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1480,"record_offset":9776,"parent_record_offset":9736,"ancestor_record_offset":2052,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1480,"record_offset":7540,"parent_record_offset":7272,"ancestor_record_offset":2052,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1480,"record_offset":9412,"parent_record_offset":9384,"ancestor_record_offset":2052,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1480,"record_offset":7352,"parent_record_offset":7272,"ancestor_record_offset":2052,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1480,"record_offset":7576,"parent_record_offset":7272,"ancestor_record_offset":2052,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1480,"record_offset":9704,"parent_record_offset":9136,"ancestor_record_offset":2052,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1480,"record_offset":2764,"parent_record_offset":2736,"ancestor_record_offset":2052,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1480,"record_offset":4732,"parent_record_offset":3900,"ancestor_record_offset":2052,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1480,"record_offset":4336,"parent_record_offset":4048,"ancestor_record_offset":2052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1480,"record_offset":5080,"parent_record_offset":5056,"ancestor_record_offset":2052,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1480,"record_offset":9312,"parent_record_offset":9288,"ancestor_record_offset":2052,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1480,"record_offset":3228,"parent_record_offset":3204,"ancestor_record_offset":2052,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1480,"record_offset":13792,"parent_record_offset":13724,"ancestor_record_offset":2052,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1480,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":2052,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1480,"record_offset":3412,"parent_record_offset":3384,"ancestor_record_offset":2052,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1480,"record_offset":13816,"parent_record_offset":13792,"ancestor_record_offset":2052,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1480,"record_offset":4228,"parent_record_offset":4204,"ancestor_record_offset":2052,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1480,"record_offset":7148,"parent_record_offset":7120,"ancestor_record_offset":2052,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1480,"record_offset":7176,"parent_record_offset":7148,"ancestor_record_offset":2052,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1480,"record_offset":7512,"parent_record_offset":7272,"ancestor_record_offset":2052,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1480,"record_offset":5548,"parent_record_offset":5520,"ancestor_record_offset":2052,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1480,"record_offset":5576,"parent_record_offset":5548,"ancestor_record_offset":2052,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1480,"record_offset":5056,"parent_record_offset":4884,"ancestor_record_offset":2052,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1480,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":2052,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1480,"record_offset":9288,"parent_record_offset":9264,"ancestor_record_offset":2052,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1480,"record_offset":3180,"parent_record_offset":3156,"ancestor_record_offset":2052,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1480,"record_offset":3484,"parent_record_offset":3456,"ancestor_record_offset":2052,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1480,"record_offset":9200,"parent_record_offset":9172,"ancestor_record_offset":2052,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1480,"record_offset":3132,"parent_record_offset":3016,"ancestor_record_offset":2052,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1480,"record_offset":9264,"parent_record_offset":9200,"ancestor_record_offset":2052,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1480,"record_offset":2484,"parent_record_offset":2456,"ancestor_record_offset":2052,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1480,"record_offset":2844,"parent_record_offset":2816,"ancestor_record_offset":2052,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1480,"record_offset":2984,"parent_record_offset":2952,"ancestor_record_offset":2052,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1480,"record_offset":2952,"parent_record_offset":2916,"ancestor_record_offset":2052,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1480,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":2052,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1480,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":2052,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1480,"record_offset":2868,"parent_record_offset":2844,"ancestor_record_offset":2052,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1480,"record_offset":3456,"parent_record_offset":2916,"ancestor_record_offset":2052,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1480,"record_offset":2456,"parent_record_offset":2424,"ancestor_record_offset":2052,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1480,"record_offset":2424,"parent_record_offset":2396,"ancestor_record_offset":2052,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1480,"record_offset":2616,"parent_record_offset":2584,"ancestor_record_offset":2052,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1480,"record_offset":2712,"parent_record_offset":2616,"ancestor_record_offset":2052,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1480,"record_offset":2584,"parent_record_offset":2560,"ancestor_record_offset":2052,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1480,"record_offset":7092,"parent_record_offset":7064,"ancestor_record_offset":2052,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1480,"record_offset":2396,"parent_record_offset":2360,"ancestor_record_offset":2052,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1480,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":2052,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1480,"record_offset":3016,"parent_record_offset":2984,"ancestor_record_offset":2052,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1480,"record_offset":3300,"parent_record_offset":3016,"ancestor_record_offset":2052,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1480,"record_offset":3272,"parent_record_offset":3016,"ancestor_record_offset":2052,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":1480,"record_offset":3076,"parent_record_offset":3016,"ancestor_record_offset":2052,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1480,"record_offset":14372,"parent_record_offset":14308,"ancestor_record_offset":2052,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1480,"record_offset":7424,"parent_record_offset":7388,"ancestor_record_offset":2052,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1480,"record_offset":14396,"parent_record_offset":14372,"ancestor_record_offset":2052,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1480,"record_offset":7448,"parent_record_offset":7424,"ancestor_record_offset":2052,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1480,"record_offset":7024,"parent_record_offset":6992,"ancestor_record_offset":2052,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1480,"record_offset":7608,"parent_record_offset":7576,"ancestor_record_offset":2052,"inlinee":12261,"source_line":175,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1480,"record_offset":14280,"parent_record_offset":14252,"ancestor_record_offset":2052,"inlinee":4939,"source_line":445,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1480,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":2052,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1480,"record_offset":11128,"parent_record_offset":11104,"ancestor_record_offset":2052,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1480,"record_offset":5828,"parent_record_offset":5772,"ancestor_record_offset":2052,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1480,"record_offset":11296,"parent_record_offset":11268,"ancestor_record_offset":2052,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1480,"record_offset":11268,"parent_record_offset":11244,"ancestor_record_offset":2052,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1480,"record_offset":5316,"parent_record_offset":5288,"ancestor_record_offset":2052,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1480,"record_offset":5344,"parent_record_offset":5316,"ancestor_record_offset":2052,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1480,"record_offset":5428,"parent_record_offset":5316,"ancestor_record_offset":2052,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1480,"record_offset":5772,"parent_record_offset":5748,"ancestor_record_offset":2052,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1480,"record_offset":9356,"parent_record_offset":9136,"ancestor_record_offset":2052,"inlinee":4568,"source_line":374,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1480,"record_offset":7388,"parent_record_offset":7272,"ancestor_record_offset":2052,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1480,"record_offset":14308,"parent_record_offset":14280,"ancestor_record_offset":2052,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1480,"record_offset":7480,"parent_record_offset":7388,"ancestor_record_offset":2052,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1480,"record_offset":14344,"parent_record_offset":14308,"ancestor_record_offset":2052,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1480,"record_offset":3828,"parent_record_offset":3792,"ancestor_record_offset":2052,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1480,"record_offset":12732,"parent_record_offset":12700,"ancestor_record_offset":2052,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1480,"record_offset":13248,"parent_record_offset":12988,"ancestor_record_offset":2052,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1480,"record_offset":3972,"parent_record_offset":3944,"ancestor_record_offset":2052,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1480,"record_offset":4048,"parent_record_offset":3900,"ancestor_record_offset":2052,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1480,"record_offset":4308,"parent_record_offset":4048,"ancestor_record_offset":2052,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1480,"record_offset":5860,"parent_record_offset":5828,"ancestor_record_offset":2052,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1480,"record_offset":4000,"parent_record_offset":3972,"ancestor_record_offset":2052,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1480,"record_offset":3900,"parent_record_offset":3864,"ancestor_record_offset":2052,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1480,"record_offset":3864,"parent_record_offset":3828,"ancestor_record_offset":2052,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1480,"record_offset":5252,"parent_record_offset":5216,"ancestor_record_offset":2052,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1480,"record_offset":5492,"parent_record_offset":5252,"ancestor_record_offset":2052,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1480,"record_offset":5616,"parent_record_offset":5252,"ancestor_record_offset":2052,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1480,"record_offset":5288,"parent_record_offset":5252,"ancestor_record_offset":2052,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/01480.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 207
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11665
// 首条位置: {"module":1480,"inlinee":71269}
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
// 完整结构及原始类型记录: circle_turret.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: circle_turret.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: circle_turret.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret
// 信息 ID: 975dea342e3c12c259f198d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_circle_supports @ 0x788658 -> 0x124cc30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret::closure$0
// 信息 ID: 9ff42a19a0ad917a862a754c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret; module=1480, inline_record=13908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
