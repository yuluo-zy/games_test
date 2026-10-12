// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/construct_lintels/mod.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 367f6575e9254a618d923a0c1a4342be4b00ba199b08180737d528e10e04d1e1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 410, 1455, 1462, 1511
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
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
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
// 函数符号: system_wall_constructor::construct_lintels::construct_lintels
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":410,"record_offset":280,"rva":8456416,"va":5377165536}
// 编译实例 RVAs: 0x8108e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7437503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8bf7c7100","size":5,"soft_fail":false,"target_va":5384609408,"va":5377171900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-1489626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e82645e9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377168309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc_zeroed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1490072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e86843e9ff","size":5,"soft_fail":false,"target_va":5375678736,"va":5377168803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-1489974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8ca43e9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377168673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8446109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8631f7fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377168296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140082d10
//     owner=unresolved_namespace; count=4
//     target_aliases=5 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7922934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e80a1b87ff","size":5,"soft_fail":false,"target_va":5369244944,"va":5377167873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14019e7e0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6764523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e815c898ff","size":5,"soft_fail":false,"target_va":5370406880,"va":5377171398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=3
//     target_aliases=6 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3687813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e87bbac7ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5377166848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7ede0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2541102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e82ec62600","size":5,"soft_fail":false,"target_va":5379714528,"va":5377173421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbeb30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8052582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e866df7a00","size":5,"soft_fail":false,"target_va":5385218864,"va":5377166277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f11b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12453547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8ab06be00","size":5,"soft_fail":false,"target_va":5389619632,"va":5377166080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f4d30
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12467002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e83a3bbe00","size":5,"soft_fail":false,"target_va":5389634864,"va":5377167857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f5920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12469151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e89f43be00","size":5,"soft_fail":false,"target_va":5389637920,"va":5377168764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405490
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12530979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e82335bf00","size":5,"soft_fail":false,"target_va":5389702288,"va":5377171304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414054a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12531110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8a635bf00","size":5,"soft_fail":false,"target_va":5389702304,"va":5377171189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141413c80
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12590162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8521cc000","size":5,"soft_fail":false,"target_va":5389761664,"va":5377171497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14147a2d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13011881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8a98bc600","size":5,"soft_fail":false,"target_va":5390181072,"va":5377169186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39186583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e897f05502","size":5,"soft_fail":false,"target_va":5416353040,"va":5377166452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39888104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8e8a46002","size":5,"soft_fail":false,"target_va":5417057088,"va":5377168979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t39196231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e847165602","size":5,"soft_fail":false,"target_va":5416366080,"va":5377169844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39447860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e834ed5902","size":5,"soft_fail":false,"target_va":5416620794,"va":5377172929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39449119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e81ff25902","size":5,"soft_fail":false,"target_va":5416621408,"va":5377172284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t39449946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e85af55902","size":5,"soft_fail":false,"target_va":5416621600,"va":5377171649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=17
//     evidence={"assembly":"\tcallq\t39449332","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8f4f25902","size":5,"soft_fail":false,"target_va":5416621762,"va":5377172425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39449838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8eef45902","size":5,"soft_fail":false,"target_va":5416622240,"va":5377172397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39449681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e851f45902","size":5,"soft_fail":false,"target_va":5416622320,"va":5377172634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e848290000","size":5,"soft_fail":false,"target_va":5377176272,"va":5377165699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39447969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8a1ed5902","size":5,"soft_fail":false,"target_va":5416621376,"va":5377173402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::material_stats::MaterialStats::add_lintels_length
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2085138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e812d11f00","size":5,"soft_fail":false,"target_va":5379256560,"va":5377171417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::u_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4066981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8a50e3e00","size":5,"soft_fail":false,"target_va":5381234640,"va":5377167654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_path_segment::WallPathSegment::pt_count
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2048709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8c5421f00","size":5,"soft_fail":false,"target_va":5379218752,"va":5377170038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39405198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e88e465902","size":5,"soft_fail":false,"target_va":5416576832,"va":5377171629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t39404530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8f2435902","size":5,"soft_fail":false,"target_va":5416576144,"va":5377171609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6817790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8fe076800","size":5,"soft_fail":false,"target_va":5383984848,"va":5377167053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6813810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e872f86700","size":5,"soft_fail":false,"target_va":5383985360,"va":5377171545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6797753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8b9b96700","size":5,"soft_fail":false,"target_va":5383963792,"va":5377166034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6806260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8f4da6700","size":5,"soft_fail":false,"target_va":5383972224,"va":5377165959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6792908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8cca66700","size":5,"soft_fail":false,"target_va":5383964912,"va":5377171999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6797142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e856b76700","size":5,"soft_fail":false,"target_va":5383962816,"va":5377165669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39524709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e865195b02","size":5,"soft_fail":false,"target_va":5416697008,"va":5377172294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39447040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e800ea5902","size":5,"soft_fail":false,"target_va":5416619360,"va":5377172315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12547180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e86c74bf00","size":5,"soft_fail":false,"target_va":5389714944,"va":5377167759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopIdx::index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12530380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e8cc32bf00","size":5,"soft_fail":false,"target_va":5389701616,"va":5377171231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12530460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377165536,"raw_hex":"e81c33bf00","size":5,"soft_fail":false,"target_va":5389701744,"va":5377171279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5377165536,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5377166102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":20768,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11483,"source_line":3637,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":9100,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6382,"source_line":2922,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":7816,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":11836,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8414,"source_line":3650,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":13920,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":9476,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":18544,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":8840,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9357,"source_line":3923,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":4432,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":410,"record_offset":7844,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_reader::MessageReader::read
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_reader.rs
//     evidence={"module":410,"record_offset":2900,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9339,"source_line":27,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":410,"record_offset":19424,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":410,"record_offset":18936,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":410,"record_offset":9700,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":410,"record_offset":3668,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":410,"record_offset":3612,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":410,"record_offset":11556,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":410,"record_offset":2864,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":410,"record_offset":13124,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":410,"record_offset":6304,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":410,"record_offset":9616,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":410,"record_offset":5712,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26084,"source_line":3133,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":410,"record_offset":6388,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":410,"record_offset":8952,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":410,"record_offset":14060,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":410,"record_offset":2804,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":410,"record_offset":7384,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11663,"source_line":196,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":410,"record_offset":15708,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":410,"record_offset":15620,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":410,"record_offset":19480,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":410,"record_offset":19340,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":410,"record_offset":1996,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":410,"record_offset":588,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":410,"record_offset":984,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":410,"record_offset":6444,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":410,"record_offset":7900,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":410,"record_offset":19020,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_lintels::profile::wooden_lintel_profile
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/profile.rs
//     evidence={"module":410,"record_offset":9796,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26342,"source_line":12,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::place_pillars::PillarProposals::add_proposal
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/place_pillars.rs
//     evidence={"module":410,"record_offset":14172,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26104,"source_line":62,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/wall_path_segment.rs
//     evidence={"module":410,"record_offset":9016,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26093,"source_line":23,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::new_for_point_count
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":410,"record_offset":12252,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26183,"source_line":1305,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":410,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":280,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":410,"record_offset":10740,"parent_record_offset":10716,"ancestor_record_offset":280,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":410,"record_offset":11448,"parent_record_offset":11424,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":410,"record_offset":11472,"parent_record_offset":11448,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":410,"record_offset":10764,"parent_record_offset":10740,"ancestor_record_offset":280,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":410,"record_offset":12096,"parent_record_offset":12072,"ancestor_record_offset":280,"inlinee":6459,"source_line":172,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":410,"record_offset":11496,"parent_record_offset":11472,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":410,"record_offset":10692,"parent_record_offset":10548,"ancestor_record_offset":280,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":410,"record_offset":12024,"parent_record_offset":11984,"ancestor_record_offset":280,"inlinee":6458,"source_line":434,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":410,"record_offset":11424,"parent_record_offset":11336,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":410,"record_offset":5188,"parent_record_offset":5156,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":410,"record_offset":5096,"parent_record_offset":5072,"ancestor_record_offset":280,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":410,"record_offset":10488,"parent_record_offset":10460,"ancestor_record_offset":280,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":410,"record_offset":11908,"parent_record_offset":11872,"ancestor_record_offset":280,"inlinee":8417,"source_line":219,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":410,"record_offset":11364,"parent_record_offset":11336,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":410,"record_offset":11336,"parent_record_offset":11312,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":410,"record_offset":5236,"parent_record_offset":5212,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":410,"record_offset":5212,"parent_record_offset":5188,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":410,"record_offset":5120,"parent_record_offset":5096,"ancestor_record_offset":280,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":410,"record_offset":10548,"parent_record_offset":10516,"ancestor_record_offset":280,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":410,"record_offset":10516,"parent_record_offset":10488,"ancestor_record_offset":280,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":410,"record_offset":11944,"parent_record_offset":11908,"ancestor_record_offset":280,"inlinee":8418,"source_line":588,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":410,"record_offset":11312,"parent_record_offset":11288,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":410,"record_offset":10580,"parent_record_offset":10548,"ancestor_record_offset":280,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":5156,"parent_record_offset":4480,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":9136,"parent_record_offset":9100,"ancestor_record_offset":280,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":3240,"parent_record_offset":3216,"ancestor_record_offset":280,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":20800,"parent_record_offset":20768,"ancestor_record_offset":280,"inlinee":11072,"source_line":2588,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":280,"inlinee":10726,"source_line":3961,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":10848,"parent_record_offset":10824,"ancestor_record_offset":280,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":12752,"parent_record_offset":12352,"ancestor_record_offset":280,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":5672,"parent_record_offset":4480,"ancestor_record_offset":280,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":14248,"parent_record_offset":14172,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":14276,"parent_record_offset":14248,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":5072,"parent_record_offset":4480,"ancestor_record_offset":280,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":5276,"parent_record_offset":4480,"ancestor_record_offset":280,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":10432,"parent_record_offset":10396,"ancestor_record_offset":280,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":10460,"parent_record_offset":10432,"ancestor_record_offset":280,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":9888,"parent_record_offset":9796,"ancestor_record_offset":280,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":3692,"parent_record_offset":3668,"ancestor_record_offset":280,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":11580,"parent_record_offset":11556,"ancestor_record_offset":280,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":11124,"parent_record_offset":11096,"ancestor_record_offset":280,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$6::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":20932,"parent_record_offset":20800,"ancestor_record_offset":280,"inlinee":10784,"source_line":3638,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":410,"record_offset":3216,"parent_record_offset":2988,"ancestor_record_offset":280,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":410,"record_offset":11188,"parent_record_offset":11124,"ancestor_record_offset":280,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":410,"record_offset":4456,"parent_record_offset":4432,"ancestor_record_offset":280,"inlinee":10725,"source_line":17,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":410,"record_offset":10824,"parent_record_offset":10396,"ancestor_record_offset":280,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":410,"record_offset":12352,"parent_record_offset":12324,"ancestor_record_offset":280,"inlinee":6996,"source_line":13,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":410,"record_offset":11872,"parent_record_offset":11836,"ancestor_record_offset":280,"inlinee":9075,"source_line":49,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":410,"record_offset":10364,"parent_record_offset":10332,"ancestor_record_offset":280,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":410,"record_offset":10396,"parent_record_offset":10364,"ancestor_record_offset":280,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":410,"record_offset":7868,"parent_record_offset":7844,"ancestor_record_offset":280,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::MessageIteratorWithId::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":410,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":280,"inlinee":9344,"source_line":54,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":410,"record_offset":5328,"parent_record_offset":5304,"ancestor_record_offset":280,"inlinee":11533,"source_line":14,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$0::size_hint
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":410,"record_offset":4556,"parent_record_offset":4528,"ancestor_record_offset":280,"inlinee":26082,"source_line":18,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":410,"record_offset":5352,"parent_record_offset":5328,"ancestor_record_offset":280,"inlinee":11534,"source_line":86,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::iterators::impl$3::size_hint
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/iterators.rs
//     evidence={"module":410,"record_offset":4584,"parent_record_offset":4556,"ancestor_record_offset":280,"inlinee":26083,"source_line":103,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_cursor::MessageCursor::read_with_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_cursor.rs
//     evidence={"module":410,"record_offset":2964,"parent_record_offset":2924,"ancestor_record_offset":280,"inlinee":9342,"source_line":89,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/system_param.rs
//     evidence={"module":410,"record_offset":2924,"parent_record_offset":2900,"ancestor_record_offset":280,"inlinee":9614,"source_line":1064,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":410,"record_offset":10604,"parent_record_offset":10580,"ancestor_record_offset":280,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":410,"record_offset":2324,"parent_record_offset":2296,"ancestor_record_offset":280,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":410,"record_offset":2352,"parent_record_offset":2324,"ancestor_record_offset":280,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":410,"record_offset":2272,"parent_record_offset":2244,"ancestor_record_offset":280,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":410,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":280,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":410,"record_offset":2412,"parent_record_offset":2296,"ancestor_record_offset":280,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":410,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":280,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":410,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":280,"inlinee":6450,"source_line":1410,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":410,"record_offset":20956,"parent_record_offset":20932,"ancestor_record_offset":280,"inlinee":10786,"source_line":2197,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":410,"record_offset":14548,"parent_record_offset":14524,"ancestor_record_offset":280,"inlinee":17499,"source_line":1878,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":410,"record_offset":12844,"parent_record_offset":12816,"ancestor_record_offset":280,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":410,"record_offset":7144,"parent_record_offset":7120,"ancestor_record_offset":280,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":410,"record_offset":6712,"parent_record_offset":6688,"ancestor_record_offset":280,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":410,"record_offset":6796,"parent_record_offset":6772,"ancestor_record_offset":280,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":410,"record_offset":11060,"parent_record_offset":11020,"ancestor_record_offset":280,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":410,"record_offset":14652,"parent_record_offset":14172,"ancestor_record_offset":280,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":410,"record_offset":15852,"parent_record_offset":15828,"ancestor_record_offset":280,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":410,"record_offset":5788,"parent_record_offset":5748,"ancestor_record_offset":280,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":410,"record_offset":5404,"parent_record_offset":5380,"ancestor_record_offset":280,"inlinee":7044,"source_line":331,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":410,"record_offset":5380,"parent_record_offset":5352,"ancestor_record_offset":280,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":410,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":280,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":410,"record_offset":4612,"parent_record_offset":4584,"ancestor_record_offset":280,"inlinee":10731,"source_line":182,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":410,"record_offset":10200,"parent_record_offset":9796,"ancestor_record_offset":280,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":410,"record_offset":3636,"parent_record_offset":3612,"ancestor_record_offset":280,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":410,"record_offset":10896,"parent_record_offset":10872,"ancestor_record_offset":280,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":410,"record_offset":5304,"parent_record_offset":4480,"ancestor_record_offset":280,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":410,"record_offset":4528,"parent_record_offset":4480,"ancestor_record_offset":280,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":410,"record_offset":10992,"parent_record_offset":10920,"ancestor_record_offset":280,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":410,"record_offset":11692,"parent_record_offset":11556,"ancestor_record_offset":280,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":410,"record_offset":11716,"parent_record_offset":11692,"ancestor_record_offset":280,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":410,"record_offset":12816,"parent_record_offset":12788,"ancestor_record_offset":280,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":410,"record_offset":12788,"parent_record_offset":12752,"ancestor_record_offset":280,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":410,"record_offset":11800,"parent_record_offset":11776,"ancestor_record_offset":280,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":410,"record_offset":10300,"parent_record_offset":9796,"ancestor_record_offset":280,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":410,"record_offset":10872,"parent_record_offset":10848,"ancestor_record_offset":280,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":410,"record_offset":11096,"parent_record_offset":10992,"ancestor_record_offset":280,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":410,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":280,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":410,"record_offset":10628,"parent_record_offset":10604,"ancestor_record_offset":280,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":410,"record_offset":3172,"parent_record_offset":3144,"ancestor_record_offset":280,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":410,"record_offset":10652,"parent_record_offset":10628,"ancestor_record_offset":280,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":410,"record_offset":5000,"parent_record_offset":4612,"ancestor_record_offset":280,"inlinee":7142,"source_line":2300,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":410,"record_offset":3032,"parent_record_offset":2988,"ancestor_record_offset":280,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":410,"record_offset":11392,"parent_record_offset":11364,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":410,"record_offset":6944,"parent_record_offset":6916,"ancestor_record_offset":280,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":410,"record_offset":6916,"parent_record_offset":6888,"ancestor_record_offset":280,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":410,"record_offset":15892,"parent_record_offset":15732,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":410,"record_offset":15648,"parent_record_offset":15620,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":410,"record_offset":6064,"parent_record_offset":5940,"ancestor_record_offset":280,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":410,"record_offset":2144,"parent_record_offset":2104,"ancestor_record_offset":280,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":410,"record_offset":5636,"parent_record_offset":5612,"ancestor_record_offset":280,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":410,"record_offset":5428,"parent_record_offset":5404,"ancestor_record_offset":280,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":410,"record_offset":7648,"parent_record_offset":7384,"ancestor_record_offset":280,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":410,"record_offset":6112,"parent_record_offset":6088,"ancestor_record_offset":280,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":410,"record_offset":5612,"parent_record_offset":5304,"ancestor_record_offset":280,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":410,"record_offset":5460,"parent_record_offset":5380,"ancestor_record_offset":280,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":410,"record_offset":7460,"parent_record_offset":7384,"ancestor_record_offset":280,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":410,"record_offset":7680,"parent_record_offset":7384,"ancestor_record_offset":280,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":410,"record_offset":14492,"parent_record_offset":14172,"ancestor_record_offset":280,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":410,"record_offset":932,"parent_record_offset":904,"ancestor_record_offset":280,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":410,"record_offset":2760,"parent_record_offset":2104,"ancestor_record_offset":280,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":410,"record_offset":4736,"parent_record_offset":4712,"ancestor_record_offset":280,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":410,"record_offset":6028,"parent_record_offset":6004,"ancestor_record_offset":280,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":410,"record_offset":2476,"parent_record_offset":2244,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":410,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":410,"record_offset":4712,"parent_record_offset":4688,"ancestor_record_offset":280,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":410,"record_offset":7060,"parent_record_offset":7036,"ancestor_record_offset":280,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":410,"record_offset":5560,"parent_record_offset":5536,"ancestor_record_offset":280,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":410,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":280,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":410,"record_offset":4688,"parent_record_offset":4664,"ancestor_record_offset":280,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":410,"record_offset":1380,"parent_record_offset":1356,"ancestor_record_offset":280,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":410,"record_offset":1588,"parent_record_offset":1560,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":410,"record_offset":11156,"parent_record_offset":11124,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":410,"record_offset":1688,"parent_record_offset":1660,"ancestor_record_offset":280,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":410,"record_offset":1660,"parent_record_offset":1632,"ancestor_record_offset":280,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":410,"record_offset":3088,"parent_record_offset":2988,"ancestor_record_offset":280,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":410,"record_offset":3456,"parent_record_offset":2988,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":410,"record_offset":8864,"parent_record_offset":8840,"ancestor_record_offset":280,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":410,"record_offset":7620,"parent_record_offset":7384,"ancestor_record_offset":280,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":410,"record_offset":14584,"parent_record_offset":14172,"ancestor_record_offset":280,"inlinee":5770,"source_line":304,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":9576,"parent_record_offset":9540,"ancestor_record_offset":280,"inlinee":8788,"source_line":94,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":14020,"parent_record_offset":13984,"ancestor_record_offset":280,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":9916,"parent_record_offset":9888,"ancestor_record_offset":280,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":9508,"parent_record_offset":9476,"ancestor_record_offset":280,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":9944,"parent_record_offset":9916,"ancestor_record_offset":280,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":12212,"parent_record_offset":12184,"ancestor_record_offset":280,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":3144,"parent_record_offset":3116,"ancestor_record_offset":280,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":3116,"parent_record_offset":3088,"ancestor_record_offset":280,"inlinee":8593,"source_line":545,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":13984,"parent_record_offset":13952,"ancestor_record_offset":280,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":410,"record_offset":9540,"parent_record_offset":9508,"ancestor_record_offset":280,"inlinee":11933,"source_line":579,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":410,"record_offset":3480,"parent_record_offset":3456,"ancestor_record_offset":280,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":410,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":280,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":410,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":280,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":410,"record_offset":8888,"parent_record_offset":8864,"ancestor_record_offset":280,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":410,"record_offset":1716,"parent_record_offset":1688,"ancestor_record_offset":280,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":410,"record_offset":10920,"parent_record_offset":10896,"ancestor_record_offset":280,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":410,"record_offset":5536,"parent_record_offset":5484,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":410,"record_offset":4664,"parent_record_offset":4612,"ancestor_record_offset":280,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":410,"record_offset":1308,"parent_record_offset":1192,"ancestor_record_offset":280,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":410,"record_offset":9728,"parent_record_offset":9700,"ancestor_record_offset":280,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":410,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":280,"inlinee":4734,"source_line":302,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insert_tail
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":410,"record_offset":5940,"parent_record_offset":5844,"ancestor_record_offset":280,"inlinee":9485,"source_line":542,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":410,"record_offset":5844,"parent_record_offset":5748,"ancestor_record_offset":280,"inlinee":9500,"source_line":580,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::unstable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\unstable\mod.rs
//     evidence={"module":410,"record_offset":5748,"parent_record_offset":5712,"ancestor_record_offset":280,"inlinee":11895,"source_line":22,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":410,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":280,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":410,"record_offset":1020,"parent_record_offset":984,"ancestor_record_offset":280,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":410,"record_offset":1160,"parent_record_offset":1128,"ancestor_record_offset":280,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":410,"record_offset":1128,"parent_record_offset":1092,"ancestor_record_offset":280,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":410,"record_offset":1068,"parent_record_offset":1044,"ancestor_record_offset":280,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":410,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":280,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":410,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":280,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":410,"record_offset":1632,"parent_record_offset":1092,"ancestor_record_offset":280,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":410,"record_offset":696,"parent_record_offset":664,"ancestor_record_offset":280,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":410,"record_offset":664,"parent_record_offset":636,"ancestor_record_offset":280,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":410,"record_offset":824,"parent_record_offset":792,"ancestor_record_offset":280,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":410,"record_offset":880,"parent_record_offset":824,"ancestor_record_offset":280,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":410,"record_offset":792,"parent_record_offset":760,"ancestor_record_offset":280,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":410,"record_offset":636,"parent_record_offset":588,"ancestor_record_offset":280,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":410,"record_offset":904,"parent_record_offset":880,"ancestor_record_offset":280,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":410,"record_offset":1192,"parent_record_offset":1160,"ancestor_record_offset":280,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":410,"record_offset":1476,"parent_record_offset":1192,"ancestor_record_offset":280,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":410,"record_offset":1448,"parent_record_offset":1192,"ancestor_record_offset":280,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":410,"record_offset":1252,"parent_record_offset":1192,"ancestor_record_offset":280,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":19188,"parent_record_offset":19124,"ancestor_record_offset":280,"inlinee":4967,"source_line":3033,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":7532,"parent_record_offset":7496,"ancestor_record_offset":280,"inlinee":4960,"source_line":3097,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":19364,"parent_record_offset":19340,"ancestor_record_offset":280,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":19212,"parent_record_offset":19188,"ancestor_record_offset":280,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":7556,"parent_record_offset":7532,"ancestor_record_offset":280,"inlinee":4714,"source_line":4020,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":410,"record_offset":19388,"parent_record_offset":19364,"ancestor_record_offset":280,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":410,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":280,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":410,"record_offset":7712,"parent_record_offset":7680,"ancestor_record_offset":280,"inlinee":11665,"source_line":213,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":410,"record_offset":19100,"parent_record_offset":19076,"ancestor_record_offset":280,"inlinee":4972,"source_line":598,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":410,"record_offset":7168,"parent_record_offset":7144,"ancestor_record_offset":280,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":410,"record_offset":9064,"parent_record_offset":9016,"ancestor_record_offset":280,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":410,"record_offset":15796,"parent_record_offset":15764,"ancestor_record_offset":280,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":410,"record_offset":15764,"parent_record_offset":15732,"ancestor_record_offset":280,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":410,"record_offset":15732,"parent_record_offset":15708,"ancestor_record_offset":280,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":410,"record_offset":6856,"parent_record_offset":6828,"ancestor_record_offset":280,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":410,"record_offset":6888,"parent_record_offset":6856,"ancestor_record_offset":280,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":410,"record_offset":6828,"parent_record_offset":6576,"ancestor_record_offset":280,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":410,"record_offset":6688,"parent_record_offset":6576,"ancestor_record_offset":280,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":410,"record_offset":7240,"parent_record_offset":6576,"ancestor_record_offset":280,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":410,"record_offset":6772,"parent_record_offset":6576,"ancestor_record_offset":280,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":410,"record_offset":6632,"parent_record_offset":6576,"ancestor_record_offset":280,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":410,"record_offset":7120,"parent_record_offset":7096,"ancestor_record_offset":280,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":410,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":280,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":410,"record_offset":7924,"parent_record_offset":7900,"ancestor_record_offset":280,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":410,"record_offset":7096,"parent_record_offset":6988,"ancestor_record_offset":280,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":410,"record_offset":6500,"parent_record_offset":6468,"ancestor_record_offset":280,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":410,"record_offset":7036,"parent_record_offset":7012,"ancestor_record_offset":280,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":410,"record_offset":7332,"parent_record_offset":6576,"ancestor_record_offset":280,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":410,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":280,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":410,"record_offset":6552,"parent_record_offset":6528,"ancestor_record_offset":280,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":410,"record_offset":6528,"parent_record_offset":6468,"ancestor_record_offset":280,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":410,"record_offset":7984,"parent_record_offset":7924,"ancestor_record_offset":280,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":410,"record_offset":6576,"parent_record_offset":6552,"ancestor_record_offset":280,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":410,"record_offset":6660,"parent_record_offset":6576,"ancestor_record_offset":280,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":410,"record_offset":6988,"parent_record_offset":6576,"ancestor_record_offset":280,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":410,"record_offset":19528,"parent_record_offset":19504,"ancestor_record_offset":280,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":410,"record_offset":19504,"parent_record_offset":19480,"ancestor_record_offset":280,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":410,"record_offset":19552,"parent_record_offset":19528,"ancestor_record_offset":280,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":410,"record_offset":7496,"parent_record_offset":7384,"ancestor_record_offset":280,"inlinee":4959,"source_line":66,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_mutex.rs
//     evidence={"module":410,"record_offset":19124,"parent_record_offset":19100,"ancestor_record_offset":280,"inlinee":4965,"source_line":100,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":410,"record_offset":7588,"parent_record_offset":7496,"ancestor_record_offset":280,"inlinee":4962,"source_line":1112,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":410,"record_offset":19160,"parent_record_offset":19124,"ancestor_record_offset":280,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":410,"record_offset":2032,"parent_record_offset":1996,"ancestor_record_offset":280,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":410,"record_offset":19948,"parent_record_offset":19916,"ancestor_record_offset":280,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":410,"record_offset":20404,"parent_record_offset":20200,"ancestor_record_offset":280,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":410,"record_offset":2172,"parent_record_offset":2144,"ancestor_record_offset":280,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":410,"record_offset":2244,"parent_record_offset":2104,"ancestor_record_offset":280,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":410,"record_offset":2448,"parent_record_offset":2244,"ancestor_record_offset":280,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":410,"record_offset":15828,"parent_record_offset":15796,"ancestor_record_offset":280,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":410,"record_offset":2200,"parent_record_offset":2172,"ancestor_record_offset":280,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":410,"record_offset":2104,"parent_record_offset":2068,"ancestor_record_offset":280,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":410,"record_offset":2068,"parent_record_offset":2032,"ancestor_record_offset":280,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_lintels::profile::wooden_lintel_profile::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_lintels/profile.rs
//     evidence={"module":410,"record_offset":11020,"parent_record_offset":10992,"ancestor_record_offset":280,"inlinee":26344,"source_line":38,"archive":"_pdb/C/modules/00410.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::construct_lintels::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1367C
// 首条位置: {"module":1462,"inlinee":79484}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::construct_lintels::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135EC
// 首条位置: {"module":1455,"inlinee":79340}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1455,"record_offset":63312,"parent_record_offset":63280,"ancestor_record_offset":63280,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1455,"record_offset":63344,"parent_record_offset":63312,"ancestor_record_offset":63280,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1455,"record_offset":63376,"parent_record_offset":63344,"ancestor_record_offset":63280,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_lintels::construct_lintels::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x135ED
// 首条位置: {"module":1455,"inlinee":79341}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1455,"record_offset":63620,"parent_record_offset":63592,"ancestor_record_offset":63592,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1455,"record_offset":63676,"parent_record_offset":63592,"ancestor_record_offset":63592,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1455,"record_offset":63704,"parent_record_offset":63676,"ancestor_record_offset":63592,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01455.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1455,"record_offset":63644,"parent_record_offset":63620,"ancestor_record_offset":63592,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01455.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/three-version-source-provenance.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_lintels/mod.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_lintels/mod.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/construct_lintels/mod.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::construct_lintels
// 信息 ID: da1b645be08a2b108677d8cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e41be7cda312f714793a712c: void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>)> > @ 0x27fbcf -> 0x8108e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>),tuple$<>,void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::arches::LintelSegments>)> @ 0x2f5dd3 -> 0x8108e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::construct_lintels::closure$2
// 信息 ID: b66138ef3ee18504b1601f3d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_lintels::construct_lintels", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_lintels::construct_lintels::closure_env$2 <- system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_lintels::construct_lintels::closure_env$2 <- system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_lintels::construct_lintels::closure_env$2 <- system_wall_constructor::construct_lintels::place_support_beams::place_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_lintels::construct_lintels::closure_env$2 <- system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1462, inline_record=4472; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_lintels::place_support_beams::emit_support_beams::closure_env$0<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> >; module=1462, inline_record=6488; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::add_diagonal_support<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1511, inline_record=11232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::construct_lintels::closure$3
// 信息 ID: b91807dd729db890bdde3a5a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::construct_lintels::construct_lintels", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_lintels::construct_lintels::closure_env$3 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_lintels::construct_lintels::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_lintels::construct_lintels::closure_env$4> >; module=1455, inline_record=63280; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_lintels::construct_lintels::closure_env$4> >; module=1455, inline_record=64508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_lintels::construct_lintels::closure$4
// 信息 ID: 5bba1fd8f9305db36e225f40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::construct_lintels::construct_lintels", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_lintels::construct_lintels::closure_env$4 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_lintels::construct_lintels::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_lintels::construct_lintels::closure_env$4> >; module=1455, inline_record=63592; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<utils::loop_range::LoopRange,system_wall_constructor::construct_lintels::construct_lintels::closure_env$3>,core::slice::iter::Iter<f32> >,system_wall_constructor::construct_lintels::construct_lintels::closure_env$4> >; module=1455, inline_record=64816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
