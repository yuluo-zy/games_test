// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/decorator_visual/flag.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 09e353aaade5769b9fa68f8e952ba9720a35420b5950f2cabe3dae1af50f5b33
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: flag.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 450
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
// 函数符号: system_decorator::decorator_visual::flag::generate_flags
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":450,"record_offset":2064,"rva":8386240,"va":5377095360}
// 编译实例 RVAs: 0x7ff6c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=1
//     target_aliases=532 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8145838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e852b483ff","size":5,"soft_fail":false,"target_va":5368953728,"va":5377099561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2420501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8eb10dbff","size":5,"soft_fail":false,"target_va":5374676688,"va":5377097184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140601be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2100388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e85cf3dfff","size":5,"soft_fail":false,"target_va":5375007712,"va":5377108095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2021605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e81b27e1ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377103088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=5
//     target_aliases=14 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t209529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e879320300","size":5,"soft_fail":false,"target_va":5377307216,"va":5377097682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t210120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8c8340300","size":5,"soft_fail":false,"target_va":5377309600,"va":5377099475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140839340
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t228789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8b57d0300","size":5,"soft_fail":false,"target_va":5377332032,"va":5377103238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1c2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2213588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8d4c62100","size":5,"soft_fail":false,"target_va":5379310240,"va":5377096647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3300521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8a95c3200","size":5,"soft_fail":false,"target_va":5380398224,"va":5377097698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b26a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3305494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e816703200","size":5,"soft_fail":false,"target_va":5380401776,"va":5377096277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c620
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3513509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8a59c3500","size":5,"soft_fail":false,"target_va":5380621856,"va":5377108342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4789840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e850164900","size":5,"soft_fail":false,"target_va":5381889520,"va":5377099675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f7dc0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10452402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8b27d9f00","size":5,"soft_fail":false,"target_va":5387550144,"va":5377097737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125cf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10864780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e88cc8a500","size":5,"soft_fail":false,"target_va":5387964304,"va":5377099519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14126a450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10920147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8d3a0a600","size":5,"soft_fail":false,"target_va":5388018768,"va":5377098616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flag.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34744025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8d9261202","size":5,"soft_fail":false,"target_va":5411847168,"va":5377103138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34433219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8c3680d02","size":5,"soft_fail":false,"target_va":5411540992,"va":5377107768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t173279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8dfa40200","size":5,"soft_fail":false,"target_va":5377274800,"va":5377101516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34434369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8416d0d02","size":5,"soft_fail":false,"target_va":5411541584,"va":5377107210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34435315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8f3700d02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377106456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34434409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8696d0d02","size":5,"soft_fail":false,"target_va":5411542064,"va":5377107650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34434848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8206f0d02","size":5,"soft_fail":false,"target_va":5411542296,"va":5377107443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t34435339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e80b710d02","size":5,"soft_fail":false,"target_va":5411541904,"va":5377106560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8e1faffff","size":5,"soft_fail":false,"target_va":5377094208,"va":5377095514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34447764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e894a10d02","size":5,"soft_fail":false,"target_va":5411543360,"va":5377095591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2206560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e860ab2100","size":5,"soft_fail":false,"target_va":5379303968,"va":5377097403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t208742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8662f0300","size":5,"soft_fail":false,"target_va":5377307232,"va":5377098485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2222905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e839eb2100","size":5,"soft_fail":false,"target_va":5379319216,"va":5377096306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2211611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e81bbf2100","size":5,"soft_fail":false,"target_va":5379310144,"va":5377098528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::iter_modified
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2208588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e84cb32100","size":5,"soft_fail":false,"target_va":5379305584,"va":5377096991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3304094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e89e6a3200","size":5,"soft_fail":false,"target_va":5380401520,"va":5377097421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3318764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8eca33200","size":5,"soft_fail":false,"target_va":5380417344,"va":5377098575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2694072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8b81b2900","size":5,"soft_fail":false,"target_va":5379792624,"va":5377098547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_cuboid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t849600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8c0f60c00","size":5,"soft_fail":false,"target_va":5377949680,"va":5377100075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_raycast_w_params
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t835367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e827bf0c00","size":5,"soft_fail":false,"target_va":5377935216,"va":5377099844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4289481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8c9734100","size":5,"soft_fail":false,"target_va":5381385536,"va":5377096050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4294670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e80e884100","size":5,"soft_fail":false,"target_va":5381390640,"va":5377095965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4290327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e817774100","size":5,"soft_fail":false,"target_va":5381386480,"va":5377096148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4289211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8bb724100","size":5,"soft_fail":false,"target_va":5381384704,"va":5377095488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34538214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8e6020f02","size":5,"soft_fail":false,"target_va":5411644800,"va":5377106581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34429901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e8cd5b0d02","size":5,"soft_fail":false,"target_va":5411537104,"va":5377107198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t148532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e834440200","size":5,"soft_fail":false,"target_va":5377249968,"va":5377101431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t153937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377095360,"raw_hex":"e851590200","size":5,"soft_fail":false,"target_va":5377251504,"va":5377097562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":450,"record_offset":22648,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":450,"record_offset":20796,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":450,"record_offset":9504,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":450,"record_offset":14068,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":450,"record_offset":15944,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":450,"record_offset":26096,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":450,"record_offset":6100,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":450,"record_offset":7564,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":450,"record_offset":6072,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":450,"record_offset":13128,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":450,"record_offset":4764,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":450,"record_offset":14768,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":450,"record_offset":9684,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":24795,"source_line":298,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":450,"record_offset":15508,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":450,"record_offset":17720,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":24802,"source_line":309,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":450,"record_offset":6016,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":24514,"source_line":273,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":450,"record_offset":14796,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":450,"record_offset":14824,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$8::mul_assign
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":450,"record_offset":11876,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":13681,"source_line":910,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":450,"record_offset":11104,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":25347,"source_line":923,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":450,"record_offset":15480,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":450,"record_offset":16056,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":450,"record_offset":11788,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":450,"record_offset":3780,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":450,"record_offset":2344,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":450,"record_offset":2804,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":450,"record_offset":13736,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4949,"source_line":535,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":450,"record_offset":14096,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":450,"record_offset":9560,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":24805,"source_line":262,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::albedo_map
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":450,"record_offset":22984,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":24809,"source_line":131,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_roof_cloth
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":450,"record_offset":21012,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":25352,"source_line":158,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::flag_wall_cloth
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":450,"record_offset":24548,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":25471,"source_line":164,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::mark_as_preview
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":450,"record_offset":20768,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":24808,"source_line":84,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":450,"record_offset":20740,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":24807,"source_line":54,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":450,"record_offset":13164,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":450,"record_offset":15756,"parent_record_offset":2064,"ancestor_record_offset":2064,"inlinee":25152,"source_line":70,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":450,"record_offset":22792,"parent_record_offset":22768,"ancestor_record_offset":2064,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":450,"record_offset":22816,"parent_record_offset":22792,"ancestor_record_offset":2064,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":450,"record_offset":22728,"parent_record_offset":22684,"ancestor_record_offset":2064,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":450,"record_offset":22768,"parent_record_offset":22728,"ancestor_record_offset":2064,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":450,"record_offset":22684,"parent_record_offset":22648,"ancestor_record_offset":2064,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":450,"record_offset":20864,"parent_record_offset":20832,"ancestor_record_offset":2064,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":450,"record_offset":20912,"parent_record_offset":20888,"ancestor_record_offset":2064,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":450,"record_offset":20888,"parent_record_offset":20864,"ancestor_record_offset":2064,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":450,"record_offset":10924,"parent_record_offset":9728,"ancestor_record_offset":2064,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":450,"record_offset":26148,"parent_record_offset":26124,"ancestor_record_offset":2064,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":450,"record_offset":20832,"parent_record_offset":20796,"ancestor_record_offset":2064,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":450,"record_offset":9528,"parent_record_offset":9504,"ancestor_record_offset":2064,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":450,"record_offset":4116,"parent_record_offset":4088,"ancestor_record_offset":2064,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":450,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":2064,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":450,"record_offset":4144,"parent_record_offset":4116,"ancestor_record_offset":2064,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":450,"record_offset":4064,"parent_record_offset":4036,"ancestor_record_offset":2064,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":450,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":2064,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":450,"record_offset":4260,"parent_record_offset":4088,"ancestor_record_offset":2064,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":450,"record_offset":4372,"parent_record_offset":4348,"ancestor_record_offset":2064,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":450,"record_offset":9216,"parent_record_offset":9192,"ancestor_record_offset":2064,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":450,"record_offset":6040,"parent_record_offset":6016,"ancestor_record_offset":2064,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":450,"record_offset":6740,"parent_record_offset":6716,"ancestor_record_offset":2064,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":450,"record_offset":8724,"parent_record_offset":8700,"ancestor_record_offset":2064,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":450,"record_offset":6836,"parent_record_offset":6800,"ancestor_record_offset":2064,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":450,"record_offset":11572,"parent_record_offset":11152,"ancestor_record_offset":2064,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":450,"record_offset":11480,"parent_record_offset":11424,"ancestor_record_offset":2064,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":450,"record_offset":11248,"parent_record_offset":11224,"ancestor_record_offset":2064,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":450,"record_offset":11280,"parent_record_offset":11196,"ancestor_record_offset":2064,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":450,"record_offset":11344,"parent_record_offset":11152,"ancestor_record_offset":2064,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":450,"record_offset":15844,"parent_record_offset":15820,"ancestor_record_offset":2064,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":450,"record_offset":9652,"parent_record_offset":9560,"ancestor_record_offset":2064,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":450,"record_offset":11060,"parent_record_offset":11036,"ancestor_record_offset":2064,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":450,"record_offset":8748,"parent_record_offset":8724,"ancestor_record_offset":2064,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":450,"record_offset":9284,"parent_record_offset":8588,"ancestor_record_offset":2064,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":450,"record_offset":6132,"parent_record_offset":6100,"ancestor_record_offset":2064,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":450,"record_offset":6220,"parent_record_offset":6196,"ancestor_record_offset":2064,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":450,"record_offset":7316,"parent_record_offset":7292,"ancestor_record_offset":2064,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":450,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":2064,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":450,"record_offset":4192,"parent_record_offset":4168,"ancestor_record_offset":2064,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":450,"record_offset":2504,"parent_record_offset":2472,"ancestor_record_offset":2064,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":450,"record_offset":6636,"parent_record_offset":6604,"ancestor_record_offset":2064,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":450,"record_offset":6604,"parent_record_offset":6572,"ancestor_record_offset":2064,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":450,"record_offset":13700,"parent_record_offset":13676,"ancestor_record_offset":2064,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":450,"record_offset":13644,"parent_record_offset":13620,"ancestor_record_offset":2064,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":450,"record_offset":11816,"parent_record_offset":11788,"ancestor_record_offset":2064,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":450,"record_offset":13588,"parent_record_offset":13564,"ancestor_record_offset":2064,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":450,"record_offset":3932,"parent_record_offset":3888,"ancestor_record_offset":2064,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":450,"record_offset":7136,"parent_record_offset":7112,"ancestor_record_offset":2064,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":450,"record_offset":6164,"parent_record_offset":6132,"ancestor_record_offset":2064,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":450,"record_offset":7232,"parent_record_offset":6244,"ancestor_record_offset":2064,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":450,"record_offset":7256,"parent_record_offset":7232,"ancestor_record_offset":2064,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":450,"record_offset":7112,"parent_record_offset":6324,"ancestor_record_offset":2064,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":450,"record_offset":7292,"parent_record_offset":6220,"ancestor_record_offset":2064,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":450,"record_offset":9460,"parent_record_offset":8392,"ancestor_record_offset":2064,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":450,"record_offset":11012,"parent_record_offset":9728,"ancestor_record_offset":2064,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":450,"record_offset":2752,"parent_record_offset":2724,"ancestor_record_offset":2064,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":450,"record_offset":4720,"parent_record_offset":3888,"ancestor_record_offset":2064,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":450,"record_offset":4324,"parent_record_offset":4036,"ancestor_record_offset":2064,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":450,"record_offset":6896,"parent_record_offset":6872,"ancestor_record_offset":2064,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":450,"record_offset":3216,"parent_record_offset":3192,"ancestor_record_offset":2064,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":450,"record_offset":3192,"parent_record_offset":3168,"ancestor_record_offset":2064,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":450,"record_offset":3400,"parent_record_offset":3372,"ancestor_record_offset":2064,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":450,"record_offset":22844,"parent_record_offset":22816,"ancestor_record_offset":2064,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":450,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":2064,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":450,"record_offset":13228,"parent_record_offset":13164,"ancestor_record_offset":2064,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":450,"record_offset":13256,"parent_record_offset":13228,"ancestor_record_offset":2064,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":450,"record_offset":3144,"parent_record_offset":3120,"ancestor_record_offset":2064,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":450,"record_offset":3168,"parent_record_offset":3144,"ancestor_record_offset":2064,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":450,"record_offset":3472,"parent_record_offset":3444,"ancestor_record_offset":2064,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":450,"record_offset":3120,"parent_record_offset":3004,"ancestor_record_offset":2064,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":450,"record_offset":2472,"parent_record_offset":2444,"ancestor_record_offset":2064,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":450,"record_offset":2832,"parent_record_offset":2804,"ancestor_record_offset":2064,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":450,"record_offset":2972,"parent_record_offset":2940,"ancestor_record_offset":2064,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":450,"record_offset":2940,"parent_record_offset":2904,"ancestor_record_offset":2064,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":450,"record_offset":2880,"parent_record_offset":2856,"ancestor_record_offset":2064,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":450,"record_offset":2904,"parent_record_offset":2880,"ancestor_record_offset":2064,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":450,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":2064,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":450,"record_offset":3444,"parent_record_offset":2904,"ancestor_record_offset":2064,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":450,"record_offset":2444,"parent_record_offset":2408,"ancestor_record_offset":2064,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":450,"record_offset":2408,"parent_record_offset":2380,"ancestor_record_offset":2064,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":450,"record_offset":2604,"parent_record_offset":2572,"ancestor_record_offset":2064,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":450,"record_offset":2700,"parent_record_offset":2604,"ancestor_record_offset":2064,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":450,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":2064,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":450,"record_offset":2380,"parent_record_offset":2344,"ancestor_record_offset":2064,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":450,"record_offset":2724,"parent_record_offset":2700,"ancestor_record_offset":2064,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":450,"record_offset":3004,"parent_record_offset":2972,"ancestor_record_offset":2064,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":450,"record_offset":3288,"parent_record_offset":3004,"ancestor_record_offset":2064,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":450,"record_offset":3260,"parent_record_offset":3004,"ancestor_record_offset":2064,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":450,"record_offset":3064,"parent_record_offset":3004,"ancestor_record_offset":2064,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":450,"record_offset":10952,"parent_record_offset":10924,"ancestor_record_offset":2064,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":450,"record_offset":26184,"parent_record_offset":26148,"ancestor_record_offset":2064,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":450,"record_offset":10976,"parent_record_offset":10952,"ancestor_record_offset":2064,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":450,"record_offset":26208,"parent_record_offset":26184,"ancestor_record_offset":2064,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":450,"record_offset":26240,"parent_record_offset":26148,"ancestor_record_offset":2064,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":450,"record_offset":9240,"parent_record_offset":9216,"ancestor_record_offset":2064,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$17::iter::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":450,"record_offset":7160,"parent_record_offset":7136,"ancestor_record_offset":2064,"inlinee":24786,"source_line":746,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$1::iter_modified::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":450,"record_offset":8392,"parent_record_offset":8356,"ancestor_record_offset":2064,"inlinee":24794,"source_line":77,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":450,"record_offset":15664,"parent_record_offset":15544,"ancestor_record_offset":2064,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":450,"record_offset":9728,"parent_record_offset":9684,"ancestor_record_offset":2064,"inlinee":24797,"source_line":249,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":450,"record_offset":15544,"parent_record_offset":15508,"ancestor_record_offset":2064,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":450,"record_offset":17764,"parent_record_offset":17720,"ancestor_record_offset":2064,"inlinee":24803,"source_line":258,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$22::eq
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":450,"record_offset":10696,"parent_record_offset":10668,"ancestor_record_offset":2064,"inlinee":24799,"source_line":94,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::collision_mesh::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":450,"record_offset":11036,"parent_record_offset":11012,"ancestor_record_offset":2064,"inlinee":24813,"source_line":252,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":450,"record_offset":15604,"parent_record_offset":15572,"ancestor_record_offset":2064,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::interaction_mesh::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":450,"record_offset":19072,"parent_record_offset":19048,"ancestor_record_offset":2064,"inlinee":24815,"source_line":261,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":450,"record_offset":11900,"parent_record_offset":11876,"ancestor_record_offset":2064,"inlinee":13682,"source_line":727,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":450,"record_offset":11128,"parent_record_offset":11104,"ancestor_record_offset":2064,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":450,"record_offset":11152,"parent_record_offset":11128,"ancestor_record_offset":2064,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":450,"record_offset":11600,"parent_record_offset":11152,"ancestor_record_offset":2064,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":450,"record_offset":13620,"parent_record_offset":13388,"ancestor_record_offset":2064,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":450,"record_offset":13676,"parent_record_offset":13388,"ancestor_record_offset":2064,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":450,"record_offset":13564,"parent_record_offset":13388,"ancestor_record_offset":2064,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":450,"record_offset":11224,"parent_record_offset":11196,"ancestor_record_offset":2064,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":450,"record_offset":11196,"parent_record_offset":11152,"ancestor_record_offset":2064,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":450,"record_offset":6532,"parent_record_offset":6492,"ancestor_record_offset":2064,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":450,"record_offset":6572,"parent_record_offset":6532,"ancestor_record_offset":2064,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":450,"record_offset":6680,"parent_record_offset":6492,"ancestor_record_offset":2064,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":450,"record_offset":6492,"parent_record_offset":6444,"ancestor_record_offset":2064,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":450,"record_offset":8700,"parent_record_offset":8588,"ancestor_record_offset":2064,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":450,"record_offset":6716,"parent_record_offset":6444,"ancestor_record_offset":2064,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":450,"record_offset":9312,"parent_record_offset":8588,"ancestor_record_offset":2064,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":450,"record_offset":6800,"parent_record_offset":6772,"ancestor_record_offset":2064,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":450,"record_offset":6772,"parent_record_offset":6444,"ancestor_record_offset":2064,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":450,"record_offset":8784,"parent_record_offset":8588,"ancestor_record_offset":2064,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":450,"record_offset":8672,"parent_record_offset":8588,"ancestor_record_offset":2064,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":450,"record_offset":9192,"parent_record_offset":9168,"ancestor_record_offset":2064,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":450,"record_offset":8452,"parent_record_offset":8424,"ancestor_record_offset":2064,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":450,"record_offset":8480,"parent_record_offset":8452,"ancestor_record_offset":2064,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":450,"record_offset":13760,"parent_record_offset":13736,"ancestor_record_offset":2064,"inlinee":4950,"source_line":754,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":450,"record_offset":9168,"parent_record_offset":9000,"ancestor_record_offset":2064,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":450,"record_offset":6376,"parent_record_offset":6348,"ancestor_record_offset":2064,"inlinee":10642,"source_line":3171,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":450,"record_offset":7644,"parent_record_offset":7620,"ancestor_record_offset":2064,"inlinee":24792,"source_line":3317,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":450,"record_offset":8512,"parent_record_offset":8480,"ancestor_record_offset":2064,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":7044,"parent_record_offset":7020,"ancestor_record_offset":2064,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":7020,"parent_record_offset":6376,"ancestor_record_offset":2064,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":9052,"parent_record_offset":9024,"ancestor_record_offset":2064,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":6872,"parent_record_offset":6444,"ancestor_record_offset":2064,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":9404,"parent_record_offset":8588,"ancestor_record_offset":2064,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":13868,"parent_record_offset":13808,"ancestor_record_offset":2064,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":6444,"parent_record_offset":6408,"ancestor_record_offset":2064,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":9024,"parent_record_offset":9000,"ancestor_record_offset":2064,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":8564,"parent_record_offset":8540,"ancestor_record_offset":2064,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":8540,"parent_record_offset":8480,"ancestor_record_offset":2064,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":13784,"parent_record_offset":13760,"ancestor_record_offset":2064,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":13840,"parent_record_offset":13808,"ancestor_record_offset":2064,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":8588,"parent_record_offset":8564,"ancestor_record_offset":2064,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":13808,"parent_record_offset":13784,"ancestor_record_offset":2064,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":8644,"parent_record_offset":8588,"ancestor_record_offset":2064,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":6408,"parent_record_offset":6376,"ancestor_record_offset":2064,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":450,"record_offset":9000,"parent_record_offset":8588,"ancestor_record_offset":2064,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":450,"record_offset":7620,"parent_record_offset":7596,"ancestor_record_offset":2064,"inlinee":24790,"source_line":1801,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":450,"record_offset":3816,"parent_record_offset":3780,"ancestor_record_offset":2064,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":450,"record_offset":4828,"parent_record_offset":4796,"ancestor_record_offset":2064,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":450,"record_offset":5344,"parent_record_offset":5084,"ancestor_record_offset":2064,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":450,"record_offset":3960,"parent_record_offset":3932,"ancestor_record_offset":2064,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":450,"record_offset":4036,"parent_record_offset":3888,"ancestor_record_offset":2064,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":450,"record_offset":4296,"parent_record_offset":4036,"ancestor_record_offset":2064,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":450,"record_offset":8424,"parent_record_offset":8392,"ancestor_record_offset":2064,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$47::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":450,"record_offset":6348,"parent_record_offset":6324,"ancestor_record_offset":2064,"inlinee":10640,"source_line":2392,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$21::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":450,"record_offset":7596,"parent_record_offset":7564,"ancestor_record_offset":2064,"inlinee":24788,"source_line":1562,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":450,"record_offset":3988,"parent_record_offset":3960,"ancestor_record_offset":2064,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":450,"record_offset":3888,"parent_record_offset":3852,"ancestor_record_offset":2064,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":450,"record_offset":3852,"parent_record_offset":3816,"ancestor_record_offset":2064,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":450,"record_offset":15796,"parent_record_offset":15756,"ancestor_record_offset":2064,"inlinee":25154,"source_line":39,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":450,"record_offset":15820,"parent_record_offset":15796,"ancestor_record_offset":2064,"inlinee":25156,"source_line":58,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":450,"record_offset":13200,"parent_record_offset":13164,"ancestor_record_offset":2064,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":450,"record_offset":13292,"parent_record_offset":13164,"ancestor_record_offset":2064,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":450,"record_offset":15716,"parent_record_offset":15664,"ancestor_record_offset":2064,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":450,"record_offset":15688,"parent_record_offset":15664,"ancestor_record_offset":2064,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/00450.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::flag::generate_flags
// 信息 ID: 6863d753c09377c2c13bbd02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 380a216a45240cabd4eb5314: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)> > @ 0x317c3a -> 0x7ff6c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)> > > @ 0x44d66a -> 0x7ff6c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
