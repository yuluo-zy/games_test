// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 31d237fbb16f8ba872ad63e9702d55cb878f29d94fcfd3f7c29029fcad7649b0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cottage_dormer.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 434, 435, 458, 1518
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
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
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
// 函数符号: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6256
// 首条位置: {"module":434,"inlinee":25174}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":434,"record_offset":2044,"rva":8157888,"va":5376867008}
// 编译实例 RVAs: 0x7c7ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7606379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e895ef8bff","size":5,"soft_fail":false,"target_va":5369267664,"va":5376874038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6810196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8ac1598ff","size":5,"soft_fail":false,"target_va":5370067488,"va":5376877679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2191897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8e78ddeff","size":5,"soft_fail":false,"target_va":5374676688,"va":5376868580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1788743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8b9b4e4ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5376870226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1789180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e804b3e4ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5376870679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408393b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t458814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e83e000700","size":5,"soft_fail":false,"target_va":5377332144,"va":5376873325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bc520
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1000215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e817430f00","size":5,"soft_fail":false,"target_va":5377869088,"va":5376868868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408e4750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1164331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e82bc41100","size":5,"soft_fail":false,"target_va":5378033488,"va":5376869152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1b100
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2437992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e868332500","size":5,"soft_fail":false,"target_va":5379305728,"va":5376867731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b26a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3534063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8efec3500","size":5,"soft_fail":false,"target_va":5380401776,"va":5376867708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5064657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8d1474d00","size":5,"soft_fail":false,"target_va":5381934528,"va":5376869866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141269710
//     owner=unresolved_namespace; count=9
//     target_aliases=6 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11145970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8f212aa00","size":5,"soft_fail":false,"target_va":5388015376,"va":5376869401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14128ca70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11285046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e83632ac00","size":5,"soft_fail":false,"target_va":5388159600,"va":5376874549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34978685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e87dbb1502","size":5,"soft_fail":false,"target_va":5411847952,"va":5376869262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34664683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8ebf01002","size":5,"soft_fail":false,"target_va":5411540992,"va":5376876304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t402105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8b9220600","size":5,"soft_fail":false,"target_va":5377274800,"va":5376872690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34666285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e82df71002","size":5,"soft_fail":false,"target_va":5411541584,"va":5376875294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34666556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e83cf81002","size":5,"soft_fail":false,"target_va":5411541776,"va":5376875215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t34666457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8d9f71002","size":5,"soft_fail":false,"target_va":5411542064,"va":5376875602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=14
//     evidence={"assembly":"\tcallq\t34666156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8acf61002","size":5,"soft_fail":false,"target_va":5411542296,"va":5376876135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t34666538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e82af81002","size":5,"soft_fail":false,"target_va":5411541904,"va":5376875361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e830fbffff","size":5,"soft_fail":false,"target_va":5376865936,"va":5376867163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34676115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8931d1102","size":5,"soft_fail":false,"target_va":5411543360,"va":5376867240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1160123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8bbb31100","size":5,"soft_fail":false,"target_va":5378028416,"va":5376868288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2435159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e857282500","size":5,"soft_fail":false,"target_va":5379303968,"va":5376868804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::iter_modified
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2437254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e886302500","size":5,"soft_fail":false,"target_va":5379305584,"va":5376868325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3532759","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8d7e73500","size":5,"soft_fail":false,"target_va":5380401520,"va":5376868756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::dormers::DormerHoleSsbo::cottage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11259713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e841cfab00","size":5,"soft_fail":false,"target_va":5388134112,"va":5376874394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2925549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8eda32c00","size":5,"soft_fail":false,"target_va":5379793584,"va":5376868030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_curve3
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10710827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e82b6fa300","size":5,"soft_fail":false,"target_va":5387580144,"va":5376869312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::get_closest_u_from_pos
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10711845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e82573a300","size":5,"soft_fail":false,"target_va":5387581024,"va":5376869174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10711756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8cc72a300","size":5,"soft_fail":false,"target_va":5387580992,"va":5376869231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4517861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8e5ef4400","size":5,"soft_fail":false,"target_va":5381385536,"va":5376867670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4523050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e82a044500","size":5,"soft_fail":false,"target_va":5381390640,"va":5376867585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4511380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e894d64400","size":5,"soft_fail":false,"target_va":5381386480,"va":5376875095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4517562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8baee4400","size":5,"soft_fail":false,"target_va":5381384704,"va":5376867137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34769563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e89b8a1202","size":5,"soft_fail":false,"target_va":5411644800,"va":5376875232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11064676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e864d5a800","size":5,"soft_fail":false,"target_va":5387942848,"va":5376878167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34661817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8b9e51002","size":5,"soft_fail":false,"target_va":5411537104,"va":5376875282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t380924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8fccf0500","size":5,"soft_fail":false,"target_va":5377249968,"va":5376869039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t382594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e882d60500","size":5,"soft_fail":false,"target_va":5377251504,"va":5376868905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_dormers::collision_geo::get_collision_mesh
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11279941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8451eac00","size":5,"soft_fail":false,"target_va":5388148912,"va":5376868966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5034804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e834d34c00","size":5,"soft_fail":false,"target_va":5381907008,"va":5376872199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation_bad
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5016150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376867008,"raw_hex":"e8568a4c00","size":5,"soft_fail":false,"target_va":5381886416,"va":5376870261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":434,"record_offset":9216,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":434,"record_offset":19620,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":434,"record_offset":9132,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":434,"record_offset":9188,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":434,"record_offset":19492,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":434,"record_offset":4824,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":434,"record_offset":4796,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":434,"record_offset":5552,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":434,"record_offset":22268,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":434,"record_offset":19408,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":434,"record_offset":4740,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":24514,"source_line":273,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::u8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":434,"record_offset":19264,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25181,"source_line":237,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":434,"record_offset":21792,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":7382,"source_line":237,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":434,"record_offset":20312,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":434,"record_offset":21736,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25184,"source_line":143,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":21560,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":434,"record_offset":3756,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":434,"record_offset":2324,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":434,"record_offset":2780,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":434,"record_offset":9304,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":24805,"source_line":262,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_corner_window::has_cloth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_corner_window.rs
//     evidence={"module":434,"record_offset":19208,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25180,"source_line":348,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":434,"record_offset":9096,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25174,"source_line":32,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::get_dormer_panel_xforms
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":434,"record_offset":9428,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25175,"source_line":172,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":434,"record_offset":19844,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":24808,"source_line":84,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":434,"record_offset":20092,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25100,"source_line":170,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":434,"record_offset":19816,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":24807,"source_line":54,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":434,"record_offset":13224,"parent_record_offset":13200,"ancestor_record_offset":2044,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":434,"record_offset":13248,"parent_record_offset":13224,"ancestor_record_offset":2044,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":434,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":2044,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":434,"record_offset":13168,"parent_record_offset":9428,"ancestor_record_offset":2044,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":434,"record_offset":13200,"parent_record_offset":13168,"ancestor_record_offset":2044,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":434,"record_offset":14448,"parent_record_offset":14360,"ancestor_record_offset":2044,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":434,"record_offset":19004,"parent_record_offset":18980,"ancestor_record_offset":2044,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":434,"record_offset":5852,"parent_record_offset":5828,"ancestor_record_offset":2044,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":434,"record_offset":14388,"parent_record_offset":14360,"ancestor_record_offset":2044,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":434,"record_offset":14360,"parent_record_offset":14336,"ancestor_record_offset":2044,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":434,"record_offset":5900,"parent_record_offset":5876,"ancestor_record_offset":2044,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":434,"record_offset":5876,"parent_record_offset":5852,"ancestor_record_offset":2044,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":434,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":2044,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":434,"record_offset":19664,"parent_record_offset":19620,"ancestor_record_offset":2044,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":434,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":2044,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":434,"record_offset":5804,"parent_record_offset":5704,"ancestor_record_offset":2044,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":434,"record_offset":10292,"parent_record_offset":10232,"ancestor_record_offset":2044,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":434,"record_offset":6240,"parent_record_offset":6216,"ancestor_record_offset":2044,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":434,"record_offset":6004,"parent_record_offset":5672,"ancestor_record_offset":2044,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":434,"record_offset":9156,"parent_record_offset":9132,"ancestor_record_offset":2044,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":434,"record_offset":5672,"parent_record_offset":5628,"ancestor_record_offset":2044,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":434,"record_offset":6996,"parent_record_offset":6972,"ancestor_record_offset":2044,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":434,"record_offset":6476,"parent_record_offset":5628,"ancestor_record_offset":2044,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":434,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":2044,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":434,"record_offset":6972,"parent_record_offset":5628,"ancestor_record_offset":2044,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":434,"record_offset":7020,"parent_record_offset":6996,"ancestor_record_offset":2044,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":434,"record_offset":6920,"parent_record_offset":6896,"ancestor_record_offset":2044,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":434,"record_offset":6600,"parent_record_offset":6576,"ancestor_record_offset":2044,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":434,"record_offset":6576,"parent_record_offset":6552,"ancestor_record_offset":2044,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":434,"record_offset":6780,"parent_record_offset":6756,"ancestor_record_offset":2044,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":434,"record_offset":6552,"parent_record_offset":6524,"ancestor_record_offset":2044,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":434,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":2044,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":434,"record_offset":6216,"parent_record_offset":5628,"ancestor_record_offset":2044,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":434,"record_offset":6896,"parent_record_offset":6872,"ancestor_record_offset":2044,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":434,"record_offset":6872,"parent_record_offset":6848,"ancestor_record_offset":2044,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":434,"record_offset":5604,"parent_record_offset":5580,"ancestor_record_offset":2044,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":434,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":2044,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":434,"record_offset":7044,"parent_record_offset":7020,"ancestor_record_offset":2044,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":434,"record_offset":6064,"parent_record_offset":6036,"ancestor_record_offset":2044,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":434,"record_offset":4092,"parent_record_offset":4064,"ancestor_record_offset":2044,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":434,"record_offset":4144,"parent_record_offset":4120,"ancestor_record_offset":2044,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":434,"record_offset":4120,"parent_record_offset":4092,"ancestor_record_offset":2044,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":434,"record_offset":4040,"parent_record_offset":4012,"ancestor_record_offset":2044,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":434,"record_offset":4064,"parent_record_offset":4040,"ancestor_record_offset":2044,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":434,"record_offset":4236,"parent_record_offset":4064,"ancestor_record_offset":2044,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":434,"record_offset":4348,"parent_record_offset":4324,"ancestor_record_offset":2044,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":434,"record_offset":10068,"parent_record_offset":10044,"ancestor_record_offset":2044,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":434,"record_offset":5972,"parent_record_offset":5948,"ancestor_record_offset":2044,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":434,"record_offset":10092,"parent_record_offset":10068,"ancestor_record_offset":2044,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":434,"record_offset":8808,"parent_record_offset":8784,"ancestor_record_offset":2044,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":434,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":2044,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":434,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":2044,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":434,"record_offset":8316,"parent_record_offset":8292,"ancestor_record_offset":2044,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":434,"record_offset":5300,"parent_record_offset":5264,"ancestor_record_offset":2044,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":434,"record_offset":20872,"parent_record_offset":20336,"ancestor_record_offset":2044,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":434,"record_offset":20780,"parent_record_offset":20724,"ancestor_record_offset":2044,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":434,"record_offset":20380,"parent_record_offset":20336,"ancestor_record_offset":2044,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":434,"record_offset":20432,"parent_record_offset":20408,"ancestor_record_offset":2044,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":434,"record_offset":20644,"parent_record_offset":20336,"ancestor_record_offset":2044,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":434,"record_offset":10132,"parent_record_offset":9976,"ancestor_record_offset":2044,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":434,"record_offset":9396,"parent_record_offset":9304,"ancestor_record_offset":2044,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":434,"record_offset":10160,"parent_record_offset":9976,"ancestor_record_offset":2044,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":434,"record_offset":8340,"parent_record_offset":8316,"ancestor_record_offset":2044,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":434,"record_offset":8876,"parent_record_offset":8180,"ancestor_record_offset":2044,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":434,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":2044,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":434,"record_offset":14416,"parent_record_offset":14388,"ancestor_record_offset":2044,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":434,"record_offset":2480,"parent_record_offset":2448,"ancestor_record_offset":2044,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":434,"record_offset":19232,"parent_record_offset":19208,"ancestor_record_offset":2044,"inlinee":9282,"source_line":1980,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":434,"record_offset":19364,"parent_record_offset":19336,"ancestor_record_offset":2044,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":434,"record_offset":21952,"parent_record_offset":21816,"ancestor_record_offset":2044,"inlinee":6324,"source_line":2065,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":434,"record_offset":5108,"parent_record_offset":5080,"ancestor_record_offset":2044,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":434,"record_offset":5080,"parent_record_offset":5052,"ancestor_record_offset":2044,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":434,"record_offset":5948,"parent_record_offset":5672,"ancestor_record_offset":2044,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":434,"record_offset":14188,"parent_record_offset":14156,"ancestor_record_offset":2044,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":434,"record_offset":10620,"parent_record_offset":10592,"ancestor_record_offset":2044,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":434,"record_offset":11792,"parent_record_offset":11764,"ancestor_record_offset":2044,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":434,"record_offset":3908,"parent_record_offset":3864,"ancestor_record_offset":2044,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":434,"record_offset":5516,"parent_record_offset":5492,"ancestor_record_offset":2044,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":434,"record_offset":10044,"parent_record_offset":10020,"ancestor_record_offset":2044,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":434,"record_offset":10020,"parent_record_offset":9976,"ancestor_record_offset":2044,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":434,"record_offset":6716,"parent_record_offset":6692,"ancestor_record_offset":2044,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":434,"record_offset":9336,"parent_record_offset":9304,"ancestor_record_offset":2044,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":434,"record_offset":9360,"parent_record_offset":9336,"ancestor_record_offset":2044,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":434,"record_offset":5492,"parent_record_offset":4824,"ancestor_record_offset":2044,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":434,"record_offset":9052,"parent_record_offset":7996,"ancestor_record_offset":2044,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":434,"record_offset":2728,"parent_record_offset":2700,"ancestor_record_offset":2044,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":434,"record_offset":4696,"parent_record_offset":3864,"ancestor_record_offset":2044,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":434,"record_offset":4300,"parent_record_offset":4012,"ancestor_record_offset":2044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":434,"record_offset":6168,"parent_record_offset":6136,"ancestor_record_offset":2044,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":434,"record_offset":5360,"parent_record_offset":5336,"ancestor_record_offset":2044,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":434,"record_offset":3192,"parent_record_offset":3168,"ancestor_record_offset":2044,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":434,"record_offset":3168,"parent_record_offset":3144,"ancestor_record_offset":2044,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":434,"record_offset":3376,"parent_record_offset":3348,"ancestor_record_offset":2044,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":434,"record_offset":13276,"parent_record_offset":13248,"ancestor_record_offset":2044,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":434,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":2044,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":434,"record_offset":5704,"parent_record_offset":5672,"ancestor_record_offset":2044,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":434,"record_offset":5772,"parent_record_offset":5744,"ancestor_record_offset":2044,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":434,"record_offset":10324,"parent_record_offset":10292,"ancestor_record_offset":2044,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":434,"record_offset":5744,"parent_record_offset":5704,"ancestor_record_offset":2044,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":434,"record_offset":10356,"parent_record_offset":10324,"ancestor_record_offset":2044,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":434,"record_offset":19432,"parent_record_offset":19408,"ancestor_record_offset":2044,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":434,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":2044,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":434,"record_offset":3144,"parent_record_offset":3120,"ancestor_record_offset":2044,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":434,"record_offset":3448,"parent_record_offset":3420,"ancestor_record_offset":2044,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":434,"record_offset":19524,"parent_record_offset":19492,"ancestor_record_offset":2044,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":434,"record_offset":3096,"parent_record_offset":2980,"ancestor_record_offset":2044,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":434,"record_offset":2448,"parent_record_offset":2420,"ancestor_record_offset":2044,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":434,"record_offset":2808,"parent_record_offset":2780,"ancestor_record_offset":2044,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":434,"record_offset":2948,"parent_record_offset":2916,"ancestor_record_offset":2044,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":434,"record_offset":2916,"parent_record_offset":2880,"ancestor_record_offset":2044,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":434,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":2044,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":434,"record_offset":2880,"parent_record_offset":2856,"ancestor_record_offset":2044,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":434,"record_offset":2832,"parent_record_offset":2808,"ancestor_record_offset":2044,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":434,"record_offset":3420,"parent_record_offset":2880,"ancestor_record_offset":2044,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":434,"record_offset":2420,"parent_record_offset":2388,"ancestor_record_offset":2044,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":434,"record_offset":2388,"parent_record_offset":2360,"ancestor_record_offset":2044,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":434,"record_offset":2580,"parent_record_offset":2548,"ancestor_record_offset":2044,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":434,"record_offset":2676,"parent_record_offset":2580,"ancestor_record_offset":2044,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":434,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":2044,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":434,"record_offset":2360,"parent_record_offset":2324,"ancestor_record_offset":2044,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":434,"record_offset":2700,"parent_record_offset":2676,"ancestor_record_offset":2044,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":434,"record_offset":2980,"parent_record_offset":2948,"ancestor_record_offset":2044,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":434,"record_offset":3264,"parent_record_offset":2980,"ancestor_record_offset":2044,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":434,"record_offset":3236,"parent_record_offset":2980,"ancestor_record_offset":2044,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":434,"record_offset":3040,"parent_record_offset":2980,"ancestor_record_offset":2044,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":434,"record_offset":9244,"parent_record_offset":9216,"ancestor_record_offset":2044,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":434,"record_offset":9268,"parent_record_offset":9244,"ancestor_record_offset":2044,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":434,"record_offset":8832,"parent_record_offset":8808,"ancestor_record_offset":2044,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$1::iter_modified::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":434,"record_offset":7996,"parent_record_offset":7964,"ancestor_record_offset":2044,"inlinee":24794,"source_line":77,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":434,"record_offset":19288,"parent_record_offset":19264,"ancestor_record_offset":2044,"inlinee":25182,"source_line":143,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":434,"record_offset":21816,"parent_record_offset":21792,"ancestor_record_offset":2044,"inlinee":7383,"source_line":161,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":434,"record_offset":19312,"parent_record_offset":19288,"ancestor_record_offset":2044,"inlinee":25183,"source_line":122,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":434,"record_offset":19336,"parent_record_offset":19312,"ancestor_record_offset":2044,"inlinee":7384,"source_line":128,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::mul_high_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":434,"record_offset":21924,"parent_record_offset":21816,"ancestor_record_offset":2044,"inlinee":7386,"source_line":214,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":434,"record_offset":6036,"parent_record_offset":5628,"ancestor_record_offset":2044,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":434,"record_offset":6112,"parent_record_offset":6088,"ancestor_record_offset":2044,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":434,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":2044,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":434,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":2044,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":434,"record_offset":20336,"parent_record_offset":20312,"ancestor_record_offset":2044,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":434,"record_offset":20408,"parent_record_offset":20336,"ancestor_record_offset":2044,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":434,"record_offset":12060,"parent_record_offset":9428,"ancestor_record_offset":2044,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":11972,"parent_record_offset":11936,"ancestor_record_offset":2044,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":11936,"parent_record_offset":11908,"ancestor_record_offset":2044,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":11908,"parent_record_offset":11880,"ancestor_record_offset":2044,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":10564,"parent_record_offset":10536,"ancestor_record_offset":2044,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":21760,"parent_record_offset":21736,"ancestor_record_offset":2044,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":11880,"parent_record_offset":11852,"ancestor_record_offset":2044,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":10592,"parent_record_offset":10564,"ancestor_record_offset":2044,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":14156,"parent_record_offset":13980,"ancestor_record_offset":2044,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":11764,"parent_record_offset":11480,"ancestor_record_offset":2044,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":434,"record_offset":20524,"parent_record_offset":20496,"ancestor_record_offset":2044,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":434,"record_offset":20496,"parent_record_offset":20336,"ancestor_record_offset":2044,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":434,"record_offset":5016,"parent_record_offset":4980,"ancestor_record_offset":2044,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":434,"record_offset":5052,"parent_record_offset":5016,"ancestor_record_offset":2044,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":434,"record_offset":5148,"parent_record_offset":4980,"ancestor_record_offset":2044,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":434,"record_offset":4980,"parent_record_offset":4928,"ancestor_record_offset":2044,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":434,"record_offset":8292,"parent_record_offset":8180,"ancestor_record_offset":2044,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":434,"record_offset":5180,"parent_record_offset":4928,"ancestor_record_offset":2044,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":434,"record_offset":8904,"parent_record_offset":8180,"ancestor_record_offset":2044,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":434,"record_offset":5264,"parent_record_offset":5236,"ancestor_record_offset":2044,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":434,"record_offset":5236,"parent_record_offset":4928,"ancestor_record_offset":2044,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":434,"record_offset":8376,"parent_record_offset":8180,"ancestor_record_offset":2044,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":434,"record_offset":8264,"parent_record_offset":8180,"ancestor_record_offset":2044,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":434,"record_offset":8784,"parent_record_offset":8760,"ancestor_record_offset":2044,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":434,"record_offset":8048,"parent_record_offset":8024,"ancestor_record_offset":2044,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":434,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":2044,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":434,"record_offset":8760,"parent_record_offset":8592,"ancestor_record_offset":2044,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":434,"record_offset":4872,"parent_record_offset":4848,"ancestor_record_offset":2044,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":434,"record_offset":7256,"parent_record_offset":7224,"ancestor_record_offset":2044,"inlinee":24792,"source_line":3317,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":434,"record_offset":8104,"parent_record_offset":8072,"ancestor_record_offset":2044,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":434,"record_offset":8700,"parent_record_offset":8616,"ancestor_record_offset":2044,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":434,"record_offset":8644,"parent_record_offset":8616,"ancestor_record_offset":2044,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":434,"record_offset":5336,"parent_record_offset":4928,"ancestor_record_offset":2044,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":434,"record_offset":8996,"parent_record_offset":8180,"ancestor_record_offset":2044,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":434,"record_offset":4928,"parent_record_offset":4896,"ancestor_record_offset":2044,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":434,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":2044,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":434,"record_offset":8156,"parent_record_offset":8132,"ancestor_record_offset":2044,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":434,"record_offset":8132,"parent_record_offset":8072,"ancestor_record_offset":2044,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":434,"record_offset":8180,"parent_record_offset":8156,"ancestor_record_offset":2044,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":434,"record_offset":8236,"parent_record_offset":8180,"ancestor_record_offset":2044,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":434,"record_offset":4896,"parent_record_offset":4872,"ancestor_record_offset":2044,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":434,"record_offset":8592,"parent_record_offset":8180,"ancestor_record_offset":2044,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":434,"record_offset":7224,"parent_record_offset":7192,"ancestor_record_offset":2044,"inlinee":24790,"source_line":1801,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":434,"record_offset":6804,"parent_record_offset":6780,"ancestor_record_offset":2044,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":434,"record_offset":3792,"parent_record_offset":3756,"ancestor_record_offset":2044,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":434,"record_offset":22560,"parent_record_offset":22528,"ancestor_record_offset":2044,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":434,"record_offset":23076,"parent_record_offset":22816,"ancestor_record_offset":2044,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":434,"record_offset":3936,"parent_record_offset":3908,"ancestor_record_offset":2044,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":434,"record_offset":4012,"parent_record_offset":3864,"ancestor_record_offset":2044,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":434,"record_offset":4272,"parent_record_offset":4012,"ancestor_record_offset":2044,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":434,"record_offset":8024,"parent_record_offset":7996,"ancestor_record_offset":2044,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":434,"record_offset":4848,"parent_record_offset":4824,"ancestor_record_offset":2044,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":434,"record_offset":7192,"parent_record_offset":7156,"ancestor_record_offset":2044,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":434,"record_offset":12008,"parent_record_offset":11972,"ancestor_record_offset":2044,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":434,"record_offset":3964,"parent_record_offset":3936,"ancestor_record_offset":2044,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":434,"record_offset":3864,"parent_record_offset":3828,"ancestor_record_offset":2044,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":434,"record_offset":3828,"parent_record_offset":3792,"ancestor_record_offset":2044,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":434,"record_offset":5580,"parent_record_offset":5552,"ancestor_record_offset":2044,"inlinee":25172,"source_line":71,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":434,"record_offset":9780,"parent_record_offset":9756,"ancestor_record_offset":2044,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":434,"record_offset":11524,"parent_record_offset":11480,"ancestor_record_offset":2044,"inlinee":24704,"source_line":681,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":434,"record_offset":10232,"parent_record_offset":10200,"ancestor_record_offset":2044,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":434,"record_offset":10200,"parent_record_offset":9428,"ancestor_record_offset":2044,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":434,"record_offset":11480,"parent_record_offset":9428,"ancestor_record_offset":2044,"inlinee":24703,"source_line":792,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":434,"record_offset":10264,"parent_record_offset":10232,"ancestor_record_offset":2044,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":434,"record_offset":10400,"parent_record_offset":10232,"ancestor_record_offset":2044,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":434,"record_offset":9976,"parent_record_offset":9940,"ancestor_record_offset":2044,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":434,"record_offset":9728,"parent_record_offset":9428,"ancestor_record_offset":2044,"inlinee":25176,"source_line":41,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":434,"record_offset":9756,"parent_record_offset":9728,"ancestor_record_offset":2044,"inlinee":24686,"source_line":90,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":434,"record_offset":9940,"parent_record_offset":9904,"ancestor_record_offset":2044,"inlinee":25178,"source_line":115,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":434,"record_offset":10536,"parent_record_offset":10232,"ancestor_record_offset":2044,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":434,"record_offset":11852,"parent_record_offset":11480,"ancestor_record_offset":2044,"inlinee":24692,"source_line":99,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6254
// 首条位置: {"module":434,"inlinee":25172}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":434,"record_offset":5604,"parent_record_offset":5580,"ancestor_record_offset":5580,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":434,"record_offset":5852,"parent_record_offset":5828,"ancestor_record_offset":5580,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":434,"record_offset":5900,"parent_record_offset":5876,"ancestor_record_offset":5580,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":434,"record_offset":5876,"parent_record_offset":5852,"ancestor_record_offset":5580,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":434,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":5580,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":434,"record_offset":5804,"parent_record_offset":5704,"ancestor_record_offset":5580,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":434,"record_offset":6240,"parent_record_offset":6216,"ancestor_record_offset":5580,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":434,"record_offset":6004,"parent_record_offset":5672,"ancestor_record_offset":5580,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":434,"record_offset":5672,"parent_record_offset":5628,"ancestor_record_offset":5580,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":434,"record_offset":6996,"parent_record_offset":6972,"ancestor_record_offset":5580,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":434,"record_offset":6476,"parent_record_offset":5628,"ancestor_record_offset":5580,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":434,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":5580,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":434,"record_offset":6972,"parent_record_offset":5628,"ancestor_record_offset":5580,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":434,"record_offset":7020,"parent_record_offset":6996,"ancestor_record_offset":5580,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":434,"record_offset":6920,"parent_record_offset":6896,"ancestor_record_offset":5580,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":434,"record_offset":6600,"parent_record_offset":6576,"ancestor_record_offset":5580,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":434,"record_offset":6576,"parent_record_offset":6552,"ancestor_record_offset":5580,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":434,"record_offset":6780,"parent_record_offset":6756,"ancestor_record_offset":5580,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":434,"record_offset":6552,"parent_record_offset":6524,"ancestor_record_offset":5580,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":434,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":5580,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":434,"record_offset":6216,"parent_record_offset":5628,"ancestor_record_offset":5580,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":434,"record_offset":6896,"parent_record_offset":6872,"ancestor_record_offset":5580,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":434,"record_offset":6872,"parent_record_offset":6848,"ancestor_record_offset":5580,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":434,"record_offset":5628,"parent_record_offset":5604,"ancestor_record_offset":5580,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":434,"record_offset":7044,"parent_record_offset":7020,"ancestor_record_offset":5580,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":434,"record_offset":6064,"parent_record_offset":6036,"ancestor_record_offset":5580,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":434,"record_offset":5972,"parent_record_offset":5948,"ancestor_record_offset":5580,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":434,"record_offset":5948,"parent_record_offset":5672,"ancestor_record_offset":5580,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":434,"record_offset":6692,"parent_record_offset":6628,"ancestor_record_offset":5580,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":434,"record_offset":6716,"parent_record_offset":6692,"ancestor_record_offset":5580,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":434,"record_offset":6756,"parent_record_offset":6576,"ancestor_record_offset":5580,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":434,"record_offset":6168,"parent_record_offset":6136,"ancestor_record_offset":5580,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":434,"record_offset":5704,"parent_record_offset":5672,"ancestor_record_offset":5580,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":434,"record_offset":5772,"parent_record_offset":5744,"ancestor_record_offset":5580,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":434,"record_offset":5744,"parent_record_offset":5704,"ancestor_record_offset":5580,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":434,"record_offset":6036,"parent_record_offset":5628,"ancestor_record_offset":5580,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":434,"record_offset":6112,"parent_record_offset":6088,"ancestor_record_offset":5580,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":434,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":5580,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":434,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":5580,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":434,"record_offset":6804,"parent_record_offset":6780,"ancestor_record_offset":5580,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::get_dormer_panel_xforms
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 172
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6257
// 首条位置: {"module":434,"inlinee":25175}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":434,"record_offset":13168,"parent_record_offset":9428,"ancestor_record_offset":9428,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":434,"record_offset":9904,"parent_record_offset":9428,"ancestor_record_offset":9428,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":434,"record_offset":14264,"parent_record_offset":9428,"ancestor_record_offset":9428,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":434,"record_offset":12060,"parent_record_offset":9428,"ancestor_record_offset":9428,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":12092,"parent_record_offset":9428,"ancestor_record_offset":9428,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":434,"record_offset":10200,"parent_record_offset":9428,"ancestor_record_offset":9428,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":434,"record_offset":11480,"parent_record_offset":9428,"ancestor_record_offset":9428,"inlinee":24703,"source_line":792,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":434,"record_offset":9728,"parent_record_offset":9428,"ancestor_record_offset":9428,"inlinee":25176,"source_line":41,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":434,"record_offset":13224,"parent_record_offset":13200,"ancestor_record_offset":9428,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":434,"record_offset":13248,"parent_record_offset":13224,"ancestor_record_offset":9428,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":434,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":9428,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":434,"record_offset":13200,"parent_record_offset":13168,"ancestor_record_offset":9428,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":434,"record_offset":14448,"parent_record_offset":14360,"ancestor_record_offset":9428,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":434,"record_offset":19004,"parent_record_offset":18980,"ancestor_record_offset":9428,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":434,"record_offset":14388,"parent_record_offset":14360,"ancestor_record_offset":9428,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":434,"record_offset":14360,"parent_record_offset":14336,"ancestor_record_offset":9428,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":434,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":9428,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":434,"record_offset":10292,"parent_record_offset":10232,"ancestor_record_offset":9428,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":434,"record_offset":10068,"parent_record_offset":10044,"ancestor_record_offset":9428,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":434,"record_offset":10092,"parent_record_offset":10068,"ancestor_record_offset":9428,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":434,"record_offset":10132,"parent_record_offset":9976,"ancestor_record_offset":9428,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":434,"record_offset":10160,"parent_record_offset":9976,"ancestor_record_offset":9428,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":434,"record_offset":14416,"parent_record_offset":14388,"ancestor_record_offset":9428,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":434,"record_offset":14188,"parent_record_offset":14156,"ancestor_record_offset":9428,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":434,"record_offset":10620,"parent_record_offset":10592,"ancestor_record_offset":9428,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":434,"record_offset":11792,"parent_record_offset":11764,"ancestor_record_offset":9428,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":434,"record_offset":10044,"parent_record_offset":10020,"ancestor_record_offset":9428,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":434,"record_offset":10020,"parent_record_offset":9976,"ancestor_record_offset":9428,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":434,"record_offset":14288,"parent_record_offset":14264,"ancestor_record_offset":9428,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":434,"record_offset":13276,"parent_record_offset":13248,"ancestor_record_offset":9428,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":434,"record_offset":10324,"parent_record_offset":10292,"ancestor_record_offset":9428,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":434,"record_offset":10356,"parent_record_offset":10324,"ancestor_record_offset":9428,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":11972,"parent_record_offset":11936,"ancestor_record_offset":9428,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":11936,"parent_record_offset":11908,"ancestor_record_offset":9428,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":11908,"parent_record_offset":11880,"ancestor_record_offset":9428,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":10564,"parent_record_offset":10536,"ancestor_record_offset":9428,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":11880,"parent_record_offset":11852,"ancestor_record_offset":9428,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":10592,"parent_record_offset":10564,"ancestor_record_offset":9428,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":14156,"parent_record_offset":13980,"ancestor_record_offset":9428,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":434,"record_offset":11764,"parent_record_offset":11480,"ancestor_record_offset":9428,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":434,"record_offset":12008,"parent_record_offset":11972,"ancestor_record_offset":9428,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":434,"record_offset":9780,"parent_record_offset":9756,"ancestor_record_offset":9428,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":434,"record_offset":11524,"parent_record_offset":11480,"ancestor_record_offset":9428,"inlinee":24704,"source_line":681,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":434,"record_offset":10232,"parent_record_offset":10200,"ancestor_record_offset":9428,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":434,"record_offset":10264,"parent_record_offset":10232,"ancestor_record_offset":9428,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":434,"record_offset":10400,"parent_record_offset":10232,"ancestor_record_offset":9428,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":434,"record_offset":9976,"parent_record_offset":9940,"ancestor_record_offset":9428,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":434,"record_offset":9756,"parent_record_offset":9728,"ancestor_record_offset":9428,"inlinee":24686,"source_line":90,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":434,"record_offset":9940,"parent_record_offset":9904,"ancestor_record_offset":9428,"inlinee":25178,"source_line":115,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":434,"record_offset":10536,"parent_record_offset":10232,"ancestor_record_offset":9428,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":434,"record_offset":11852,"parent_record_offset":11480,"ancestor_record_offset":9428,"inlinee":24692,"source_line":99,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds
// 记录形式: compiled_procedure; instances=1
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":434,"record_offset":23896,"rva":8169152,"va":5376878272}
// 编译实例 RVAs: 0x7ca6c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1408e4750
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1154847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376878272,"raw_hex":"e81f9f1100","size":5,"soft_fail":false,"target_va":5378033488,"va":5376878636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5056023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376878272,"raw_hex":"e817264d00","size":5,"soft_fail":false,"target_va":5381934560,"va":5376878532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: cottage_dormer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34969216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376878272,"raw_hex":"e880961502","size":5,"soft_fail":false,"target_va":5411847952,"va":5376878731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34662232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376878272,"raw_hex":"e858e71002","size":5,"soft_fail":false,"target_va":5411542064,"va":5376879827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1149745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376878272,"raw_hex":"e8318b1100","size":5,"soft_fail":false,"target_va":5378028128,"va":5376878378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10702599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376878272,"raw_hex":"e8074fa300","size":5,"soft_fail":false,"target_va":5387581248,"va":5376878644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5048224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376878272,"raw_hex":"e8a0074d00","size":5,"soft_fail":false,"target_va":5381926912,"va":5376878683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5041636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376878272,"raw_hex":"e8e4ed4c00","size":5,"soft_fail":false,"target_va":5381920464,"va":5376878823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5041348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376878272,"raw_hex":"e8c4ec4c00","size":5,"soft_fail":false,"target_va":5381920048,"va":5376878695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":434,"record_offset":24244,"parent_record_offset":23896,"ancestor_record_offset":23896,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":434,"record_offset":24188,"parent_record_offset":23896,"ancestor_record_offset":23896,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":434,"record_offset":24136,"parent_record_offset":23896,"ancestor_record_offset":23896,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::dormer_roof_height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/assemble_dormer_roof_tiles.rs
//     evidence={"module":434,"record_offset":24096,"parent_record_offset":23896,"ancestor_record_offset":23896,"inlinee":25186,"source_line":389,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":434,"record_offset":24584,"parent_record_offset":23896,"ancestor_record_offset":23896,"inlinee":25176,"source_line":41,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::subtract
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":434,"record_offset":24436,"parent_record_offset":23896,"ancestor_record_offset":23896,"inlinee":25187,"source_line":48,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrapping_contains
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":434,"record_offset":24696,"parent_record_offset":23896,"ancestor_record_offset":23896,"inlinee":25188,"source_line":73,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":434,"record_offset":24300,"parent_record_offset":24272,"ancestor_record_offset":23896,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":434,"record_offset":25048,"parent_record_offset":24976,"ancestor_record_offset":23896,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":434,"record_offset":25092,"parent_record_offset":25048,"ancestor_record_offset":23896,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":434,"record_offset":24328,"parent_record_offset":24300,"ancestor_record_offset":23896,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":434,"record_offset":24212,"parent_record_offset":24188,"ancestor_record_offset":23896,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":434,"record_offset":24160,"parent_record_offset":24136,"ancestor_record_offset":23896,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":434,"record_offset":24868,"parent_record_offset":24816,"ancestor_record_offset":23896,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":434,"record_offset":24816,"parent_record_offset":24696,"ancestor_record_offset":23896,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":434,"record_offset":24272,"parent_record_offset":24244,"ancestor_record_offset":23896,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":434,"record_offset":24356,"parent_record_offset":24328,"ancestor_record_offset":23896,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":434,"record_offset":24384,"parent_record_offset":24356,"ancestor_record_offset":23896,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":434,"record_offset":24532,"parent_record_offset":24504,"ancestor_record_offset":23896,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":434,"record_offset":24464,"parent_record_offset":24436,"ancestor_record_offset":23896,"inlinee":25176,"source_line":41,"archive":"_pdb/B/modules/00434.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":434,"record_offset":24504,"parent_record_offset":24464,"ancestor_record_offset":23896,"inlinee":24686,"source_line":90,"archive":"_pdb/B/modules/00434.jsonl.gz"}
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
// 完整结构及原始类型记录: cottage_dormer.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cottage_dormer.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: cottage_dormer.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
// 信息 ID: ce2ffe6177a8c750e3cb7ee1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=434, inline_record=9096; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs; module=435, inline_record=25160; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs; module=435, inline_record=30796; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cache_decorator_transforms::cache_decorator_transforms; module=458, inline_record=9800; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id; module=1518, inline_record=6172; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part; module=1518, inline_record=13468; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part; module=1518, inline_record=14176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers
// 信息 ID: fc681e201b938d2332052e2d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4a9057a11b4413ac9febf1f6: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x14c647 -> 0x7c7ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::system::system_param::Local<system_decorator::decorator_visual::cottage_dormers::collision_geo::DormerCollisionGeoLib>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > > @ 0x43911f -> 0x7c7ac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers::closure$0
// 信息 ID: ebae04b22b1a4fcbb2f5b9e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=434, inline_record=5580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::get_dormer_panel_xforms
// 信息 ID: 3a301a52ce59b030c92e5501
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers; module=434, inline_record=9428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_dormers::cottage_dormer::is_out_of_roof_bounds
// 信息 ID: f983c758b70e18976f45963e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
