// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/decorator_visual/cottage_wall_window.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 692e1e711e9247fa0cc7b07c9bc9648db30e534ed22150010f8efab6f8c037d6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: cottage_wall_window.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 442, 1503, 1510, 1546, 1552, 3054
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
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows
// 记录形式: compiled_procedure; instances=1
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":442,"record_offset":2224,"rva":8306640,"va":5377015760}
// 编译实例 RVAs: 0x7ebfd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7754683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e845ac89ff","size":5,"soft_fail":false,"target_va":5369267664,"va":5377022342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2340960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8a047dcff","size":5,"soft_fail":false,"target_va":5374676336,"va":5377017291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2340333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8134adcff","size":5,"soft_fail":false,"target_va":5374676688,"va":5377017016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1943321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8e758e2ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377024804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1940529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8cf63e2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377022028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=3
//     target_aliases=14 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t289736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8c86b0400","size":5,"soft_fail":false,"target_va":5377307216,"va":5377017475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408393b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t311741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8bdc10400","size":5,"soft_fail":false,"target_va":5377332144,"va":5377020398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408bc520
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t851510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e836fe0c00","size":5,"soft_fail":false,"target_va":5377869088,"va":5377017573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1b100
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2289214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e83eee2200","size":5,"soft_fail":false,"target_va":5379305728,"va":5377016509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3380728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8f8953300","size":5,"soft_fail":false,"target_va":5380398224,"va":5377017491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b26a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3385291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8cba73300","size":5,"soft_fail":false,"target_va":5380401776,"va":5377016480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c620
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3592374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8b6d03600","size":5,"soft_fail":false,"target_va":5380621856,"va":5377029477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91eb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4870958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e82e534a00","size":5,"soft_fail":false,"target_va":5381889712,"va":5377018749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4868073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8e9474a00","size":5,"soft_fail":false,"target_va":5381889824,"va":5377021746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4915451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8fb004b00","size":5,"soft_fail":false,"target_va":5381934528,"va":5377019072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4915570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e872014b00","size":5,"soft_fail":false,"target_va":5381934560,"va":5377018985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f7dc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10532128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e820b5a000","size":5,"soft_fail":false,"target_va":5387550144,"va":5377018011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125cf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10946781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8dd08a700","size":5,"soft_fail":false,"target_va":5387964304,"va":5377017518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141260ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10958045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8dd34a700","size":5,"soft_fail":false,"target_va":5387979936,"va":5377021886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14128ca70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11134218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e80ae5a900","size":5,"soft_fail":false,"target_va":5388159600,"va":5377025377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14128cc30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11134216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e808e5a900","size":5,"soft_fail":false,"target_va":5388160048,"va":5377025827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34513315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8a3a10e02","size":5,"soft_fail":false,"target_va":5411540992,"va":5377027672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t255036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e83ce40300","size":5,"soft_fail":false,"target_va":5377274800,"va":5377019759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34514324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e894a50e02","size":5,"soft_fail":false,"target_va":5411541584,"va":5377027255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34514889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8c9a70e02","size":5,"soft_fail":false,"target_va":5411541808,"va":5377026914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34514886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8c6a70e02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377026885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t34514857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8a9a70e02","size":5,"soft_fail":false,"target_va":5411542064,"va":5377027202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t34514872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8b8a70e02","size":5,"soft_fail":false,"target_va":5411541904,"va":5377027027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8fdf9ffff","size":5,"soft_fail":false,"target_va":5377014384,"va":5377015918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34527360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e880d80e02","size":5,"soft_fail":false,"target_va":5411543360,"va":5377015995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2286222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e88ee22200","size":5,"soft_fail":false,"target_va":5379303968,"va":5377017741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t285814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8765c0400","size":5,"soft_fail":false,"target_va":5377307232,"va":5377021413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::iter_modified
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2288776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e888ec2200","size":5,"soft_fail":false,"target_va":5379305584,"va":5377016803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3384308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8f4a33300","size":5,"soft_fail":false,"target_va":5380401520,"va":5377017207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::dormers::DormerHoleSsbo::cottage
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11111642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8da8ca900","size":5,"soft_fail":false,"target_va":5388134112,"va":5377022465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3398635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8ebdb3300","size":5,"soft_fail":false,"target_va":5380417344,"va":5377018704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4369089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8c1aa4200","size":5,"soft_fail":false,"target_va":5381385536,"va":5377016442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4374278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e806bf4200","size":5,"soft_fail":false,"target_va":5381390640,"va":5377016357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4359710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e81e864200","size":5,"soft_fail":false,"target_va":5381386480,"va":5377026765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4368811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8aba94200","size":5,"soft_fail":false,"target_va":5381384704,"va":5377015888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34617631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e81f391002","size":5,"soft_fail":false,"target_va":5411644800,"va":5377027164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34509856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e820940e02","size":5,"soft_fail":false,"target_va":5411537104,"va":5377027243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t230431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e81f840300","size":5,"soft_fail":false,"target_va":5377249968,"va":5377019532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t233889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8a1910300","size":5,"soft_fail":false,"target_va":5377251504,"va":5377017610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_wall_window::add_lintels
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e869210000","size":5,"soft_fail":false,"target_va":5377030544,"va":5377021986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6280","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e888180000","size":5,"soft_fail":false,"target_va":5377030064,"va":5377023779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle3d::extent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4896955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8bbb84a00","size":5,"soft_fail":false,"target_va":5381916256,"va":5377019296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4885464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8d88b4a00","size":5,"soft_fail":false,"target_va":5381907008,"va":5377021539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4871324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e89c544a00","size":5,"soft_fail":false,"target_va":5381890192,"va":5377018863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_projective_rotation_bad
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4867320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377015760,"raw_hex":"e8f8444a00","size":5,"soft_fail":false,"target_va":5381886416,"va":5377019091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":442,"record_offset":14424,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":442,"record_offset":17960,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":442,"record_offset":18016,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":442,"record_offset":9012,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":442,"record_offset":9068,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":442,"record_offset":21924,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":442,"record_offset":19636,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":442,"record_offset":5060,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":442,"record_offset":5032,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":442,"record_offset":15532,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":442,"record_offset":15476,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":442,"record_offset":15616,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":442,"record_offset":8848,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":25099,"source_line":291,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":442,"record_offset":9872,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":24795,"source_line":298,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":442,"record_offset":11256,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":24802,"source_line":309,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":442,"record_offset":4976,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":24514,"source_line":273,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":442,"record_offset":9096,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":442,"record_offset":15156,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":25181,"source_line":237,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] fastrand::Rng::usize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":442,"record_offset":18452,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":7382,"source_line":237,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":442,"record_offset":9128,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":442,"record_offset":12792,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":442,"record_offset":13016,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":442,"record_offset":13172,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":442,"record_offset":12960,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":442,"record_offset":16572,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$30::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":442,"record_offset":13228,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":13651,"source_line":1297,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":442,"record_offset":14152,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":442,"record_offset":9784,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":442,"record_offset":17460,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":442,"record_offset":3992,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":442,"record_offset":2568,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":442,"record_offset":3016,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":442,"record_offset":7772,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::add_cloth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":442,"record_offset":19132,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":25395,"source_line":250,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":442,"record_offset":14240,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":24805,"source_line":262,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_corner_window::has_cloth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_corner_window.rs
//     evidence={"module":442,"record_offset":19076,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":25180,"source_line":348,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::is_bay_window
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/assemble_dormer_roof_tiles.rs
//     evidence={"module":442,"record_offset":20224,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":25195,"source_line":381,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::subdivide_wall_decorator
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/mod.rs
//     evidence={"module":442,"record_offset":15328,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":25394,"source_line":12,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":442,"record_offset":14396,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":24808,"source_line":84,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_window_glass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":442,"record_offset":14676,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":25100,"source_line":170,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":442,"record_offset":14368,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":24807,"source_line":54,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":442,"record_offset":14932,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":442,"record_offset":12680,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::offset_x_clamped
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":442,"record_offset":12708,"parent_record_offset":2224,"ancestor_record_offset":2224,"inlinee":25386,"source_line":221,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":442,"record_offset":19248,"parent_record_offset":19224,"ancestor_record_offset":2224,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":442,"record_offset":19272,"parent_record_offset":19248,"ancestor_record_offset":2224,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":442,"record_offset":15800,"parent_record_offset":15776,"ancestor_record_offset":2224,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":442,"record_offset":19192,"parent_record_offset":19164,"ancestor_record_offset":2224,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":442,"record_offset":19224,"parent_record_offset":19192,"ancestor_record_offset":2224,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":442,"record_offset":15776,"parent_record_offset":15688,"ancestor_record_offset":2224,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":442,"record_offset":19164,"parent_record_offset":19132,"ancestor_record_offset":2224,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":442,"record_offset":19396,"parent_record_offset":19372,"ancestor_record_offset":2224,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":442,"record_offset":14500,"parent_record_offset":14468,"ancestor_record_offset":2224,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":442,"record_offset":15716,"parent_record_offset":15688,"ancestor_record_offset":2224,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":442,"record_offset":15688,"parent_record_offset":15664,"ancestor_record_offset":2224,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":442,"record_offset":14548,"parent_record_offset":14524,"ancestor_record_offset":2224,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":442,"record_offset":14524,"parent_record_offset":14500,"ancestor_record_offset":2224,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":442,"record_offset":15664,"parent_record_offset":15640,"ancestor_record_offset":2224,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":442,"record_offset":23784,"parent_record_offset":23748,"ancestor_record_offset":2224,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":442,"record_offset":11076,"parent_record_offset":9912,"ancestor_record_offset":2224,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":442,"record_offset":23596,"parent_record_offset":23572,"ancestor_record_offset":2224,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":442,"record_offset":14468,"parent_record_offset":14424,"ancestor_record_offset":2224,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":442,"record_offset":15556,"parent_record_offset":15532,"ancestor_record_offset":2224,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":442,"record_offset":18756,"parent_record_offset":18732,"ancestor_record_offset":2224,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":442,"record_offset":18804,"parent_record_offset":18780,"ancestor_record_offset":2224,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":442,"record_offset":9036,"parent_record_offset":9012,"ancestor_record_offset":2224,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":442,"record_offset":4328,"parent_record_offset":4300,"ancestor_record_offset":2224,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":442,"record_offset":4380,"parent_record_offset":4356,"ancestor_record_offset":2224,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":442,"record_offset":4356,"parent_record_offset":4328,"ancestor_record_offset":2224,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":442,"record_offset":4276,"parent_record_offset":4248,"ancestor_record_offset":2224,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":442,"record_offset":4300,"parent_record_offset":4276,"ancestor_record_offset":2224,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":442,"record_offset":4472,"parent_record_offset":4300,"ancestor_record_offset":2224,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":442,"record_offset":4584,"parent_record_offset":4560,"ancestor_record_offset":2224,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":442,"record_offset":7484,"parent_record_offset":7460,"ancestor_record_offset":2224,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":442,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":2224,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":442,"record_offset":5476,"parent_record_offset":5452,"ancestor_record_offset":2224,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":442,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":2224,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":442,"record_offset":5572,"parent_record_offset":5536,"ancestor_record_offset":2224,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":442,"record_offset":9572,"parent_record_offset":9152,"ancestor_record_offset":2224,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":442,"record_offset":9480,"parent_record_offset":9424,"ancestor_record_offset":2224,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":442,"record_offset":9248,"parent_record_offset":9224,"ancestor_record_offset":2224,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":442,"record_offset":9280,"parent_record_offset":9196,"ancestor_record_offset":2224,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":442,"record_offset":9344,"parent_record_offset":9152,"ancestor_record_offset":2224,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":442,"record_offset":14336,"parent_record_offset":14240,"ancestor_record_offset":2224,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":442,"record_offset":8968,"parent_record_offset":8944,"ancestor_record_offset":2224,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":442,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":2224,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":442,"record_offset":7552,"parent_record_offset":6856,"ancestor_record_offset":2224,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":442,"record_offset":15500,"parent_record_offset":15476,"ancestor_record_offset":2224,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":442,"record_offset":15580,"parent_record_offset":15556,"ancestor_record_offset":2224,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":442,"record_offset":17984,"parent_record_offset":17960,"ancestor_record_offset":2224,"inlinee":5075,"source_line":180,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":442,"record_offset":4404,"parent_record_offset":4380,"ancestor_record_offset":2224,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":442,"record_offset":15744,"parent_record_offset":15716,"ancestor_record_offset":2224,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":442,"record_offset":2716,"parent_record_offset":2684,"ancestor_record_offset":2224,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":442,"record_offset":19100,"parent_record_offset":19076,"ancestor_record_offset":2224,"inlinee":9282,"source_line":1980,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":442,"record_offset":15256,"parent_record_offset":15228,"ancestor_record_offset":2224,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":442,"record_offset":20100,"parent_record_offset":19964,"ancestor_record_offset":2224,"inlinee":6324,"source_line":2065,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":442,"record_offset":5380,"parent_record_offset":5352,"ancestor_record_offset":2224,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":442,"record_offset":5352,"parent_record_offset":5324,"ancestor_record_offset":2224,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":442,"record_offset":12924,"parent_record_offset":12900,"ancestor_record_offset":2224,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":442,"record_offset":12840,"parent_record_offset":12816,"ancestor_record_offset":2224,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":442,"record_offset":9812,"parent_record_offset":9784,"ancestor_record_offset":2224,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":442,"record_offset":12984,"parent_record_offset":12960,"ancestor_record_offset":2224,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":442,"record_offset":4144,"parent_record_offset":4100,"ancestor_record_offset":2224,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":442,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":2224,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":442,"record_offset":14276,"parent_record_offset":14240,"ancestor_record_offset":2224,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":442,"record_offset":14300,"parent_record_offset":14276,"ancestor_record_offset":2224,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":442,"record_offset":5764,"parent_record_offset":5060,"ancestor_record_offset":2224,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":442,"record_offset":7728,"parent_record_offset":6672,"ancestor_record_offset":2224,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":442,"record_offset":8912,"parent_record_offset":8884,"ancestor_record_offset":2224,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":442,"record_offset":2964,"parent_record_offset":2936,"ancestor_record_offset":2224,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":442,"record_offset":4932,"parent_record_offset":4100,"ancestor_record_offset":2224,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":442,"record_offset":4536,"parent_record_offset":4248,"ancestor_record_offset":2224,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":442,"record_offset":5632,"parent_record_offset":5608,"ancestor_record_offset":2224,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":442,"record_offset":23816,"parent_record_offset":23784,"ancestor_record_offset":2224,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":442,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":2224,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":442,"record_offset":18080,"parent_record_offset":18016,"ancestor_record_offset":2224,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":442,"record_offset":3404,"parent_record_offset":3380,"ancestor_record_offset":2224,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":442,"record_offset":3612,"parent_record_offset":3584,"ancestor_record_offset":2224,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":442,"record_offset":18104,"parent_record_offset":18080,"ancestor_record_offset":2224,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":442,"record_offset":19300,"parent_record_offset":19272,"ancestor_record_offset":2224,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":442,"record_offset":4428,"parent_record_offset":4404,"ancestor_record_offset":2224,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":442,"record_offset":3356,"parent_record_offset":3332,"ancestor_record_offset":2224,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":442,"record_offset":3380,"parent_record_offset":3356,"ancestor_record_offset":2224,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":442,"record_offset":3684,"parent_record_offset":3656,"ancestor_record_offset":2224,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":442,"record_offset":19664,"parent_record_offset":19636,"ancestor_record_offset":2224,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":442,"record_offset":3332,"parent_record_offset":3216,"ancestor_record_offset":2224,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":442,"record_offset":2684,"parent_record_offset":2656,"ancestor_record_offset":2224,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":442,"record_offset":3044,"parent_record_offset":3016,"ancestor_record_offset":2224,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":442,"record_offset":3184,"parent_record_offset":3152,"ancestor_record_offset":2224,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":442,"record_offset":3152,"parent_record_offset":3116,"ancestor_record_offset":2224,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":442,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":2224,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":442,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":2224,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":442,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":2224,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":442,"record_offset":3656,"parent_record_offset":3116,"ancestor_record_offset":2224,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":442,"record_offset":2656,"parent_record_offset":2624,"ancestor_record_offset":2224,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":442,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":2224,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":442,"record_offset":2816,"parent_record_offset":2784,"ancestor_record_offset":2224,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":442,"record_offset":2912,"parent_record_offset":2816,"ancestor_record_offset":2224,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":442,"record_offset":2784,"parent_record_offset":2760,"ancestor_record_offset":2224,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":442,"record_offset":2600,"parent_record_offset":2568,"ancestor_record_offset":2224,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":442,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":2224,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":442,"record_offset":3216,"parent_record_offset":3184,"ancestor_record_offset":2224,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":442,"record_offset":3500,"parent_record_offset":3216,"ancestor_record_offset":2224,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":442,"record_offset":3472,"parent_record_offset":3216,"ancestor_record_offset":2224,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":442,"record_offset":3276,"parent_record_offset":3216,"ancestor_record_offset":2224,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":442,"record_offset":11104,"parent_record_offset":11076,"ancestor_record_offset":2224,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":442,"record_offset":23632,"parent_record_offset":23596,"ancestor_record_offset":2224,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":442,"record_offset":11128,"parent_record_offset":11104,"ancestor_record_offset":2224,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":442,"record_offset":23656,"parent_record_offset":23632,"ancestor_record_offset":2224,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":442,"record_offset":23688,"parent_record_offset":23596,"ancestor_record_offset":2224,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":442,"record_offset":7508,"parent_record_offset":7484,"ancestor_record_offset":2224,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$1::iter_modified::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":442,"record_offset":6672,"parent_record_offset":6640,"ancestor_record_offset":2224,"inlinee":24794,"source_line":77,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":442,"record_offset":9912,"parent_record_offset":9872,"ancestor_record_offset":2224,"inlinee":24797,"source_line":249,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":442,"record_offset":8884,"parent_record_offset":8848,"ancestor_record_offset":2224,"inlinee":25059,"source_line":223,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":442,"record_offset":11300,"parent_record_offset":11256,"ancestor_record_offset":2224,"inlinee":24803,"source_line":258,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$22::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":442,"record_offset":10848,"parent_record_offset":10820,"ancestor_record_offset":2224,"inlinee":24799,"source_line":94,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":442,"record_offset":11188,"parent_record_offset":11164,"ancestor_record_offset":2224,"inlinee":24813,"source_line":252,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":442,"record_offset":8944,"parent_record_offset":8912,"ancestor_record_offset":2224,"inlinee":25042,"source_line":226,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":442,"record_offset":12612,"parent_record_offset":12588,"ancestor_record_offset":2224,"inlinee":24815,"source_line":261,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":442,"record_offset":8624,"parent_record_offset":8600,"ancestor_record_offset":2224,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":442,"record_offset":15180,"parent_record_offset":15156,"ancestor_record_offset":2224,"inlinee":25182,"source_line":143,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_mod_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":442,"record_offset":18476,"parent_record_offset":18452,"ancestor_record_offset":2224,"inlinee":7383,"source_line":161,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":442,"record_offset":15204,"parent_record_offset":15180,"ancestor_record_offset":2224,"inlinee":25183,"source_line":122,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::Rng::gen_u64
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":442,"record_offset":15228,"parent_record_offset":15204,"ancestor_record_offset":2224,"inlinee":7384,"source_line":128,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] fastrand::mul_high_u64
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fastrand-1.8.0/src/lib.rs
//     evidence={"module":442,"record_offset":18580,"parent_record_offset":18476,"ancestor_record_offset":2224,"inlinee":7386,"source_line":214,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":442,"record_offset":9152,"parent_record_offset":9128,"ancestor_record_offset":2224,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":442,"record_offset":9600,"parent_record_offset":9152,"ancestor_record_offset":2224,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":442,"record_offset":13356,"parent_record_offset":13332,"ancestor_record_offset":2224,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":442,"record_offset":13332,"parent_record_offset":13308,"ancestor_record_offset":2224,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":442,"record_offset":13096,"parent_record_offset":13064,"ancestor_record_offset":2224,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":442,"record_offset":13064,"parent_record_offset":13040,"ancestor_record_offset":2224,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":442,"record_offset":13040,"parent_record_offset":13016,"ancestor_record_offset":2224,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":442,"record_offset":12816,"parent_record_offset":12792,"ancestor_record_offset":2224,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":442,"record_offset":12900,"parent_record_offset":12792,"ancestor_record_offset":2224,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":442,"record_offset":9224,"parent_record_offset":9196,"ancestor_record_offset":2224,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":442,"record_offset":9196,"parent_record_offset":9152,"ancestor_record_offset":2224,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":442,"record_offset":5288,"parent_record_offset":5252,"ancestor_record_offset":2224,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":442,"record_offset":5324,"parent_record_offset":5288,"ancestor_record_offset":2224,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":442,"record_offset":5420,"parent_record_offset":5252,"ancestor_record_offset":2224,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":442,"record_offset":5252,"parent_record_offset":5200,"ancestor_record_offset":2224,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":442,"record_offset":6968,"parent_record_offset":6856,"ancestor_record_offset":2224,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":442,"record_offset":5452,"parent_record_offset":5200,"ancestor_record_offset":2224,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":442,"record_offset":7580,"parent_record_offset":6856,"ancestor_record_offset":2224,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":442,"record_offset":5536,"parent_record_offset":5508,"ancestor_record_offset":2224,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":442,"record_offset":5508,"parent_record_offset":5200,"ancestor_record_offset":2224,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":442,"record_offset":7052,"parent_record_offset":6856,"ancestor_record_offset":2224,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":442,"record_offset":6940,"parent_record_offset":6856,"ancestor_record_offset":2224,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":442,"record_offset":7460,"parent_record_offset":7436,"ancestor_record_offset":2224,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":442,"record_offset":6724,"parent_record_offset":6700,"ancestor_record_offset":2224,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":442,"record_offset":6748,"parent_record_offset":6724,"ancestor_record_offset":2224,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":442,"record_offset":7436,"parent_record_offset":7268,"ancestor_record_offset":2224,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":442,"record_offset":5124,"parent_record_offset":5092,"ancestor_record_offset":2224,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":442,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":2224,"inlinee":24792,"source_line":3317,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":442,"record_offset":6780,"parent_record_offset":6748,"ancestor_record_offset":2224,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":442,"record_offset":7376,"parent_record_offset":7292,"ancestor_record_offset":2224,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":442,"record_offset":7320,"parent_record_offset":7292,"ancestor_record_offset":2224,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":442,"record_offset":5608,"parent_record_offset":5200,"ancestor_record_offset":2224,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":442,"record_offset":7672,"parent_record_offset":6856,"ancestor_record_offset":2224,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":442,"record_offset":5200,"parent_record_offset":5160,"ancestor_record_offset":2224,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":442,"record_offset":7292,"parent_record_offset":7268,"ancestor_record_offset":2224,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":442,"record_offset":6832,"parent_record_offset":6808,"ancestor_record_offset":2224,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":442,"record_offset":6808,"parent_record_offset":6748,"ancestor_record_offset":2224,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":442,"record_offset":6856,"parent_record_offset":6832,"ancestor_record_offset":2224,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":442,"record_offset":6912,"parent_record_offset":6856,"ancestor_record_offset":2224,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":442,"record_offset":5160,"parent_record_offset":5124,"ancestor_record_offset":2224,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":442,"record_offset":7268,"parent_record_offset":6856,"ancestor_record_offset":2224,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":442,"record_offset":5908,"parent_record_offset":5884,"ancestor_record_offset":2224,"inlinee":24790,"source_line":1801,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":442,"record_offset":4028,"parent_record_offset":3992,"ancestor_record_offset":2224,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":442,"record_offset":20932,"parent_record_offset":20900,"ancestor_record_offset":2224,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":442,"record_offset":21448,"parent_record_offset":21188,"ancestor_record_offset":2224,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":442,"record_offset":4172,"parent_record_offset":4144,"ancestor_record_offset":2224,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":442,"record_offset":4248,"parent_record_offset":4100,"ancestor_record_offset":2224,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":442,"record_offset":4508,"parent_record_offset":4248,"ancestor_record_offset":2224,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":442,"record_offset":6700,"parent_record_offset":6672,"ancestor_record_offset":2224,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":442,"record_offset":5092,"parent_record_offset":5060,"ancestor_record_offset":2224,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":442,"record_offset":5884,"parent_record_offset":5852,"ancestor_record_offset":2224,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":442,"record_offset":13128,"parent_record_offset":13096,"ancestor_record_offset":2224,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":442,"record_offset":4200,"parent_record_offset":4172,"ancestor_record_offset":2224,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":442,"record_offset":4100,"parent_record_offset":4064,"ancestor_record_offset":2224,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":442,"record_offset":4064,"parent_record_offset":4028,"ancestor_record_offset":2224,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::with_offset_unbounded
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":442,"record_offset":15380,"parent_record_offset":15328,"ancestor_record_offset":2224,"inlinee":25129,"source_line":247,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1172E
// 首条位置: {"module":1510,"inlinee":71470}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 291
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x116FD
// 首条位置: {"module":1503,"inlinee":71421}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1503,"record_offset":31912,"parent_record_offset":31388,"ancestor_record_offset":31388,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1503,"record_offset":31708,"parent_record_offset":31388,"ancestor_record_offset":31388,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1503,"record_offset":31432,"parent_record_offset":31388,"ancestor_record_offset":31388,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1503,"record_offset":31616,"parent_record_offset":31388,"ancestor_record_offset":31388,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1503,"record_offset":31524,"parent_record_offset":31388,"ancestor_record_offset":31388,"inlinee":9342,"source_line":1105,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1503,"record_offset":31464,"parent_record_offset":31432,"ancestor_record_offset":31388,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1503,"record_offset":31876,"parent_record_offset":31736,"ancestor_record_offset":31388,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1503,"record_offset":31556,"parent_record_offset":31524,"ancestor_record_offset":31388,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1503,"record_offset":31648,"parent_record_offset":31616,"ancestor_record_offset":31388,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1503,"record_offset":31804,"parent_record_offset":31768,"ancestor_record_offset":31388,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1503,"record_offset":31768,"parent_record_offset":31736,"ancestor_record_offset":31388,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1503,"record_offset":31736,"parent_record_offset":31708,"ancestor_record_offset":31388,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1503,"record_offset":31840,"parent_record_offset":31804,"ancestor_record_offset":31388,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01503.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms
// 记录形式: compiled_procedure; instances=1
// 源码行: 411
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":442,"record_offset":23916,"rva":8320944,"va":5377030064}
// 编译实例 RVAs: 0x7ef7b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141260b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10949252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377030064,"raw_hex":"e88412a700","size":5,"soft_fail":false,"target_va":5387979584,"va":5377030327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34511417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377030064,"raw_hex":"e8399a0e02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377030354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34511669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377030064,"raw_hex":"e8359b0e02","size":5,"soft_fail":false,"target_va":5411542064,"va":5377030390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34511448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377030064,"raw_hex":"e8589a0e02","size":5,"soft_fail":false,"target_va":5411541904,"va":5377030451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1301548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377030064,"raw_hex":"e82cdc1300","size":5,"soft_fail":false,"target_va":5378331728,"va":5377030175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::normals_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2196922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377030064,"raw_hex":"e8ba852100","size":5,"soft_fail":false,"target_va":5379227136,"va":5377030209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions_without_indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2196584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377030064,"raw_hex":"e868842100","size":5,"soft_fail":false,"target_va":5379226784,"va":5377030195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":442,"record_offset":24784,"parent_record_offset":23916,"ancestor_record_offset":23916,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":442,"record_offset":24700,"parent_record_offset":23916,"ancestor_record_offset":23916,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":442,"record_offset":24160,"parent_record_offset":23916,"ancestor_record_offset":23916,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":442,"record_offset":24088,"parent_record_offset":23916,"ancestor_record_offset":23916,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":442,"record_offset":24724,"parent_record_offset":24700,"ancestor_record_offset":23916,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":442,"record_offset":24656,"parent_record_offset":24632,"ancestor_record_offset":23916,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":442,"record_offset":24632,"parent_record_offset":24208,"ancestor_record_offset":23916,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":442,"record_offset":24288,"parent_record_offset":24264,"ancestor_record_offset":23916,"inlinee":11428,"source_line":58,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":442,"record_offset":24264,"parent_record_offset":24240,"ancestor_record_offset":23916,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":442,"record_offset":24240,"parent_record_offset":24208,"ancestor_record_offset":23916,"inlinee":9852,"source_line":595,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":442,"record_offset":24184,"parent_record_offset":24160,"ancestor_record_offset":23916,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":442,"record_offset":24208,"parent_record_offset":24184,"ancestor_record_offset":23916,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":442,"record_offset":24748,"parent_record_offset":24724,"ancestor_record_offset":23916,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":442,"record_offset":24384,"parent_record_offset":24360,"ancestor_record_offset":23916,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":442,"record_offset":24360,"parent_record_offset":24336,"ancestor_record_offset":23916,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":442,"record_offset":24336,"parent_record_offset":24312,"ancestor_record_offset":23916,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":442,"record_offset":24312,"parent_record_offset":24288,"ancestor_record_offset":23916,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 442
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11885
// 首条位置: {"module":1546,"inlinee":71813}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1546,"record_offset":25972,"parent_record_offset":25932,"ancestor_record_offset":25932,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$8::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1546,"record_offset":26936,"parent_record_offset":25932,"ancestor_record_offset":25932,"inlinee":13681,"source_line":910,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1546,"record_offset":26848,"parent_record_offset":25932,"ancestor_record_offset":25932,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1546,"record_offset":26760,"parent_record_offset":25972,"ancestor_record_offset":25932,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1546,"record_offset":26700,"parent_record_offset":26000,"ancestor_record_offset":25932,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1546,"record_offset":26608,"parent_record_offset":26552,"ancestor_record_offset":25932,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1546,"record_offset":26048,"parent_record_offset":26000,"ancestor_record_offset":25932,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1546,"record_offset":26112,"parent_record_offset":26076,"ancestor_record_offset":25932,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1546,"record_offset":26232,"parent_record_offset":26076,"ancestor_record_offset":25932,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1546,"record_offset":26876,"parent_record_offset":26848,"ancestor_record_offset":25932,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1546,"record_offset":26960,"parent_record_offset":26936,"ancestor_record_offset":25932,"inlinee":13682,"source_line":727,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1546,"record_offset":26000,"parent_record_offset":25972,"ancestor_record_offset":25932,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1546,"record_offset":26076,"parent_record_offset":26000,"ancestor_record_offset":25932,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1546,"record_offset":26808,"parent_record_offset":26784,"ancestor_record_offset":25932,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1546,"record_offset":26784,"parent_record_offset":26760,"ancestor_record_offset":25932,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1546,"record_offset":26352,"parent_record_offset":26324,"ancestor_record_offset":25932,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1546,"record_offset":26324,"parent_record_offset":26000,"ancestor_record_offset":25932,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/01546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::cottage_wall_window::add_lintels
// 记录形式: compiled_procedure; instances=1
// 源码行: 451
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":442,"record_offset":24844,"rva":8321424,"va":5377030544}
// 编译实例 RVAs: 0x7ef990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140833ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t278852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377030544,"raw_hex":"e844410400","size":5,"soft_fail":false,"target_va":5377309600,"va":5377030743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3367463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377030544,"raw_hex":"e827623300","size":5,"soft_fail":false,"target_va":5380398224,"va":5377030756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125cf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: cottage_wall_window.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10933510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377030544,"raw_hex":"e806d5a600","size":5,"soft_fail":false,"target_va":5387964304,"va":5377030789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-2671824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377030544,"raw_hex":"e8303bd7ff","size":5,"soft_fail":false,"target_va":5374359040,"va":5377030859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34508724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377030544,"raw_hex":"e8b48f0e02","size":5,"soft_fail":false,"target_va":5411541904,"va":5377033175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t276538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377030544,"raw_hex":"e83a380400","size":5,"soft_fail":false,"target_va":5377307232,"va":5377030689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::with_y_offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4860809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377030544,"raw_hex":"e8892b4a00","size":5,"soft_fail":false,"target_va":5381891536,"va":5377030722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":442,"record_offset":25252,"parent_record_offset":24844,"ancestor_record_offset":24844,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":442,"record_offset":25444,"parent_record_offset":24844,"ancestor_record_offset":24844,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":442,"record_offset":25032,"parent_record_offset":24844,"ancestor_record_offset":24844,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":442,"record_offset":25580,"parent_record_offset":24844,"ancestor_record_offset":24844,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":442,"record_offset":27396,"parent_record_offset":24844,"ancestor_record_offset":24844,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":442,"record_offset":26204,"parent_record_offset":24844,"ancestor_record_offset":24844,"inlinee":25397,"source_line":1508,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::color_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":442,"record_offset":26376,"parent_record_offset":24844,"ancestor_record_offset":24844,"inlinee":25101,"source_line":99,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::custom_shader
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":442,"record_offset":26320,"parent_record_offset":24844,"ancestor_record_offset":24844,"inlinee":24810,"source_line":126,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":442,"record_offset":26348,"parent_record_offset":24844,"ancestor_record_offset":24844,"inlinee":24808,"source_line":84,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":442,"record_offset":26292,"parent_record_offset":24844,"ancestor_record_offset":24844,"inlinee":24807,"source_line":54,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":442,"record_offset":25324,"parent_record_offset":25292,"ancestor_record_offset":24844,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":442,"record_offset":25372,"parent_record_offset":25348,"ancestor_record_offset":24844,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":442,"record_offset":25348,"parent_record_offset":25324,"ancestor_record_offset":24844,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":442,"record_offset":25292,"parent_record_offset":25252,"ancestor_record_offset":24844,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":442,"record_offset":26024,"parent_record_offset":25604,"ancestor_record_offset":24844,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":442,"record_offset":25932,"parent_record_offset":25876,"ancestor_record_offset":24844,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":442,"record_offset":25700,"parent_record_offset":25676,"ancestor_record_offset":24844,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":442,"record_offset":25732,"parent_record_offset":25648,"ancestor_record_offset":24844,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":442,"record_offset":25796,"parent_record_offset":25604,"ancestor_record_offset":24844,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":442,"record_offset":25152,"parent_record_offset":25128,"ancestor_record_offset":24844,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":442,"record_offset":27424,"parent_record_offset":27396,"ancestor_record_offset":24844,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":442,"record_offset":26232,"parent_record_offset":26204,"ancestor_record_offset":24844,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":442,"record_offset":25096,"parent_record_offset":25068,"ancestor_record_offset":24844,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":442,"record_offset":25532,"parent_record_offset":25480,"ancestor_record_offset":24844,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":442,"record_offset":25412,"parent_record_offset":25252,"ancestor_record_offset":24844,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":442,"record_offset":25480,"parent_record_offset":25444,"ancestor_record_offset":24844,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":442,"record_offset":25188,"parent_record_offset":25068,"ancestor_record_offset":24844,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":442,"record_offset":25068,"parent_record_offset":25032,"ancestor_record_offset":24844,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":442,"record_offset":25128,"parent_record_offset":25096,"ancestor_record_offset":24844,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":442,"record_offset":25604,"parent_record_offset":25580,"ancestor_record_offset":24844,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":442,"record_offset":26052,"parent_record_offset":25604,"ancestor_record_offset":24844,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":442,"record_offset":25676,"parent_record_offset":25648,"ancestor_record_offset":24844,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":442,"record_offset":25648,"parent_record_offset":25604,"ancestor_record_offset":24844,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00442.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":442,"record_offset":25212,"parent_record_offset":25188,"ancestor_record_offset":24844,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00442.jsonl.gz"}
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
// 完整结构及原始类型记录: cottage_wall_window.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: cottage_wall_window.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: cottage_wall_window.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::add_lintels
// 信息 ID: 1025b2744e310175e2559f31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7ed822 -> 0x7ef990 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x7f7e34 -> 0x7ef990 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms
// 信息 ID: 03e2b7f4d8d9fecd5f9dfb3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7ee427 -> 0x7ef7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x7edf23 -> 0x7ef7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x7f8020 -> 0x7ef7b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure$0
// 信息 ID: 5184b0516328c6fb5982077d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::components::transform::Transform,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::components::transform::Transform,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::components::transform::Transform,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::components::transform::Transform,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::components::transform::Transform,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::components::transform::Transform,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<utils::components::transform::Transform,core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::components::transform::Transform,alloc::vec::impl$20::extend_trusted::closure_env$0<utils::components::transform::Transform,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::positions_without_indices::closure_env$0>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<f32,3> > >,country_core::render::mesh::impl$0::normals_without_indices::closure_env$0> >,system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms::closure_env$0> > > >; module=1546, inline_record=25932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows
// 信息 ID: b0c0a27a6a6552c30e8af199
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 121a48a843a3661860cc0c60: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > > @ 0x43c87f -> 0x7ebfd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > > @ 0x47fef8 -> 0x7ebfd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x5285db -> 0x7ebfd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$0
// 信息 ID: 1d1cc5afa72d031f5ce28572
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0 <- itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0> > > >; module=1510, inline_record=3344; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$31::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0> > > >; module=1510, inline_record=4052; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >; module=1552, inline_record=4648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure$1
// 信息 ID: a12d5d7e6403c9eb4e6b9eb2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> >; module=1503, inline_record=31388; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::components::transform::Transform,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_decorator::utils::subdivide_wall_decorator::closure_env$0>,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$0>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >,system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows::closure_env$1> >; module=1503, inline_record=32988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
