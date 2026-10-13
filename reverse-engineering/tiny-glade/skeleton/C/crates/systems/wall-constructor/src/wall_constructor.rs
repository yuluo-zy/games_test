// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/wall_constructor.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 68302097f0abe360f768eb7f43e5f721240d6437bd7c83f16472b68659bfcddc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_constructor.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 63, 81, 94, 228, 249, 403, 409, 417, 1455, 1473, 1478, 1482, 1487, 1493, 1500, 1504, 1509, 3159, 4370, 4371, 4375
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fastrand; // namespace candidate
//   package identities: [{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.8.0"},{"name":"fastrand","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.3.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1493,"record_offset":76504,"rva":21311648,"va":5390020768}
// 编译实例 RVAs: 0x14530a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-14343058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e86e2425ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5390021741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=22
//     evidence={"assembly":"\tcallq\t-14346860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8941525ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390025559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=6
//     target_aliases=1699 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21299539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8adfebafe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390021726,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ff020
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18172043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e875b7eafe","size":5,"soft_fail":false,"target_va":5371850784,"va":5390022822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14193220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8bc6d27ff","size":5,"soft_fail":false,"target_va":5375829056,"va":5390022271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11909197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8b3474aff","size":5,"soft_fail":false,"target_va":5378120480,"va":5390029672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140956fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11527205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8db1b50ff","size":5,"soft_fail":false,"target_va":5378502608,"va":5390029808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6750443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e815ff98ff","size":5,"soft_fail":false,"target_va":5383273968,"va":5390024406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6704117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e80bb499ff","size":5,"soft_fail":false,"target_va":5383325024,"va":5390029136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6704530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e86eb299ff","size":5,"soft_fail":false,"target_va":5383325056,"va":5390029581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141403be0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-333992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e858e7faff","size":5,"soft_fail":false,"target_va":5389695968,"va":5390029955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405de0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-324264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8580dfbff","size":5,"soft_fail":false,"target_va":5389704672,"va":5390028931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14142afd0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-169567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8a169fdff","size":5,"soft_fail":false,"target_va":5389856720,"va":5390026282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146cea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t95849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e869760100","size":5,"soft_fail":false,"target_va":5390126752,"va":5390030898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141478800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t151865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e839510200","size":5,"soft_fail":false,"target_va":5390174208,"va":5390022338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26327098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e83ab89101","size":5,"soft_fail":false,"target_va":5416350720,"va":5390023617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26328977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e891bf9101","size":5,"soft_fail":false,"target_va":5416353040,"va":5390024058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27033355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e80b7f9c01","size":5,"soft_fail":false,"target_va":5417057088,"va":5390023728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27035129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8f9859c01","size":5,"soft_fail":false,"target_va":5417057664,"va":5390022530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27032032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8e0799c01","size":5,"soft_fail":false,"target_va":5417058032,"va":5390025995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bf80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27032149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8557a9c01","size":5,"soft_fail":false,"target_va":5417058176,"va":5390026022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26589817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e879ba9501","size":5,"soft_fail":false,"target_va":5416620769,"va":5390030947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12654155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8b5e93eff","size":5,"soft_fail":false,"target_va":5377374288,"va":5390028438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26341636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e804f19101","size":5,"soft_fail":false,"target_va":5416366080,"va":5390024439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26598143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8ffda9501","size":5,"soft_fail":false,"target_va":5416620794,"va":5390022646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26590554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e85abd9501","size":5,"soft_fail":false,"target_va":5416621408,"va":5390030849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed::<usize, usize>
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26590450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8f2bc9501","size":5,"soft_fail":false,"target_va":5416621188,"va":5390030733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26591519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e81fc19501","size":5,"soft_fail":false,"target_va":5416622208,"va":5390030684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26590677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8d5bd9501","size":5,"soft_fail":false,"target_va":5416621762,"va":5390031080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t115327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e87fc20100","size":5,"soft_fail":false,"target_va":5390146576,"va":5390031244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8c1350000","size":5,"soft_fail":false,"target_va":5390034704,"va":5390020938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26590224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e810bc9501","size":5,"soft_fail":false,"target_va":5416621376,"va":5390031147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::BrickTransform::encode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11548263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e899c94fff","size":5,"soft_fail":false,"target_va":5378481680,"va":5390029938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::FinalizedWallHoles::wall_length
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8410128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8f0ab7fff","size":5,"soft_fail":false,"target_va":5381612160,"va":5390022283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6643785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8b79f9aff","size":5,"soft_fail":false,"target_va":5383383152,"va":5390026932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6057505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8df91a3ff","size":5,"soft_fail":false,"target_va":5383963792,"va":5390021292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6048998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e81ab3a3ff","size":5,"soft_fail":false,"target_va":5383972224,"va":5390021217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6056743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8d994a3ff","size":5,"soft_fail":false,"target_va":5383964912,"va":5390021650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6058101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e88b8fa3ff","size":5,"soft_fail":false,"target_va":5383962816,"va":5390020912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-6621488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8d0f69aff","size":5,"soft_fail":false,"target_va":5383408256,"va":5390029739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26666144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8a0e49601","size":5,"soft_fail":false,"target_va":5416697008,"va":5390030859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-263460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8dcfafbff","size":5,"soft_fail":false,"target_va":5389771136,"va":5390034591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26588350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8beb49501","size":5,"soft_fail":false,"target_va":5416619360,"va":5390031005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::perturb_row_ends
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12663724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e854c43eff","size":5,"soft_fail":false,"target_va":5377364624,"va":5390028343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::split_horizontally
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12663831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8e9c33eff","size":5,"soft_fail":false,"target_va":5377365056,"va":5390028882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::split_into_random_rows::split_into_random_rows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12656321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e83fe13eff","size":5,"soft_fail":false,"target_va":5377365648,"va":5390021964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::trim_rows::trim_rows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12652055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8e9f13eff","size":5,"soft_fail":false,"target_va":5377370208,"va":5390022258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::wall_constructor::split_brick_by_holes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12863717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e81bb73bff","size":5,"soft_fail":false,"target_va":5377164912,"va":5390028624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6727223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8c95999ff","size":5,"soft_fail":false,"target_va":5383302240,"va":5390029458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::expanded
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6721202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e84e7199ff","size":5,"soft_fail":false,"target_va":5383302352,"va":5390023549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6727162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8065a99ff","size":5,"soft_fail":false,"target_va":5383302288,"va":5390029445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6721845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8cb6e99ff","size":5,"soft_fail":false,"target_va":5383301744,"va":5390023584,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6727213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8d35999ff","size":5,"soft_fail":false,"target_va":5383302224,"va":5390029432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6748677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390020768,"raw_hex":"e8fb0599ff","size":5,"soft_fail":false,"target_va":5383276416,"va":5390025088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1493,"record_offset":80324,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1493,"record_offset":80040,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":91292,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":13888,"source_line":1920,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":91352,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":5253,"source_line":2205,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":81788,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":10044,"source_line":798,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":93036,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":14132,"source_line":3663,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\iter.rs
//     evidence={"module":1493,"record_offset":93696,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":7433,"source_line":92,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":83024,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":81732,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":83052,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":83324,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":11070,"source_line":3465,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":82120,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":90564,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":86468,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":84872,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1493,"record_offset":92924,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":79260,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":84148,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1493,"record_offset":84476,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":1493,"record_offset":82800,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":83960,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1493,"record_offset":90480,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":80604,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":82660,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1493,"record_offset":84928,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1493,"record_offset":90660,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":90892,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":81440,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":90920,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1493,"record_offset":79300,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1493,"record_offset":87948,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":79148,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":88088,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1493,"record_offset":94092,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":93840,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":92708,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":93008,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":92652,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1493,"record_offset":85452,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1493,"record_offset":88188,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":78340,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":76940,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":77328,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":82064,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_gates::place_brick_steps::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_gates/mod.rs
//     evidence={"module":1493,"record_offset":94344,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":79758,"source_line":659,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::BrickRowSine::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":88356,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":26643,"source_line":12,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::sine_wave_deform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":91124,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":26645,"source_line":45,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::sine_wave_pack
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":91264,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":26644,"source_line":73,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::WallConstructor::normalize_holes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":81468,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":26334,"source_line":56,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$5
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":88996,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":17827,"source_line":379,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1493,"record_offset":92384,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":14739,"source_line":803,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1493,"record_offset":88968,"parent_record_offset":76504,"ancestor_record_offset":76504,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":80128,"parent_record_offset":80104,"ancestor_record_offset":76504,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":80152,"parent_record_offset":80128,"ancestor_record_offset":76504,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":86220,"parent_record_offset":86196,"ancestor_record_offset":76504,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":86244,"parent_record_offset":86220,"ancestor_record_offset":76504,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":80176,"parent_record_offset":80152,"ancestor_record_offset":76504,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":86268,"parent_record_offset":86244,"ancestor_record_offset":76504,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":80104,"parent_record_offset":80072,"ancestor_record_offset":76504,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":86196,"parent_record_offset":86080,"ancestor_record_offset":76504,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1493,"record_offset":80072,"parent_record_offset":80040,"ancestor_record_offset":76504,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_slices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":93088,"parent_record_offset":93060,"ancestor_record_offset":76504,"inlinee":11741,"source_line":1563,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_range
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":93312,"parent_record_offset":93088,"ancestor_record_offset":76504,"inlinee":4789,"source_line":228,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":91436,"parent_record_offset":91412,"ancestor_record_offset":76504,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":91412,"parent_record_offset":91376,"ancestor_record_offset":76504,"inlinee":5255,"source_line":236,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":93060,"parent_record_offset":93036,"ancestor_record_offset":76504,"inlinee":11740,"source_line":1499,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":91696,"parent_record_offset":91524,"ancestor_record_offset":76504,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::push_back_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":91376,"parent_record_offset":91352,"ancestor_record_offset":76504,"inlinee":5254,"source_line":2223,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::slice_ranges
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":93116,"parent_record_offset":93088,"ancestor_record_offset":76504,"inlinee":4783,"source_line":1666,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":91524,"parent_record_offset":91376,"ancestor_record_offset":76504,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::truncate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":91320,"parent_record_offset":91292,"ancestor_record_offset":76504,"inlinee":13889,"source_line":1364,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":81816,"parent_record_offset":81788,"ancestor_record_offset":76504,"inlinee":10045,"source_line":853,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":91560,"parent_record_offset":91524,"ancestor_record_offset":76504,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":91460,"parent_record_offset":91436,"ancestor_record_offset":76504,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":83144,"parent_record_offset":83120,"ancestor_record_offset":76504,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":83472,"parent_record_offset":83444,"ancestor_record_offset":76504,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":80792,"parent_record_offset":80764,"ancestor_record_offset":76504,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":83556,"parent_record_offset":83532,"ancestor_record_offset":76504,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":86120,"parent_record_offset":86080,"ancestor_record_offset":76504,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":86080,"parent_record_offset":86044,"ancestor_record_offset":76504,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":83532,"parent_record_offset":83500,"ancestor_record_offset":76504,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":83192,"parent_record_offset":83168,"ancestor_record_offset":76504,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":83168,"parent_record_offset":83144,"ancestor_record_offset":76504,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":83500,"parent_record_offset":83472,"ancestor_record_offset":76504,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":80852,"parent_record_offset":80820,"ancestor_record_offset":76504,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":80820,"parent_record_offset":80792,"ancestor_record_offset":76504,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":86044,"parent_record_offset":86012,"ancestor_record_offset":76504,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":82284,"parent_record_offset":82240,"ancestor_record_offset":76504,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1493,"record_offset":84416,"parent_record_offset":84392,"ancestor_record_offset":76504,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1493,"record_offset":81548,"parent_record_offset":81468,"ancestor_record_offset":76504,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1493,"record_offset":81584,"parent_record_offset":81548,"ancestor_record_offset":76504,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":83120,"parent_record_offset":83080,"ancestor_record_offset":76504,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":81520,"parent_record_offset":81468,"ancestor_record_offset":76504,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":81036,"parent_record_offset":81012,"ancestor_record_offset":76504,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":83384,"parent_record_offset":83324,"ancestor_record_offset":76504,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":84440,"parent_record_offset":84416,"ancestor_record_offset":76504,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":83356,"parent_record_offset":83324,"ancestor_record_offset":76504,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":83080,"parent_record_offset":83052,"ancestor_record_offset":76504,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":83444,"parent_record_offset":83384,"ancestor_record_offset":76504,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":80736,"parent_record_offset":80700,"ancestor_record_offset":76504,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":80764,"parent_record_offset":80736,"ancestor_record_offset":76504,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":80636,"parent_record_offset":80604,"ancestor_record_offset":76504,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":81300,"parent_record_offset":81272,"ancestor_record_offset":76504,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1493,"record_offset":81364,"parent_record_offset":81300,"ancestor_record_offset":76504,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1493,"record_offset":87616,"parent_record_offset":86524,"ancestor_record_offset":76504,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1493,"record_offset":87444,"parent_record_offset":87416,"ancestor_record_offset":76504,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1493,"record_offset":81012,"parent_record_offset":80700,"ancestor_record_offset":76504,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1493,"record_offset":80668,"parent_record_offset":80636,"ancestor_record_offset":76504,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1493,"record_offset":80700,"parent_record_offset":80668,"ancestor_record_offset":76504,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1493,"record_offset":82312,"parent_record_offset":82284,"ancestor_record_offset":76504,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":78668,"parent_record_offset":78640,"ancestor_record_offset":76504,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":78696,"parent_record_offset":78668,"ancestor_record_offset":76504,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":78616,"parent_record_offset":78588,"ancestor_record_offset":76504,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":78640,"parent_record_offset":78616,"ancestor_record_offset":76504,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":78756,"parent_record_offset":78640,"ancestor_record_offset":76504,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":78868,"parent_record_offset":78844,"ancestor_record_offset":76504,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":82756,"parent_record_offset":82732,"ancestor_record_offset":76504,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":84976,"parent_record_offset":84952,"ancestor_record_offset":76504,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":85000,"parent_record_offset":84976,"ancestor_record_offset":76504,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":83828,"parent_record_offset":83796,"ancestor_record_offset":76504,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":82732,"parent_record_offset":82708,"ancestor_record_offset":76504,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":1493,"record_offset":94140,"parent_record_offset":94116,"ancestor_record_offset":76504,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1493,"record_offset":94172,"parent_record_offset":94092,"ancestor_record_offset":76504,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1493,"record_offset":94116,"parent_record_offset":94092,"ancestor_record_offset":76504,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":82088,"parent_record_offset":82064,"ancestor_record_offset":76504,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::fract
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":88480,"parent_record_offset":88456,"ancestor_record_offset":76504,"inlinee":26641,"source_line":1873,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":92848,"parent_record_offset":92824,"ancestor_record_offset":76504,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::trunc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":88504,"parent_record_offset":88480,"ancestor_record_offset":76504,"inlinee":21635,"source_line":1844,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1493,"record_offset":81660,"parent_record_offset":81620,"ancestor_record_offset":76504,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1493,"record_offset":82404,"parent_record_offset":82340,"ancestor_record_offset":76504,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1493,"record_offset":85712,"parent_record_offset":85688,"ancestor_record_offset":76504,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1493,"record_offset":85760,"parent_record_offset":85736,"ancestor_record_offset":76504,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1493,"record_offset":85500,"parent_record_offset":85452,"ancestor_record_offset":76504,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1493,"record_offset":89344,"parent_record_offset":89304,"ancestor_record_offset":76504,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1493,"record_offset":82836,"parent_record_offset":82800,"ancestor_record_offset":76504,"inlinee":17819,"source_line":180,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1493,"record_offset":84548,"parent_record_offset":84504,"ancestor_record_offset":76504,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1493,"record_offset":84504,"parent_record_offset":84476,"ancestor_record_offset":76504,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1493,"record_offset":84648,"parent_record_offset":84620,"ancestor_record_offset":76504,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1493,"record_offset":84620,"parent_record_offset":84504,"ancestor_record_offset":76504,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1493,"record_offset":81084,"parent_record_offset":81060,"ancestor_record_offset":76504,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1493,"record_offset":84676,"parent_record_offset":84648,"ancestor_record_offset":76504,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1493,"record_offset":81148,"parent_record_offset":81108,"ancestor_record_offset":76504,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":1493,"record_offset":84024,"parent_record_offset":83992,"ancestor_record_offset":76504,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":82684,"parent_record_offset":82660,"ancestor_record_offset":76504,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":82872,"parent_record_offset":82836,"ancestor_record_offset":76504,"inlinee":17821,"source_line":124,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":83992,"parent_record_offset":83960,"ancestor_record_offset":76504,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":82708,"parent_record_offset":82684,"ancestor_record_offset":76504,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":82908,"parent_record_offset":82872,"ancestor_record_offset":76504,"inlinee":17822,"source_line":355,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1493,"record_offset":83796,"parent_record_offset":83764,"ancestor_record_offset":76504,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1493,"record_offset":83764,"parent_record_offset":83384,"ancestor_record_offset":76504,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1493,"record_offset":82624,"parent_record_offset":82600,"ancestor_record_offset":76504,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":81060,"parent_record_offset":81036,"ancestor_record_offset":76504,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":81272,"parent_record_offset":81148,"ancestor_record_offset":76504,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":85784,"parent_record_offset":85760,"ancestor_record_offset":76504,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1493,"record_offset":78720,"parent_record_offset":78696,"ancestor_record_offset":76504,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1493,"record_offset":82340,"parent_record_offset":82312,"ancestor_record_offset":76504,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1493,"record_offset":82372,"parent_record_offset":82340,"ancestor_record_offset":76504,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1493,"record_offset":86164,"parent_record_offset":86120,"ancestor_record_offset":76504,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1493,"record_offset":81232,"parent_record_offset":81204,"ancestor_record_offset":76504,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1493,"record_offset":87036,"parent_record_offset":86960,"ancestor_record_offset":76504,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1493,"record_offset":81204,"parent_record_offset":81176,"ancestor_record_offset":76504,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1493,"record_offset":86996,"parent_record_offset":86960,"ancestor_record_offset":76504,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1493,"record_offset":78488,"parent_record_offset":78448,"ancestor_record_offset":76504,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1493,"record_offset":84728,"parent_record_offset":84704,"ancestor_record_offset":76504,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1493,"record_offset":84952,"parent_record_offset":84928,"ancestor_record_offset":76504,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":84580,"parent_record_offset":84548,"ancestor_record_offset":76504,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":85652,"parent_record_offset":85500,"ancestor_record_offset":76504,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":84704,"parent_record_offset":84676,"ancestor_record_offset":76504,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":85736,"parent_record_offset":85712,"ancestor_record_offset":76504,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":89756,"parent_record_offset":89304,"ancestor_record_offset":76504,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1493,"record_offset":77276,"parent_record_offset":77248,"ancestor_record_offset":76504,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1493,"record_offset":79104,"parent_record_offset":78448,"ancestor_record_offset":76504,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1493,"record_offset":78820,"parent_record_offset":78588,"ancestor_record_offset":76504,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1493,"record_offset":82976,"parent_record_offset":82952,"ancestor_record_offset":76504,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":79196,"parent_record_offset":79148,"ancestor_record_offset":76504,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":77748,"parent_record_offset":77724,"ancestor_record_offset":76504,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":77724,"parent_record_offset":77700,"ancestor_record_offset":76504,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":77932,"parent_record_offset":77904,"ancestor_record_offset":76504,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1493,"record_offset":81332,"parent_record_offset":81300,"ancestor_record_offset":76504,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1493,"record_offset":78032,"parent_record_offset":78004,"ancestor_record_offset":76504,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1493,"record_offset":78004,"parent_record_offset":77976,"ancestor_record_offset":76504,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1493,"record_offset":93464,"parent_record_offset":93060,"ancestor_record_offset":76504,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1493,"record_offset":90592,"parent_record_offset":90564,"ancestor_record_offset":76504,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1493,"record_offset":90620,"parent_record_offset":90592,"ancestor_record_offset":76504,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1493,"record_offset":87972,"parent_record_offset":87948,"ancestor_record_offset":76504,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":77676,"parent_record_offset":77652,"ancestor_record_offset":76504,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":77700,"parent_record_offset":77676,"ancestor_record_offset":76504,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":82952,"parent_record_offset":82908,"ancestor_record_offset":76504,"inlinee":6630,"source_line":418,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":78060,"parent_record_offset":78032,"ancestor_record_offset":76504,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":81108,"parent_record_offset":81084,"ancestor_record_offset":76504,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":84760,"parent_record_offset":84676,"ancestor_record_offset":76504,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":77652,"parent_record_offset":77536,"ancestor_record_offset":76504,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$180::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":84056,"parent_record_offset":84024,"ancestor_record_offset":76504,"inlinee":17824,"source_line":438,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1493,"record_offset":81620,"parent_record_offset":81584,"ancestor_record_offset":76504,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1493,"record_offset":77068,"parent_record_offset":77044,"ancestor_record_offset":76504,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1493,"record_offset":77364,"parent_record_offset":77328,"ancestor_record_offset":76504,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1493,"record_offset":77504,"parent_record_offset":77472,"ancestor_record_offset":76504,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1493,"record_offset":77472,"parent_record_offset":77436,"ancestor_record_offset":76504,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":77412,"parent_record_offset":77388,"ancestor_record_offset":76504,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":77436,"parent_record_offset":77412,"ancestor_record_offset":76504,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":77388,"parent_record_offset":77364,"ancestor_record_offset":76504,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":77976,"parent_record_offset":77436,"ancestor_record_offset":76504,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":77044,"parent_record_offset":77012,"ancestor_record_offset":76504,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":77012,"parent_record_offset":76984,"ancestor_record_offset":76504,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":77168,"parent_record_offset":77136,"ancestor_record_offset":76504,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":77224,"parent_record_offset":77168,"ancestor_record_offset":76504,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":77136,"parent_record_offset":77108,"ancestor_record_offset":76504,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":76984,"parent_record_offset":76940,"ancestor_record_offset":76504,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":77248,"parent_record_offset":77224,"ancestor_record_offset":76504,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":77536,"parent_record_offset":77504,"ancestor_record_offset":76504,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":77820,"parent_record_offset":77536,"ancestor_record_offset":76504,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":77792,"parent_record_offset":77536,"ancestor_record_offset":76504,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":77596,"parent_record_offset":77536,"ancestor_record_offset":76504,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1493,"record_offset":92972,"parent_record_offset":92948,"ancestor_record_offset":76504,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1493,"record_offset":92948,"parent_record_offset":92924,"ancestor_record_offset":76504,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":92792,"parent_record_offset":92760,"ancestor_record_offset":76504,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":92760,"parent_record_offset":92732,"ancestor_record_offset":76504,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":92732,"parent_record_offset":92708,"ancestor_record_offset":76504,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":92196,"parent_record_offset":92168,"ancestor_record_offset":76504,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":92224,"parent_record_offset":92196,"ancestor_record_offset":76504,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":92308,"parent_record_offset":92196,"ancestor_record_offset":76504,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1493,"record_offset":85688,"parent_record_offset":85452,"ancestor_record_offset":76504,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1493,"record_offset":86960,"parent_record_offset":86924,"ancestor_record_offset":76504,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":78376,"parent_record_offset":78340,"ancestor_record_offset":76504,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":79364,"parent_record_offset":79332,"ancestor_record_offset":76504,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":79816,"parent_record_offset":79612,"ancestor_record_offset":76504,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1493,"record_offset":78516,"parent_record_offset":78488,"ancestor_record_offset":76504,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":78588,"parent_record_offset":78448,"ancestor_record_offset":76504,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":78792,"parent_record_offset":78588,"ancestor_record_offset":76504,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::fract
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":88456,"parent_record_offset":88404,"ancestor_record_offset":76504,"inlinee":26640,"source_line":178,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":88428,"parent_record_offset":88404,"ancestor_record_offset":76504,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":92824,"parent_record_offset":92792,"ancestor_record_offset":76504,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1493,"record_offset":78544,"parent_record_offset":78516,"ancestor_record_offset":76504,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1493,"record_offset":78448,"parent_record_offset":78412,"ancestor_record_offset":76504,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1493,"record_offset":78412,"parent_record_offset":78376,"ancestor_record_offset":76504,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::sine_wave::random_f
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":88404,"parent_record_offset":88356,"ancestor_record_offset":76504,"inlinee":26639,"source_line":82,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::sine_wave::sine_wave_deform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":90976,"parent_record_offset":90948,"ancestor_record_offset":76504,"inlinee":26645,"source_line":45,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":81176,"parent_record_offset":81148,"ancestor_record_offset":76504,"inlinee":11021,"source_line":142,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":86924,"parent_record_offset":86888,"ancestor_record_offset":76504,"inlinee":10929,"source_line":338,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":90948,"parent_record_offset":90920,"ancestor_record_offset":76504,"inlinee":17829,"source_line":528,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$9
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":92080,"parent_record_offset":92056,"ancestor_record_offset":76504,"inlinee":17830,"source_line":630,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1493,"record_offset":92108,"parent_record_offset":92080,"ancestor_record_offset":76504,"inlinee":14739,"source_line":803,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1493,"record_offset":92140,"parent_record_offset":92108,"ancestor_record_offset":76504,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1493,"record_offset":89072,"parent_record_offset":88996,"ancestor_record_offset":76504,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1493,"record_offset":89044,"parent_record_offset":88996,"ancestor_record_offset":76504,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1493,"record_offset":84896,"parent_record_offset":84872,"ancestor_record_offset":76504,"inlinee":17826,"source_line":6,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1493,"record_offset":92168,"parent_record_offset":92108,"ancestor_record_offset":76504,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1493,"record_offset":25888,"rva":21282848,"va":5389991968}
// 编译实例 RVAs: 0x144c020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-14314262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8ea9425ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5389992945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=22
//     evidence={"assembly":"\tcallq\t-14318076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8048625ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5389996775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=6
//     target_aliases=1699 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21270743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8296fbbfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5389992930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ff020
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18143273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8d727ebfe","size":5,"soft_fail":false,"target_va":5371850784,"va":5389994052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14164432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e830de27ff","size":5,"soft_fail":false,"target_va":5375829056,"va":5389993483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11880378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e846b84aff","size":5,"soft_fail":false,"target_va":5378120480,"va":5390000853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140956fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11498387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e86d8c50ff","size":5,"soft_fail":false,"target_va":5378502608,"va":5390000990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6721691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8656f99ff","size":5,"soft_fail":false,"target_va":5383273968,"va":5389995654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6675301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e89b249aff","size":5,"soft_fail":false,"target_va":5383325024,"va":5390000320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6675711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e801239aff","size":5,"soft_fail":false,"target_va":5383325056,"va":5390000762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141403be0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-305168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8f057fbff","size":5,"soft_fail":false,"target_va":5389695968,"va":5390001131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405de0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-295448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8e87dfbff","size":5,"soft_fail":false,"target_va":5389704672,"va":5390000115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14142afd0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-140790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e80adafdff","size":5,"soft_fail":false,"target_va":5389856720,"va":5389997505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146cea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t124655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8efe60100","size":5,"soft_fail":false,"target_va":5390126752,"va":5390002092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141478800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t180650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8aac10200","size":5,"soft_fail":false,"target_va":5390174208,"va":5389993553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26355821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e86d289201","size":5,"soft_fail":false,"target_va":5416350720,"va":5389994894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26357723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8db2f9201","size":5,"soft_fail":false,"target_va":5416353040,"va":5389995312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27062095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e84fef9c01","size":5,"soft_fail":false,"target_va":5417057088,"va":5389994988,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27063900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e85cf69c01","size":5,"soft_fail":false,"target_va":5417057664,"va":5389993759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27060816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e850ea9c01","size":5,"soft_fail":false,"target_va":5417058032,"va":5389997211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bf80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27060933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8c5ea9c01","size":5,"soft_fail":false,"target_va":5417058176,"va":5389997238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26618623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8ff2a9601","size":5,"soft_fail":false,"target_va":5416620769,"va":5390002141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12625339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8455a3fff","size":5,"soft_fail":false,"target_va":5377374288,"va":5389999622,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26370392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e858619201","size":5,"soft_fail":false,"target_va":5416366080,"va":5389995683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26626914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8624b9601","size":5,"soft_fail":false,"target_va":5416620794,"va":5389993875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26619360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8e02d9601","size":5,"soft_fail":false,"target_va":5416621408,"va":5390002043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed::<usize, usize>
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26619256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8782d9601","size":5,"soft_fail":false,"target_va":5416621188,"va":5390001927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26620325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8a5319601","size":5,"soft_fail":false,"target_va":5416622208,"va":5390001878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26619483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e85b2e9601","size":5,"soft_fail":false,"target_va":5416621762,"va":5390002274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t144133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e805330200","size":5,"soft_fail":false,"target_va":5390146576,"va":5390002438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t42564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e844a60000","size":5,"soft_fail":false,"target_va":5390034704,"va":5389992135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26619030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8962c9601","size":5,"soft_fail":false,"target_va":5416621376,"va":5390002341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::BrickTransform::encode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11519439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8313a50ff","size":5,"soft_fail":false,"target_va":5378481680,"va":5390001114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::FinalizedWallHoles::wall_length
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8381340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8641c80ff","size":5,"soft_fail":false,"target_va":5381612160,"va":5389993495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6614981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e83b109bff","size":5,"soft_fail":false,"target_va":5383383152,"va":5389998128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6028717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e85302a4ff","size":5,"soft_fail":false,"target_va":5383963792,"va":5389992504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6020210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e88e23a4ff","size":5,"soft_fail":false,"target_va":5383972224,"va":5389992429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6027960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e84805a4ff","size":5,"soft_fail":false,"target_va":5383964912,"va":5389992867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6029297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e80f00a4ff","size":5,"soft_fail":false,"target_va":5383962816,"va":5389992108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-6592669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e863679bff","size":5,"soft_fail":false,"target_va":5383408256,"va":5390000920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26694950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e826559701","size":5,"soft_fail":false,"target_va":5416697008,"va":5390002053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-234660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e85c6bfcff","size":5,"soft_fail":false,"target_va":5389771136,"va":5390005791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26617156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e844259601","size":5,"soft_fail":false,"target_va":5416619360,"va":5390002199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::perturb_row_ends
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12634908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8e4343fff","size":5,"soft_fail":false,"target_va":5377364624,"va":5389999527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::split_horizontally
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12635015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e879343fff","size":5,"soft_fail":false,"target_va":5377365056,"va":5390000066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::split_into_random_rows::split_into_random_rows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12627528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8b8513fff","size":5,"soft_fail":false,"target_va":5377365648,"va":5389993171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::trim_rows::trim_rows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12623267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e85d623fff","size":5,"soft_fail":false,"target_va":5377370208,"va":5389993470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::wall_constructor::split_brick_by_holes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12834901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8ab273cff","size":5,"soft_fail":false,"target_va":5377164912,"va":5389999808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6698407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e859ca99ff","size":5,"soft_fail":false,"target_va":5383302240,"va":5390000642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::expanded
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6692482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e87ee199ff","size":5,"soft_fail":false,"target_va":5383302352,"va":5389994829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6698346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e896ca99ff","size":5,"soft_fail":false,"target_va":5383302288,"va":5390000629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6693122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e8fede99ff","size":5,"soft_fail":false,"target_va":5383301744,"va":5389994861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6698397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e863ca99ff","size":5,"soft_fail":false,"target_va":5383302224,"va":5390000616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6719909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389991968,"raw_hex":"e85b7699ff","size":5,"soft_fail":false,"target_va":5383276416,"va":5389996320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1493,"record_offset":29724,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1493,"record_offset":29440,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":40616,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":13888,"source_line":1920,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":40676,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":5253,"source_line":2205,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":31188,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":10044,"source_line":798,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":42360,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":14132,"source_line":3663,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\iter.rs
//     evidence={"module":1493,"record_offset":43020,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":7433,"source_line":92,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":32424,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":31132,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":32452,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":32724,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":11070,"source_line":3465,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":31520,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":39888,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":35792,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":34196,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1493,"record_offset":42248,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":28660,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":33472,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1493,"record_offset":33800,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":1493,"record_offset":32200,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":33312,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1493,"record_offset":39804,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":30004,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":32060,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1493,"record_offset":34252,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1493,"record_offset":39984,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":40216,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":30840,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":40244,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1493,"record_offset":28700,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1493,"record_offset":37272,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":28548,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":37412,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1493,"record_offset":43416,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":43164,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":42032,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":42332,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":41976,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1493,"record_offset":34776,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1493,"record_offset":37512,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":27740,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":26340,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":26728,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":31464,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs
//     evidence={"module":1493,"record_offset":43668,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":79756,"source_line":235,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::BrickRowSine::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":37680,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":26643,"source_line":12,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::sine_wave_deform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":40448,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":26645,"source_line":45,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::sine_wave_pack
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":40588,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":26644,"source_line":73,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::WallConstructor::normalize_holes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":30868,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":26334,"source_line":56,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$5
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":38320,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":17827,"source_line":379,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1493,"record_offset":41708,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":14739,"source_line":803,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1493,"record_offset":38292,"parent_record_offset":25888,"ancestor_record_offset":25888,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":29528,"parent_record_offset":29504,"ancestor_record_offset":25888,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":29552,"parent_record_offset":29528,"ancestor_record_offset":25888,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":35544,"parent_record_offset":35520,"ancestor_record_offset":25888,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":35568,"parent_record_offset":35544,"ancestor_record_offset":25888,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":29576,"parent_record_offset":29552,"ancestor_record_offset":25888,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":35592,"parent_record_offset":35568,"ancestor_record_offset":25888,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":29504,"parent_record_offset":29472,"ancestor_record_offset":25888,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":35520,"parent_record_offset":35404,"ancestor_record_offset":25888,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1493,"record_offset":29472,"parent_record_offset":29440,"ancestor_record_offset":25888,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_slices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":42412,"parent_record_offset":42384,"ancestor_record_offset":25888,"inlinee":11741,"source_line":1563,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_range
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":42636,"parent_record_offset":42412,"ancestor_record_offset":25888,"inlinee":4789,"source_line":228,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":40760,"parent_record_offset":40736,"ancestor_record_offset":25888,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":40736,"parent_record_offset":40700,"ancestor_record_offset":25888,"inlinee":5255,"source_line":236,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":42384,"parent_record_offset":42360,"ancestor_record_offset":25888,"inlinee":11740,"source_line":1499,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":41020,"parent_record_offset":40848,"ancestor_record_offset":25888,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::push_back_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":40700,"parent_record_offset":40676,"ancestor_record_offset":25888,"inlinee":5254,"source_line":2223,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::slice_ranges
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":42440,"parent_record_offset":42412,"ancestor_record_offset":25888,"inlinee":4783,"source_line":1666,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":40848,"parent_record_offset":40700,"ancestor_record_offset":25888,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::truncate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":40644,"parent_record_offset":40616,"ancestor_record_offset":25888,"inlinee":13889,"source_line":1364,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":31216,"parent_record_offset":31188,"ancestor_record_offset":25888,"inlinee":10045,"source_line":853,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":40884,"parent_record_offset":40848,"ancestor_record_offset":25888,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":40784,"parent_record_offset":40760,"ancestor_record_offset":25888,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":32544,"parent_record_offset":32520,"ancestor_record_offset":25888,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":32844,"parent_record_offset":32816,"ancestor_record_offset":25888,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":30192,"parent_record_offset":30164,"ancestor_record_offset":25888,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":32928,"parent_record_offset":32904,"ancestor_record_offset":25888,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":35444,"parent_record_offset":35404,"ancestor_record_offset":25888,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":35404,"parent_record_offset":35368,"ancestor_record_offset":25888,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":32904,"parent_record_offset":32872,"ancestor_record_offset":25888,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":32592,"parent_record_offset":32568,"ancestor_record_offset":25888,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":32568,"parent_record_offset":32544,"ancestor_record_offset":25888,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":32872,"parent_record_offset":32844,"ancestor_record_offset":25888,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":30252,"parent_record_offset":30220,"ancestor_record_offset":25888,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":30220,"parent_record_offset":30192,"ancestor_record_offset":25888,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":35368,"parent_record_offset":35336,"ancestor_record_offset":25888,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":31684,"parent_record_offset":31640,"ancestor_record_offset":25888,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1493,"record_offset":33740,"parent_record_offset":33716,"ancestor_record_offset":25888,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1493,"record_offset":30948,"parent_record_offset":30868,"ancestor_record_offset":25888,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1493,"record_offset":30984,"parent_record_offset":30948,"ancestor_record_offset":25888,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":32520,"parent_record_offset":32480,"ancestor_record_offset":25888,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":30920,"parent_record_offset":30868,"ancestor_record_offset":25888,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":30436,"parent_record_offset":30412,"ancestor_record_offset":25888,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":32756,"parent_record_offset":32724,"ancestor_record_offset":25888,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":33764,"parent_record_offset":33740,"ancestor_record_offset":25888,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":32480,"parent_record_offset":32452,"ancestor_record_offset":25888,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":32816,"parent_record_offset":32756,"ancestor_record_offset":25888,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":30136,"parent_record_offset":30100,"ancestor_record_offset":25888,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":30164,"parent_record_offset":30136,"ancestor_record_offset":25888,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":30036,"parent_record_offset":30004,"ancestor_record_offset":25888,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":30700,"parent_record_offset":30672,"ancestor_record_offset":25888,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1493,"record_offset":30764,"parent_record_offset":30700,"ancestor_record_offset":25888,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1493,"record_offset":36940,"parent_record_offset":35848,"ancestor_record_offset":25888,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1493,"record_offset":36768,"parent_record_offset":36740,"ancestor_record_offset":25888,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1493,"record_offset":30412,"parent_record_offset":30100,"ancestor_record_offset":25888,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1493,"record_offset":30068,"parent_record_offset":30036,"ancestor_record_offset":25888,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1493,"record_offset":30100,"parent_record_offset":30068,"ancestor_record_offset":25888,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1493,"record_offset":31712,"parent_record_offset":31684,"ancestor_record_offset":25888,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":28068,"parent_record_offset":28040,"ancestor_record_offset":25888,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":28096,"parent_record_offset":28068,"ancestor_record_offset":25888,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":28016,"parent_record_offset":27988,"ancestor_record_offset":25888,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":28040,"parent_record_offset":28016,"ancestor_record_offset":25888,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":28156,"parent_record_offset":28040,"ancestor_record_offset":25888,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":28268,"parent_record_offset":28244,"ancestor_record_offset":25888,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":32156,"parent_record_offset":32132,"ancestor_record_offset":25888,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":34300,"parent_record_offset":34276,"ancestor_record_offset":25888,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":34324,"parent_record_offset":34300,"ancestor_record_offset":25888,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":33180,"parent_record_offset":33148,"ancestor_record_offset":25888,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":32132,"parent_record_offset":32108,"ancestor_record_offset":25888,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":1493,"record_offset":43464,"parent_record_offset":43440,"ancestor_record_offset":25888,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1493,"record_offset":43496,"parent_record_offset":43416,"ancestor_record_offset":25888,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1493,"record_offset":43440,"parent_record_offset":43416,"ancestor_record_offset":25888,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":31488,"parent_record_offset":31464,"ancestor_record_offset":25888,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::fract
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":37804,"parent_record_offset":37780,"ancestor_record_offset":25888,"inlinee":26641,"source_line":1873,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":42172,"parent_record_offset":42148,"ancestor_record_offset":25888,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::trunc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":37828,"parent_record_offset":37804,"ancestor_record_offset":25888,"inlinee":21635,"source_line":1844,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1493,"record_offset":31060,"parent_record_offset":31020,"ancestor_record_offset":25888,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1493,"record_offset":31804,"parent_record_offset":31740,"ancestor_record_offset":25888,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1493,"record_offset":35036,"parent_record_offset":35012,"ancestor_record_offset":25888,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1493,"record_offset":35084,"parent_record_offset":35060,"ancestor_record_offset":25888,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1493,"record_offset":34824,"parent_record_offset":34776,"ancestor_record_offset":25888,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1493,"record_offset":38668,"parent_record_offset":38628,"ancestor_record_offset":25888,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1493,"record_offset":32236,"parent_record_offset":32200,"ancestor_record_offset":25888,"inlinee":17819,"source_line":180,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1493,"record_offset":33872,"parent_record_offset":33828,"ancestor_record_offset":25888,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1493,"record_offset":33828,"parent_record_offset":33800,"ancestor_record_offset":25888,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1493,"record_offset":33972,"parent_record_offset":33944,"ancestor_record_offset":25888,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1493,"record_offset":33944,"parent_record_offset":33828,"ancestor_record_offset":25888,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1493,"record_offset":30484,"parent_record_offset":30460,"ancestor_record_offset":25888,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1493,"record_offset":34000,"parent_record_offset":33972,"ancestor_record_offset":25888,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1493,"record_offset":30548,"parent_record_offset":30508,"ancestor_record_offset":25888,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":1493,"record_offset":33368,"parent_record_offset":33340,"ancestor_record_offset":25888,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":32084,"parent_record_offset":32060,"ancestor_record_offset":25888,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":32272,"parent_record_offset":32236,"ancestor_record_offset":25888,"inlinee":17821,"source_line":124,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":33340,"parent_record_offset":33312,"ancestor_record_offset":25888,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":32108,"parent_record_offset":32084,"ancestor_record_offset":25888,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":32308,"parent_record_offset":32272,"ancestor_record_offset":25888,"inlinee":17822,"source_line":355,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1493,"record_offset":33148,"parent_record_offset":33116,"ancestor_record_offset":25888,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1493,"record_offset":33116,"parent_record_offset":32756,"ancestor_record_offset":25888,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1493,"record_offset":32024,"parent_record_offset":32000,"ancestor_record_offset":25888,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":30460,"parent_record_offset":30436,"ancestor_record_offset":25888,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":30672,"parent_record_offset":30548,"ancestor_record_offset":25888,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":35108,"parent_record_offset":35084,"ancestor_record_offset":25888,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1493,"record_offset":28120,"parent_record_offset":28096,"ancestor_record_offset":25888,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1493,"record_offset":31740,"parent_record_offset":31712,"ancestor_record_offset":25888,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1493,"record_offset":31772,"parent_record_offset":31740,"ancestor_record_offset":25888,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1493,"record_offset":35488,"parent_record_offset":35444,"ancestor_record_offset":25888,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1493,"record_offset":30632,"parent_record_offset":30604,"ancestor_record_offset":25888,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1493,"record_offset":36360,"parent_record_offset":36284,"ancestor_record_offset":25888,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1493,"record_offset":30604,"parent_record_offset":30576,"ancestor_record_offset":25888,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1493,"record_offset":36320,"parent_record_offset":36284,"ancestor_record_offset":25888,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1493,"record_offset":27888,"parent_record_offset":27848,"ancestor_record_offset":25888,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1493,"record_offset":34052,"parent_record_offset":34028,"ancestor_record_offset":25888,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1493,"record_offset":34276,"parent_record_offset":34252,"ancestor_record_offset":25888,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":33904,"parent_record_offset":33872,"ancestor_record_offset":25888,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":34976,"parent_record_offset":34824,"ancestor_record_offset":25888,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":34028,"parent_record_offset":34000,"ancestor_record_offset":25888,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":35060,"parent_record_offset":35036,"ancestor_record_offset":25888,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":39080,"parent_record_offset":38628,"ancestor_record_offset":25888,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1493,"record_offset":26676,"parent_record_offset":26648,"ancestor_record_offset":25888,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1493,"record_offset":28504,"parent_record_offset":27848,"ancestor_record_offset":25888,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1493,"record_offset":28220,"parent_record_offset":27988,"ancestor_record_offset":25888,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1493,"record_offset":32376,"parent_record_offset":32352,"ancestor_record_offset":25888,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":28596,"parent_record_offset":28548,"ancestor_record_offset":25888,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":27148,"parent_record_offset":27124,"ancestor_record_offset":25888,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":27124,"parent_record_offset":27100,"ancestor_record_offset":25888,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":27332,"parent_record_offset":27304,"ancestor_record_offset":25888,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1493,"record_offset":30732,"parent_record_offset":30700,"ancestor_record_offset":25888,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1493,"record_offset":27432,"parent_record_offset":27404,"ancestor_record_offset":25888,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1493,"record_offset":27404,"parent_record_offset":27376,"ancestor_record_offset":25888,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1493,"record_offset":42788,"parent_record_offset":42384,"ancestor_record_offset":25888,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1493,"record_offset":39916,"parent_record_offset":39888,"ancestor_record_offset":25888,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1493,"record_offset":39944,"parent_record_offset":39916,"ancestor_record_offset":25888,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1493,"record_offset":37296,"parent_record_offset":37272,"ancestor_record_offset":25888,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":27076,"parent_record_offset":27052,"ancestor_record_offset":25888,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":27100,"parent_record_offset":27076,"ancestor_record_offset":25888,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":32352,"parent_record_offset":32308,"ancestor_record_offset":25888,"inlinee":6630,"source_line":418,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":27460,"parent_record_offset":27432,"ancestor_record_offset":25888,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":30508,"parent_record_offset":30484,"ancestor_record_offset":25888,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":34084,"parent_record_offset":34000,"ancestor_record_offset":25888,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":27052,"parent_record_offset":26936,"ancestor_record_offset":25888,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$180::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":33396,"parent_record_offset":33368,"ancestor_record_offset":25888,"inlinee":17824,"source_line":438,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1493,"record_offset":31020,"parent_record_offset":30984,"ancestor_record_offset":25888,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1493,"record_offset":26468,"parent_record_offset":26444,"ancestor_record_offset":25888,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1493,"record_offset":26764,"parent_record_offset":26728,"ancestor_record_offset":25888,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1493,"record_offset":26904,"parent_record_offset":26872,"ancestor_record_offset":25888,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1493,"record_offset":26872,"parent_record_offset":26836,"ancestor_record_offset":25888,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":26812,"parent_record_offset":26788,"ancestor_record_offset":25888,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":26836,"parent_record_offset":26812,"ancestor_record_offset":25888,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":26788,"parent_record_offset":26764,"ancestor_record_offset":25888,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":27376,"parent_record_offset":26836,"ancestor_record_offset":25888,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":26444,"parent_record_offset":26412,"ancestor_record_offset":25888,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":26412,"parent_record_offset":26384,"ancestor_record_offset":25888,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":26568,"parent_record_offset":26536,"ancestor_record_offset":25888,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":26624,"parent_record_offset":26568,"ancestor_record_offset":25888,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":26536,"parent_record_offset":26508,"ancestor_record_offset":25888,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":26384,"parent_record_offset":26340,"ancestor_record_offset":25888,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":26648,"parent_record_offset":26624,"ancestor_record_offset":25888,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":26936,"parent_record_offset":26904,"ancestor_record_offset":25888,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":27220,"parent_record_offset":26936,"ancestor_record_offset":25888,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":27192,"parent_record_offset":26936,"ancestor_record_offset":25888,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":26996,"parent_record_offset":26936,"ancestor_record_offset":25888,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1493,"record_offset":42296,"parent_record_offset":42272,"ancestor_record_offset":25888,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1493,"record_offset":42272,"parent_record_offset":42248,"ancestor_record_offset":25888,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":42116,"parent_record_offset":42084,"ancestor_record_offset":25888,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":42084,"parent_record_offset":42056,"ancestor_record_offset":25888,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":42056,"parent_record_offset":42032,"ancestor_record_offset":25888,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":41520,"parent_record_offset":41492,"ancestor_record_offset":25888,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":41548,"parent_record_offset":41520,"ancestor_record_offset":25888,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":41632,"parent_record_offset":41520,"ancestor_record_offset":25888,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1493,"record_offset":35012,"parent_record_offset":34776,"ancestor_record_offset":25888,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1493,"record_offset":36284,"parent_record_offset":36248,"ancestor_record_offset":25888,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":27776,"parent_record_offset":27740,"ancestor_record_offset":25888,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":28764,"parent_record_offset":28732,"ancestor_record_offset":25888,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":29216,"parent_record_offset":29012,"ancestor_record_offset":25888,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1493,"record_offset":27916,"parent_record_offset":27888,"ancestor_record_offset":25888,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":27988,"parent_record_offset":27848,"ancestor_record_offset":25888,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":28192,"parent_record_offset":27988,"ancestor_record_offset":25888,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::fract
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":37780,"parent_record_offset":37728,"ancestor_record_offset":25888,"inlinee":26640,"source_line":178,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":37752,"parent_record_offset":37728,"ancestor_record_offset":25888,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":42148,"parent_record_offset":42116,"ancestor_record_offset":25888,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1493,"record_offset":27944,"parent_record_offset":27916,"ancestor_record_offset":25888,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1493,"record_offset":27848,"parent_record_offset":27812,"ancestor_record_offset":25888,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1493,"record_offset":27812,"parent_record_offset":27776,"ancestor_record_offset":25888,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::sine_wave::random_f
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":37728,"parent_record_offset":37680,"ancestor_record_offset":25888,"inlinee":26639,"source_line":82,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::sine_wave::sine_wave_deform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":40300,"parent_record_offset":40272,"ancestor_record_offset":25888,"inlinee":26645,"source_line":45,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":30576,"parent_record_offset":30548,"ancestor_record_offset":25888,"inlinee":11021,"source_line":142,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":36248,"parent_record_offset":36212,"ancestor_record_offset":25888,"inlinee":10929,"source_line":338,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":40272,"parent_record_offset":40244,"ancestor_record_offset":25888,"inlinee":17829,"source_line":528,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$9
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":41404,"parent_record_offset":41380,"ancestor_record_offset":25888,"inlinee":17830,"source_line":630,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1493,"record_offset":41432,"parent_record_offset":41404,"ancestor_record_offset":25888,"inlinee":14739,"source_line":803,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1493,"record_offset":41464,"parent_record_offset":41432,"ancestor_record_offset":25888,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1493,"record_offset":38396,"parent_record_offset":38320,"ancestor_record_offset":25888,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1493,"record_offset":38368,"parent_record_offset":38320,"ancestor_record_offset":25888,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1493,"record_offset":34220,"parent_record_offset":34196,"ancestor_record_offset":25888,"inlinee":17826,"source_line":6,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1493,"record_offset":41492,"parent_record_offset":41432,"ancestor_record_offset":25888,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":249,"record_offset":24596,"rva":6786160,"va":5375495280}
// 编译实例 RVAs: 0x678c70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t182422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e896c80200","size":5,"soft_fail":false,"target_va":5375678688,"va":5375496261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=22
//     evidence={"assembly":"\tcallq\t180735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8ffc10200","size":5,"soft_fail":false,"target_va":5375678704,"va":5375497964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=6
//     target_aliases=1699 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6774059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8d5a298ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375496246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140139520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5516692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e86cd2abff","size":5,"soft_fail":false,"target_va":5369992480,"va":5375509167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ff020
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3646517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8cb5bc8ff","size":5,"soft_fail":false,"target_va":5371850784,"va":5375497296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a3c90
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1928281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8a793e2ff","size":5,"soft_fail":false,"target_va":5373574288,"va":5375502564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140547c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1258189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e833cdecff","size":5,"soft_fail":false,"target_va":5374245920,"va":5375504104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140559870
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1181270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8aaf9edff","size":5,"soft_fail":false,"target_va":5374318704,"va":5375499969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t332237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8cd110500","size":5,"soft_fail":false,"target_va":5375829056,"va":5375496814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2616874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e82aee2700","size":5,"soft_fail":false,"target_va":5378120480,"va":5375503601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7774595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e883a17600","size":5,"soft_fail":false,"target_va":5383273968,"va":5375499368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7822247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8a75b7700","size":5,"soft_fail":false,"target_va":5383325024,"va":5375502772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7821788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8dc597700","size":5,"soft_fail":false,"target_va":5383325056,"va":5375503263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40851950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8ee596f02","size":5,"soft_fail":false,"target_va":5416350720,"va":5375498765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40847864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8f8496f02","size":5,"soft_fail":false,"target_va":5416353040,"va":5375505171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70990
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40850743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e837556f02","size":5,"soft_fail":false,"target_va":5416356240,"va":5375505492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41558223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8cf207a02","size":5,"soft_fail":false,"target_va":5417057088,"va":5375498860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41560655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e84f2a7a02","size":5,"soft_fail":false,"target_va":5417057664,"va":5375497004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t41115218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8525e7302","size":5,"soft_fail":false,"target_va":5416620769,"va":5375505546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1872206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e84e911c00","size":5,"soft_fail":false,"target_va":5377374288,"va":5375502077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40866675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e873936f02","size":5,"soft_fail":false,"target_va":5416366080,"va":5375499400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t41123669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8557f7302","size":5,"soft_fail":false,"target_va":5416620794,"va":5375497120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41115921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e811617302","size":5,"soft_fail":false,"target_va":5416621408,"va":5375505482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed::<usize, usize>
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t41115817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8a9607302","size":5,"soft_fail":false,"target_va":5416621188,"va":5375505366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41116886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8d6647302","size":5,"soft_fail":false,"target_va":5416622208,"va":5375505317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41116078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8ae617302","size":5,"soft_fail":false,"target_va":5416621762,"va":5375505679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e801360000","size":5,"soft_fail":false,"target_va":5375509280,"va":5375495450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t41115625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8e95f7302","size":5,"soft_fail":false,"target_va":5416621376,"va":5375505746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::BrickTransform::encode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2977591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8376f2d00","size":5,"soft_fail":false,"target_va":5378481680,"va":5375504084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7882511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e80f477800","size":5,"soft_fail":false,"target_va":5383383152,"va":5375500636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8467985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e811368100","size":5,"soft_fail":false,"target_va":5383963792,"va":5375495802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8476492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e84c578100","size":5,"soft_fail":false,"target_va":5383972224,"va":5375495727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8468729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8f9388100","size":5,"soft_fail":false,"target_va":5383964912,"va":5375496178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8467387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8bb338100","size":5,"soft_fail":false,"target_va":5383962816,"va":5375495424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7904397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e88d9c7800","size":5,"soft_fail":false,"target_va":5383408256,"va":5375503854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41113751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e897587302","size":5,"soft_fail":false,"target_va":5416619360,"va":5375505604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::perturb_row_ends
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1862637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8ed6b1c00","size":5,"soft_fail":false,"target_va":5377364624,"va":5375501982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::sine_wave::BrickRowSine::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1869250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8c2851c00","size":5,"soft_fail":false,"target_va":5377369120,"va":5375499865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::sine_wave::sine_wave_deform
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1867859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e853801c00","size":5,"soft_fail":false,"target_va":5377369552,"va":5375501688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::sine_wave::sine_wave_pack
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1867742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8de7f1c00","size":5,"soft_fail":false,"target_va":5377369520,"va":5375501773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::split_horizontally
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1862536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8886b1c00","size":5,"soft_fail":false,"target_va":5377365056,"va":5375502515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::split_into_random_rows::split_into_random_rows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1869156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e864851c00","size":5,"soft_fail":false,"target_va":5377365648,"va":5375496487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::trim_rows::trim_rows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1873402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8fa951c00","size":5,"soft_fail":false,"target_va":5377370208,"va":5375496801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::wall_constructor::WallConstructor::normalize_holes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1667910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e846731900","size":5,"soft_fail":false,"target_va":5377164752,"va":5375496837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::wall_constructor::split_brick_by_holes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1662637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8ad5e1900","size":5,"soft_fail":false,"target_va":5377164912,"va":5375502270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7799113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e849017700","size":5,"soft_fail":false,"target_va":5383302240,"va":5375503122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::expanded
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7803654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e806137700","size":5,"soft_fail":false,"target_va":5383302352,"va":5375498693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7799174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e886017700","size":5,"soft_fail":false,"target_va":5383302288,"va":5375503109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7803011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e883107700","size":5,"soft_fail":false,"target_va":5383301744,"va":5375498728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7799123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e853017700","size":5,"soft_fail":false,"target_va":5383302224,"va":5375503096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7771815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8a7967600","size":5,"soft_fail":false,"target_va":5383276416,"va":5375504596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":249,"record_offset":28456,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":249,"record_offset":28172,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":40812,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":13888,"source_line":1920,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":40872,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":5253,"source_line":2205,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":29656,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":10044,"source_line":798,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":42556,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":14132,"source_line":3663,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\iter.rs
//     evidence={"module":249,"record_offset":43216,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":7433,"source_line":92,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":34584,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":29600,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":34612,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":34884,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":11070,"source_line":3465,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":29988,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":40336,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":44072,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":249,"record_offset":36404,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":249,"record_offset":42444,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":249,"record_offset":43612,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":249,"record_offset":27392,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":249,"record_offset":35680,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":249,"record_offset":36008,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":249,"record_offset":30668,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":249,"record_offset":35520,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":249,"record_offset":40252,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":249,"record_offset":28736,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":249,"record_offset":30528,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":249,"record_offset":36460,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":249,"record_offset":40432,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":40664,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":29572,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":40692,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":249,"record_offset":27432,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":249,"record_offset":37816,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":27280,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":37956,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":249,"record_offset":43640,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":43360,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":42228,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":42528,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":42172,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":249,"record_offset":36984,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":249,"record_offset":38056,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":26472,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":25064,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":25460,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":249,"record_offset":29932,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$9
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_playermade_stairs/mod.rs
//     evidence={"module":249,"record_offset":43892,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":17836,"source_line":903,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$5
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":249,"record_offset":38252,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":17827,"source_line":379,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":249,"record_offset":41904,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":14739,"source_line":803,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":249,"record_offset":38224,"parent_record_offset":24596,"ancestor_record_offset":24596,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":28260,"parent_record_offset":28236,"ancestor_record_offset":24596,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":28284,"parent_record_offset":28260,"ancestor_record_offset":24596,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":31080,"parent_record_offset":31056,"ancestor_record_offset":24596,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":31104,"parent_record_offset":31080,"ancestor_record_offset":24596,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":28308,"parent_record_offset":28284,"ancestor_record_offset":24596,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":31128,"parent_record_offset":31104,"ancestor_record_offset":24596,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":28236,"parent_record_offset":28204,"ancestor_record_offset":24596,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":31056,"parent_record_offset":30964,"ancestor_record_offset":24596,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":249,"record_offset":28204,"parent_record_offset":28172,"ancestor_record_offset":24596,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_slices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":42608,"parent_record_offset":42580,"ancestor_record_offset":24596,"inlinee":11741,"source_line":1563,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_range
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":42832,"parent_record_offset":42608,"ancestor_record_offset":24596,"inlinee":4789,"source_line":228,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":40956,"parent_record_offset":40932,"ancestor_record_offset":24596,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":40932,"parent_record_offset":40896,"ancestor_record_offset":24596,"inlinee":5255,"source_line":236,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":42580,"parent_record_offset":42556,"ancestor_record_offset":24596,"inlinee":11740,"source_line":1499,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":41216,"parent_record_offset":41044,"ancestor_record_offset":24596,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::push_back_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":40896,"parent_record_offset":40872,"ancestor_record_offset":24596,"inlinee":5254,"source_line":2223,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::slice_ranges
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":42636,"parent_record_offset":42608,"ancestor_record_offset":24596,"inlinee":4783,"source_line":1666,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":41044,"parent_record_offset":40896,"ancestor_record_offset":24596,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::truncate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":40840,"parent_record_offset":40812,"ancestor_record_offset":24596,"inlinee":13889,"source_line":1364,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":29684,"parent_record_offset":29656,"ancestor_record_offset":24596,"inlinee":10045,"source_line":853,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":41080,"parent_record_offset":41044,"ancestor_record_offset":24596,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":40980,"parent_record_offset":40956,"ancestor_record_offset":24596,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":34704,"parent_record_offset":34680,"ancestor_record_offset":24596,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":35032,"parent_record_offset":35004,"ancestor_record_offset":24596,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":28924,"parent_record_offset":28896,"ancestor_record_offset":24596,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":35116,"parent_record_offset":35092,"ancestor_record_offset":24596,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":30992,"parent_record_offset":30964,"ancestor_record_offset":24596,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":30964,"parent_record_offset":30940,"ancestor_record_offset":24596,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":35092,"parent_record_offset":35060,"ancestor_record_offset":24596,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":34752,"parent_record_offset":34728,"ancestor_record_offset":24596,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":34728,"parent_record_offset":34704,"ancestor_record_offset":24596,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":35060,"parent_record_offset":35032,"ancestor_record_offset":24596,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":28984,"parent_record_offset":28952,"ancestor_record_offset":24596,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":28952,"parent_record_offset":28924,"ancestor_record_offset":24596,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":30940,"parent_record_offset":30916,"ancestor_record_offset":24596,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":30152,"parent_record_offset":30108,"ancestor_record_offset":24596,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":249,"record_offset":35948,"parent_record_offset":35924,"ancestor_record_offset":24596,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":34680,"parent_record_offset":34640,"ancestor_record_offset":24596,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":29168,"parent_record_offset":29144,"ancestor_record_offset":24596,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":34944,"parent_record_offset":34884,"ancestor_record_offset":24596,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":35972,"parent_record_offset":35948,"ancestor_record_offset":24596,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":34916,"parent_record_offset":34884,"ancestor_record_offset":24596,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":34640,"parent_record_offset":34612,"ancestor_record_offset":24596,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":35004,"parent_record_offset":34944,"ancestor_record_offset":24596,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":28868,"parent_record_offset":28832,"ancestor_record_offset":24596,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":28896,"parent_record_offset":28868,"ancestor_record_offset":24596,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":28768,"parent_record_offset":28736,"ancestor_record_offset":24596,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":29432,"parent_record_offset":29404,"ancestor_record_offset":24596,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":249,"record_offset":29496,"parent_record_offset":29432,"ancestor_record_offset":24596,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":249,"record_offset":45260,"parent_record_offset":44144,"ancestor_record_offset":24596,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":249,"record_offset":45088,"parent_record_offset":45060,"ancestor_record_offset":24596,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":249,"record_offset":29144,"parent_record_offset":28832,"ancestor_record_offset":24596,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":249,"record_offset":28800,"parent_record_offset":28768,"ancestor_record_offset":24596,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":249,"record_offset":28832,"parent_record_offset":28800,"ancestor_record_offset":24596,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":249,"record_offset":30180,"parent_record_offset":30152,"ancestor_record_offset":24596,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":249,"record_offset":26800,"parent_record_offset":26772,"ancestor_record_offset":24596,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":249,"record_offset":26828,"parent_record_offset":26800,"ancestor_record_offset":24596,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":249,"record_offset":26748,"parent_record_offset":26720,"ancestor_record_offset":24596,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":249,"record_offset":26772,"parent_record_offset":26748,"ancestor_record_offset":24596,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":249,"record_offset":26888,"parent_record_offset":26772,"ancestor_record_offset":24596,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":249,"record_offset":27000,"parent_record_offset":26976,"ancestor_record_offset":24596,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":249,"record_offset":30624,"parent_record_offset":30600,"ancestor_record_offset":24596,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":249,"record_offset":36508,"parent_record_offset":36484,"ancestor_record_offset":24596,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":249,"record_offset":36532,"parent_record_offset":36508,"ancestor_record_offset":24596,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":249,"record_offset":35388,"parent_record_offset":35356,"ancestor_record_offset":24596,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":249,"record_offset":30600,"parent_record_offset":30576,"ancestor_record_offset":24596,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":249,"record_offset":43688,"parent_record_offset":43664,"ancestor_record_offset":24596,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":249,"record_offset":43720,"parent_record_offset":43640,"ancestor_record_offset":24596,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":249,"record_offset":43664,"parent_record_offset":43640,"ancestor_record_offset":24596,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":249,"record_offset":29956,"parent_record_offset":29932,"ancestor_record_offset":24596,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":249,"record_offset":42368,"parent_record_offset":42344,"ancestor_record_offset":24596,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":249,"record_offset":30272,"parent_record_offset":30208,"ancestor_record_offset":24596,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":249,"record_offset":37256,"parent_record_offset":37232,"ancestor_record_offset":24596,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":249,"record_offset":37304,"parent_record_offset":37280,"ancestor_record_offset":24596,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":249,"record_offset":37036,"parent_record_offset":36984,"ancestor_record_offset":24596,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":249,"record_offset":38692,"parent_record_offset":38652,"ancestor_record_offset":24596,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":249,"record_offset":30704,"parent_record_offset":30668,"ancestor_record_offset":24596,"inlinee":17819,"source_line":180,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":249,"record_offset":36080,"parent_record_offset":36036,"ancestor_record_offset":24596,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":249,"record_offset":36036,"parent_record_offset":36008,"ancestor_record_offset":24596,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":249,"record_offset":36180,"parent_record_offset":36152,"ancestor_record_offset":24596,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":249,"record_offset":36152,"parent_record_offset":36036,"ancestor_record_offset":24596,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":249,"record_offset":29216,"parent_record_offset":29192,"ancestor_record_offset":24596,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":249,"record_offset":36208,"parent_record_offset":36180,"ancestor_record_offset":24596,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":249,"record_offset":29280,"parent_record_offset":29240,"ancestor_record_offset":24596,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":249,"record_offset":35576,"parent_record_offset":35548,"ancestor_record_offset":24596,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":249,"record_offset":30552,"parent_record_offset":30528,"ancestor_record_offset":24596,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":249,"record_offset":30740,"parent_record_offset":30704,"ancestor_record_offset":24596,"inlinee":17821,"source_line":124,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":249,"record_offset":35548,"parent_record_offset":35520,"ancestor_record_offset":24596,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":249,"record_offset":30576,"parent_record_offset":30552,"ancestor_record_offset":24596,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":249,"record_offset":30776,"parent_record_offset":30740,"ancestor_record_offset":24596,"inlinee":17822,"source_line":355,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":249,"record_offset":35356,"parent_record_offset":35324,"ancestor_record_offset":24596,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":249,"record_offset":35324,"parent_record_offset":34944,"ancestor_record_offset":24596,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":249,"record_offset":30492,"parent_record_offset":30468,"ancestor_record_offset":24596,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":249,"record_offset":29192,"parent_record_offset":29168,"ancestor_record_offset":24596,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":249,"record_offset":29404,"parent_record_offset":29280,"ancestor_record_offset":24596,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":249,"record_offset":37328,"parent_record_offset":37304,"ancestor_record_offset":24596,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":249,"record_offset":26852,"parent_record_offset":26828,"ancestor_record_offset":24596,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":249,"record_offset":30208,"parent_record_offset":30180,"ancestor_record_offset":24596,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":249,"record_offset":30240,"parent_record_offset":30208,"ancestor_record_offset":24596,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":249,"record_offset":31024,"parent_record_offset":30992,"ancestor_record_offset":24596,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":249,"record_offset":29364,"parent_record_offset":29336,"ancestor_record_offset":24596,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":249,"record_offset":38116,"parent_record_offset":38056,"ancestor_record_offset":24596,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":249,"record_offset":29336,"parent_record_offset":29308,"ancestor_record_offset":24596,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":249,"record_offset":38084,"parent_record_offset":38056,"ancestor_record_offset":24596,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":249,"record_offset":26620,"parent_record_offset":26580,"ancestor_record_offset":24596,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":249,"record_offset":36260,"parent_record_offset":36236,"ancestor_record_offset":24596,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":249,"record_offset":36484,"parent_record_offset":36460,"ancestor_record_offset":24596,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":36112,"parent_record_offset":36080,"ancestor_record_offset":24596,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":37196,"parent_record_offset":37036,"ancestor_record_offset":24596,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":36236,"parent_record_offset":36208,"ancestor_record_offset":24596,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":37280,"parent_record_offset":37256,"ancestor_record_offset":24596,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":39104,"parent_record_offset":38652,"ancestor_record_offset":24596,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":249,"record_offset":25408,"parent_record_offset":25380,"ancestor_record_offset":24596,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":249,"record_offset":27236,"parent_record_offset":26580,"ancestor_record_offset":24596,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":249,"record_offset":26952,"parent_record_offset":26720,"ancestor_record_offset":24596,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":249,"record_offset":30844,"parent_record_offset":30820,"ancestor_record_offset":24596,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":249,"record_offset":27328,"parent_record_offset":27280,"ancestor_record_offset":24596,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":249,"record_offset":25880,"parent_record_offset":25856,"ancestor_record_offset":24596,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":249,"record_offset":25856,"parent_record_offset":25832,"ancestor_record_offset":24596,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":249,"record_offset":26064,"parent_record_offset":26036,"ancestor_record_offset":24596,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":249,"record_offset":29464,"parent_record_offset":29432,"ancestor_record_offset":24596,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":249,"record_offset":26164,"parent_record_offset":26136,"ancestor_record_offset":24596,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":249,"record_offset":26136,"parent_record_offset":26108,"ancestor_record_offset":24596,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":249,"record_offset":42984,"parent_record_offset":42580,"ancestor_record_offset":24596,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":249,"record_offset":40364,"parent_record_offset":40336,"ancestor_record_offset":24596,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":249,"record_offset":40392,"parent_record_offset":40364,"ancestor_record_offset":24596,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":249,"record_offset":37840,"parent_record_offset":37816,"ancestor_record_offset":24596,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":25808,"parent_record_offset":25784,"ancestor_record_offset":24596,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":25832,"parent_record_offset":25808,"ancestor_record_offset":24596,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":30820,"parent_record_offset":30776,"ancestor_record_offset":24596,"inlinee":6630,"source_line":418,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":26192,"parent_record_offset":26164,"ancestor_record_offset":24596,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":29240,"parent_record_offset":29216,"ancestor_record_offset":24596,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":36292,"parent_record_offset":36208,"ancestor_record_offset":24596,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":25784,"parent_record_offset":25668,"ancestor_record_offset":24596,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$180::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":35604,"parent_record_offset":35576,"ancestor_record_offset":24596,"inlinee":17824,"source_line":438,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":249,"record_offset":25196,"parent_record_offset":25172,"ancestor_record_offset":24596,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":249,"record_offset":25496,"parent_record_offset":25460,"ancestor_record_offset":24596,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":249,"record_offset":25636,"parent_record_offset":25604,"ancestor_record_offset":24596,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":249,"record_offset":25604,"parent_record_offset":25568,"ancestor_record_offset":24596,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":249,"record_offset":25544,"parent_record_offset":25520,"ancestor_record_offset":24596,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":249,"record_offset":25568,"parent_record_offset":25544,"ancestor_record_offset":24596,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":249,"record_offset":25520,"parent_record_offset":25496,"ancestor_record_offset":24596,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":249,"record_offset":26108,"parent_record_offset":25568,"ancestor_record_offset":24596,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":249,"record_offset":25172,"parent_record_offset":25140,"ancestor_record_offset":24596,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":249,"record_offset":25140,"parent_record_offset":25112,"ancestor_record_offset":24596,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":249,"record_offset":25300,"parent_record_offset":25268,"ancestor_record_offset":24596,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":249,"record_offset":25356,"parent_record_offset":25300,"ancestor_record_offset":24596,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":249,"record_offset":25268,"parent_record_offset":25236,"ancestor_record_offset":24596,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":249,"record_offset":25112,"parent_record_offset":25064,"ancestor_record_offset":24596,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":249,"record_offset":25380,"parent_record_offset":25356,"ancestor_record_offset":24596,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":249,"record_offset":25668,"parent_record_offset":25636,"ancestor_record_offset":24596,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":249,"record_offset":25952,"parent_record_offset":25668,"ancestor_record_offset":24596,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":249,"record_offset":25924,"parent_record_offset":25668,"ancestor_record_offset":24596,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":249,"record_offset":25728,"parent_record_offset":25668,"ancestor_record_offset":24596,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":249,"record_offset":42492,"parent_record_offset":42468,"ancestor_record_offset":24596,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":249,"record_offset":42468,"parent_record_offset":42444,"ancestor_record_offset":24596,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":42312,"parent_record_offset":42280,"ancestor_record_offset":24596,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":42280,"parent_record_offset":42252,"ancestor_record_offset":24596,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":42252,"parent_record_offset":42228,"ancestor_record_offset":24596,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":41716,"parent_record_offset":41688,"ancestor_record_offset":24596,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":41744,"parent_record_offset":41716,"ancestor_record_offset":24596,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":41828,"parent_record_offset":41716,"ancestor_record_offset":24596,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":249,"record_offset":37232,"parent_record_offset":36984,"ancestor_record_offset":24596,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":249,"record_offset":38364,"parent_record_offset":38252,"ancestor_record_offset":24596,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":26508,"parent_record_offset":26472,"ancestor_record_offset":24596,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":27496,"parent_record_offset":27464,"ancestor_record_offset":24596,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":27948,"parent_record_offset":27744,"ancestor_record_offset":24596,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":249,"record_offset":26648,"parent_record_offset":26620,"ancestor_record_offset":24596,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":26720,"parent_record_offset":26580,"ancestor_record_offset":24596,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":26924,"parent_record_offset":26720,"ancestor_record_offset":24596,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":249,"record_offset":42344,"parent_record_offset":42312,"ancestor_record_offset":24596,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":249,"record_offset":26676,"parent_record_offset":26648,"ancestor_record_offset":24596,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":249,"record_offset":26580,"parent_record_offset":26544,"ancestor_record_offset":24596,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":249,"record_offset":26544,"parent_record_offset":26508,"ancestor_record_offset":24596,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":249,"record_offset":29308,"parent_record_offset":29280,"ancestor_record_offset":24596,"inlinee":11021,"source_line":142,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":249,"record_offset":44568,"parent_record_offset":44532,"ancestor_record_offset":24596,"inlinee":10929,"source_line":338,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":249,"record_offset":40724,"parent_record_offset":40692,"ancestor_record_offset":24596,"inlinee":17829,"source_line":528,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$9
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":249,"record_offset":41600,"parent_record_offset":41576,"ancestor_record_offset":24596,"inlinee":17830,"source_line":630,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":249,"record_offset":41628,"parent_record_offset":41600,"ancestor_record_offset":24596,"inlinee":14739,"source_line":803,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":249,"record_offset":41660,"parent_record_offset":41628,"ancestor_record_offset":24596,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":249,"record_offset":38336,"parent_record_offset":38252,"ancestor_record_offset":24596,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":249,"record_offset":38308,"parent_record_offset":38252,"ancestor_record_offset":24596,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":249,"record_offset":36428,"parent_record_offset":36404,"ancestor_record_offset":24596,"inlinee":17826,"source_line":6,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":249,"record_offset":41688,"parent_record_offset":41628,"ancestor_record_offset":24596,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":249,"record_offset":248,"rva":6786160,"va":5375495280}
// 编译实例 RVAs: 0x678c70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t182422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e896c80200","size":5,"soft_fail":false,"target_va":5375678688,"va":5375496261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=22
//     evidence={"assembly":"\tcallq\t180735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8ffc10200","size":5,"soft_fail":false,"target_va":5375678704,"va":5375497964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=6
//     target_aliases=1699 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6774059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8d5a298ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375496246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140139520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5516692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e86cd2abff","size":5,"soft_fail":false,"target_va":5369992480,"va":5375509167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ff020
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3646517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8cb5bc8ff","size":5,"soft_fail":false,"target_va":5371850784,"va":5375497296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a3c90
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1928281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8a793e2ff","size":5,"soft_fail":false,"target_va":5373574288,"va":5375502564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140547c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1258189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e833cdecff","size":5,"soft_fail":false,"target_va":5374245920,"va":5375504104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140559870
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1181270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8aaf9edff","size":5,"soft_fail":false,"target_va":5374318704,"va":5375499969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t332237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8cd110500","size":5,"soft_fail":false,"target_va":5375829056,"va":5375496814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2616874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e82aee2700","size":5,"soft_fail":false,"target_va":5378120480,"va":5375503601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7774595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e883a17600","size":5,"soft_fail":false,"target_va":5383273968,"va":5375499368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7822247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8a75b7700","size":5,"soft_fail":false,"target_va":5383325024,"va":5375502772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7821788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8dc597700","size":5,"soft_fail":false,"target_va":5383325056,"va":5375503263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40851950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8ee596f02","size":5,"soft_fail":false,"target_va":5416350720,"va":5375498765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40847864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8f8496f02","size":5,"soft_fail":false,"target_va":5416353040,"va":5375505171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d70990
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t40850743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e837556f02","size":5,"soft_fail":false,"target_va":5416356240,"va":5375505492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41558223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8cf207a02","size":5,"soft_fail":false,"target_va":5417057088,"va":5375498860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41560655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e84f2a7a02","size":5,"soft_fail":false,"target_va":5417057664,"va":5375497004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t41115218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8525e7302","size":5,"soft_fail":false,"target_va":5416620769,"va":5375505546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1872206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e84e911c00","size":5,"soft_fail":false,"target_va":5377374288,"va":5375502077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40866675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e873936f02","size":5,"soft_fail":false,"target_va":5416366080,"va":5375499400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t41123669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8557f7302","size":5,"soft_fail":false,"target_va":5416620794,"va":5375497120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41115921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e811617302","size":5,"soft_fail":false,"target_va":5416621408,"va":5375505482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed::<usize, usize>
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t41115817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8a9607302","size":5,"soft_fail":false,"target_va":5416621188,"va":5375505366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41116886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8d6647302","size":5,"soft_fail":false,"target_va":5416622208,"va":5375505317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41116078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8ae617302","size":5,"soft_fail":false,"target_va":5416621762,"va":5375505679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e801360000","size":5,"soft_fail":false,"target_va":5375509280,"va":5375495450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t41115625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8e95f7302","size":5,"soft_fail":false,"target_va":5416621376,"va":5375505746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::BrickTransform::encode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2977591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8376f2d00","size":5,"soft_fail":false,"target_va":5378481680,"va":5375504084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7882511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e80f477800","size":5,"soft_fail":false,"target_va":5383383152,"va":5375500636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8467985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e811368100","size":5,"soft_fail":false,"target_va":5383963792,"va":5375495802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8476492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e84c578100","size":5,"soft_fail":false,"target_va":5383972224,"va":5375495727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8468729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8f9388100","size":5,"soft_fail":false,"target_va":5383964912,"va":5375496178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8467387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8bb338100","size":5,"soft_fail":false,"target_va":5383962816,"va":5375495424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t7904397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e88d9c7800","size":5,"soft_fail":false,"target_va":5383408256,"va":5375503854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t41113751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e897587302","size":5,"soft_fail":false,"target_va":5416619360,"va":5375505604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::perturb_row_ends
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1862637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8ed6b1c00","size":5,"soft_fail":false,"target_va":5377364624,"va":5375501982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::sine_wave::BrickRowSine::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1869250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8c2851c00","size":5,"soft_fail":false,"target_va":5377369120,"va":5375499865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::sine_wave::sine_wave_deform
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1867859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e853801c00","size":5,"soft_fail":false,"target_va":5377369552,"va":5375501688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::sine_wave::sine_wave_pack
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1867742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8de7f1c00","size":5,"soft_fail":false,"target_va":5377369520,"va":5375501773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::split_horizontally
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1862536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8886b1c00","size":5,"soft_fail":false,"target_va":5377365056,"va":5375502515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::split_into_random_rows::split_into_random_rows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1869156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e864851c00","size":5,"soft_fail":false,"target_va":5377365648,"va":5375496487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::trim_rows::trim_rows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1873402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8fa951c00","size":5,"soft_fail":false,"target_va":5377370208,"va":5375496801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::wall_constructor::WallConstructor::normalize_holes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1667910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e846731900","size":5,"soft_fail":false,"target_va":5377164752,"va":5375496837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::wall_constructor::split_brick_by_holes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1662637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8ad5e1900","size":5,"soft_fail":false,"target_va":5377164912,"va":5375502270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7799113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e849017700","size":5,"soft_fail":false,"target_va":5383302240,"va":5375503122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::expanded
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7803654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e806137700","size":5,"soft_fail":false,"target_va":5383302352,"va":5375498693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7799174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e886017700","size":5,"soft_fail":false,"target_va":5383302288,"va":5375503109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7803011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e883107700","size":5,"soft_fail":false,"target_va":5383301744,"va":5375498728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7799123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e853017700","size":5,"soft_fail":false,"target_va":5383302224,"va":5375503096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7771815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375495280,"raw_hex":"e8a7967600","size":5,"soft_fail":false,"target_va":5383276416,"va":5375504596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":249,"record_offset":4108,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":249,"record_offset":3824,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":16464,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":13888,"source_line":1920,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":16524,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5253,"source_line":2205,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":5308,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":10044,"source_line":798,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":18208,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14132,"source_line":3663,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\iter.rs
//     evidence={"module":249,"record_offset":18868,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7433,"source_line":92,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":10236,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":5252,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":10264,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":10536,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":11070,"source_line":3465,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":5640,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":15988,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":19724,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":249,"record_offset":12056,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":249,"record_offset":18096,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":249,"record_offset":19264,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":249,"record_offset":3044,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":249,"record_offset":11332,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":249,"record_offset":11660,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":249,"record_offset":6320,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":249,"record_offset":11172,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":249,"record_offset":15904,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":249,"record_offset":4388,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":249,"record_offset":6180,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":249,"record_offset":12112,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":249,"record_offset":16084,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":16316,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":5224,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":16344,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":249,"record_offset":3084,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":249,"record_offset":13468,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":2932,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":13608,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":249,"record_offset":19292,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":19012,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":17880,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":18180,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":17824,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":249,"record_offset":12636,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":249,"record_offset":13708,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":2124,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":716,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":1112,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":249,"record_offset":5584,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure$9
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_playermade_stairs/mod.rs
//     evidence={"module":249,"record_offset":19544,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":17836,"source_line":903,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$5
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":249,"record_offset":13904,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":17827,"source_line":379,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":249,"record_offset":17556,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14739,"source_line":803,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":249,"record_offset":13876,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":3912,"parent_record_offset":3888,"ancestor_record_offset":248,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":248,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":6732,"parent_record_offset":6708,"ancestor_record_offset":248,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":6756,"parent_record_offset":6732,"ancestor_record_offset":248,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":248,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":6780,"parent_record_offset":6756,"ancestor_record_offset":248,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":3888,"parent_record_offset":3856,"ancestor_record_offset":248,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":249,"record_offset":6708,"parent_record_offset":6616,"ancestor_record_offset":248,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":249,"record_offset":3856,"parent_record_offset":3824,"ancestor_record_offset":248,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_slices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":18260,"parent_record_offset":18232,"ancestor_record_offset":248,"inlinee":11741,"source_line":1563,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_range
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":18484,"parent_record_offset":18260,"ancestor_record_offset":248,"inlinee":4789,"source_line":228,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":16608,"parent_record_offset":16584,"ancestor_record_offset":248,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":16584,"parent_record_offset":16548,"ancestor_record_offset":248,"inlinee":5255,"source_line":236,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":18232,"parent_record_offset":18208,"ancestor_record_offset":248,"inlinee":11740,"source_line":1499,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":16868,"parent_record_offset":16696,"ancestor_record_offset":248,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::push_back_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":16548,"parent_record_offset":16524,"ancestor_record_offset":248,"inlinee":5254,"source_line":2223,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::slice_ranges
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":18288,"parent_record_offset":18260,"ancestor_record_offset":248,"inlinee":4783,"source_line":1666,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":16696,"parent_record_offset":16548,"ancestor_record_offset":248,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::truncate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":16492,"parent_record_offset":16464,"ancestor_record_offset":248,"inlinee":13889,"source_line":1364,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":5336,"parent_record_offset":5308,"ancestor_record_offset":248,"inlinee":10045,"source_line":853,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":249,"record_offset":16732,"parent_record_offset":16696,"ancestor_record_offset":248,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":16632,"parent_record_offset":16608,"ancestor_record_offset":248,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":10356,"parent_record_offset":10332,"ancestor_record_offset":248,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":10684,"parent_record_offset":10656,"ancestor_record_offset":248,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":4576,"parent_record_offset":4548,"ancestor_record_offset":248,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":10768,"parent_record_offset":10744,"ancestor_record_offset":248,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":6644,"parent_record_offset":6616,"ancestor_record_offset":248,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":6616,"parent_record_offset":6592,"ancestor_record_offset":248,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":10744,"parent_record_offset":10712,"ancestor_record_offset":248,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":10404,"parent_record_offset":10380,"ancestor_record_offset":248,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":10380,"parent_record_offset":10356,"ancestor_record_offset":248,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":10712,"parent_record_offset":10684,"ancestor_record_offset":248,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":4636,"parent_record_offset":4604,"ancestor_record_offset":248,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":4604,"parent_record_offset":4576,"ancestor_record_offset":248,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":6592,"parent_record_offset":6568,"ancestor_record_offset":248,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":249,"record_offset":5804,"parent_record_offset":5760,"ancestor_record_offset":248,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":249,"record_offset":11600,"parent_record_offset":11576,"ancestor_record_offset":248,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":10332,"parent_record_offset":10292,"ancestor_record_offset":248,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":248,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":10596,"parent_record_offset":10536,"ancestor_record_offset":248,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":11624,"parent_record_offset":11600,"ancestor_record_offset":248,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":10568,"parent_record_offset":10536,"ancestor_record_offset":248,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":10292,"parent_record_offset":10264,"ancestor_record_offset":248,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":10656,"parent_record_offset":10596,"ancestor_record_offset":248,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":4520,"parent_record_offset":4484,"ancestor_record_offset":248,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":4548,"parent_record_offset":4520,"ancestor_record_offset":248,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":4420,"parent_record_offset":4388,"ancestor_record_offset":248,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":249,"record_offset":5084,"parent_record_offset":5056,"ancestor_record_offset":248,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":249,"record_offset":5148,"parent_record_offset":5084,"ancestor_record_offset":248,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":249,"record_offset":20912,"parent_record_offset":19796,"ancestor_record_offset":248,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":249,"record_offset":20740,"parent_record_offset":20712,"ancestor_record_offset":248,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":249,"record_offset":4796,"parent_record_offset":4484,"ancestor_record_offset":248,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":249,"record_offset":4452,"parent_record_offset":4420,"ancestor_record_offset":248,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":249,"record_offset":4484,"parent_record_offset":4452,"ancestor_record_offset":248,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":249,"record_offset":5832,"parent_record_offset":5804,"ancestor_record_offset":248,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":249,"record_offset":2452,"parent_record_offset":2424,"ancestor_record_offset":248,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":249,"record_offset":2480,"parent_record_offset":2452,"ancestor_record_offset":248,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":249,"record_offset":2400,"parent_record_offset":2372,"ancestor_record_offset":248,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":249,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":248,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":249,"record_offset":2540,"parent_record_offset":2424,"ancestor_record_offset":248,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":249,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":248,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":249,"record_offset":6276,"parent_record_offset":6252,"ancestor_record_offset":248,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":249,"record_offset":12160,"parent_record_offset":12136,"ancestor_record_offset":248,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":249,"record_offset":12184,"parent_record_offset":12160,"ancestor_record_offset":248,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":249,"record_offset":11040,"parent_record_offset":11008,"ancestor_record_offset":248,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":249,"record_offset":6252,"parent_record_offset":6228,"ancestor_record_offset":248,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":249,"record_offset":19340,"parent_record_offset":19316,"ancestor_record_offset":248,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":249,"record_offset":19372,"parent_record_offset":19292,"ancestor_record_offset":248,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":249,"record_offset":19316,"parent_record_offset":19292,"ancestor_record_offset":248,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":249,"record_offset":5608,"parent_record_offset":5584,"ancestor_record_offset":248,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":249,"record_offset":18020,"parent_record_offset":17996,"ancestor_record_offset":248,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":249,"record_offset":5924,"parent_record_offset":5860,"ancestor_record_offset":248,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":249,"record_offset":12908,"parent_record_offset":12884,"ancestor_record_offset":248,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":249,"record_offset":12956,"parent_record_offset":12932,"ancestor_record_offset":248,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":249,"record_offset":12688,"parent_record_offset":12636,"ancestor_record_offset":248,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":249,"record_offset":14344,"parent_record_offset":14304,"ancestor_record_offset":248,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":249,"record_offset":6356,"parent_record_offset":6320,"ancestor_record_offset":248,"inlinee":17819,"source_line":180,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":249,"record_offset":11732,"parent_record_offset":11688,"ancestor_record_offset":248,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":249,"record_offset":11688,"parent_record_offset":11660,"ancestor_record_offset":248,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":249,"record_offset":11832,"parent_record_offset":11804,"ancestor_record_offset":248,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":249,"record_offset":11804,"parent_record_offset":11688,"ancestor_record_offset":248,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":249,"record_offset":4868,"parent_record_offset":4844,"ancestor_record_offset":248,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":249,"record_offset":11860,"parent_record_offset":11832,"ancestor_record_offset":248,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":249,"record_offset":4932,"parent_record_offset":4892,"ancestor_record_offset":248,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":249,"record_offset":11228,"parent_record_offset":11200,"ancestor_record_offset":248,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":249,"record_offset":6204,"parent_record_offset":6180,"ancestor_record_offset":248,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":249,"record_offset":6392,"parent_record_offset":6356,"ancestor_record_offset":248,"inlinee":17821,"source_line":124,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":249,"record_offset":11200,"parent_record_offset":11172,"ancestor_record_offset":248,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":249,"record_offset":6228,"parent_record_offset":6204,"ancestor_record_offset":248,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":249,"record_offset":6428,"parent_record_offset":6392,"ancestor_record_offset":248,"inlinee":17822,"source_line":355,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":249,"record_offset":11008,"parent_record_offset":10976,"ancestor_record_offset":248,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":249,"record_offset":10976,"parent_record_offset":10596,"ancestor_record_offset":248,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":249,"record_offset":6144,"parent_record_offset":6120,"ancestor_record_offset":248,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":249,"record_offset":4844,"parent_record_offset":4820,"ancestor_record_offset":248,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":249,"record_offset":5056,"parent_record_offset":4932,"ancestor_record_offset":248,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":249,"record_offset":12980,"parent_record_offset":12956,"ancestor_record_offset":248,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":249,"record_offset":2504,"parent_record_offset":2480,"ancestor_record_offset":248,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":249,"record_offset":5860,"parent_record_offset":5832,"ancestor_record_offset":248,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":249,"record_offset":5892,"parent_record_offset":5860,"ancestor_record_offset":248,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":249,"record_offset":6676,"parent_record_offset":6644,"ancestor_record_offset":248,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":249,"record_offset":5016,"parent_record_offset":4988,"ancestor_record_offset":248,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":249,"record_offset":13768,"parent_record_offset":13708,"ancestor_record_offset":248,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":249,"record_offset":4988,"parent_record_offset":4960,"ancestor_record_offset":248,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":249,"record_offset":13736,"parent_record_offset":13708,"ancestor_record_offset":248,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":249,"record_offset":2272,"parent_record_offset":2232,"ancestor_record_offset":248,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":249,"record_offset":11912,"parent_record_offset":11888,"ancestor_record_offset":248,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":249,"record_offset":12136,"parent_record_offset":12112,"ancestor_record_offset":248,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":11764,"parent_record_offset":11732,"ancestor_record_offset":248,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":12848,"parent_record_offset":12688,"ancestor_record_offset":248,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":11888,"parent_record_offset":11860,"ancestor_record_offset":248,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":12932,"parent_record_offset":12908,"ancestor_record_offset":248,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":249,"record_offset":14756,"parent_record_offset":14304,"ancestor_record_offset":248,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":249,"record_offset":1060,"parent_record_offset":1032,"ancestor_record_offset":248,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":249,"record_offset":2888,"parent_record_offset":2232,"ancestor_record_offset":248,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":249,"record_offset":2604,"parent_record_offset":2372,"ancestor_record_offset":248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":249,"record_offset":6496,"parent_record_offset":6472,"ancestor_record_offset":248,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":249,"record_offset":2980,"parent_record_offset":2932,"ancestor_record_offset":248,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":249,"record_offset":1532,"parent_record_offset":1508,"ancestor_record_offset":248,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":249,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":248,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":249,"record_offset":1716,"parent_record_offset":1688,"ancestor_record_offset":248,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":249,"record_offset":5116,"parent_record_offset":5084,"ancestor_record_offset":248,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":249,"record_offset":1816,"parent_record_offset":1788,"ancestor_record_offset":248,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":249,"record_offset":1788,"parent_record_offset":1760,"ancestor_record_offset":248,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":249,"record_offset":18636,"parent_record_offset":18232,"ancestor_record_offset":248,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":249,"record_offset":16016,"parent_record_offset":15988,"ancestor_record_offset":248,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":249,"record_offset":16044,"parent_record_offset":16016,"ancestor_record_offset":248,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":249,"record_offset":13492,"parent_record_offset":13468,"ancestor_record_offset":248,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":248,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":248,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":6472,"parent_record_offset":6428,"ancestor_record_offset":248,"inlinee":6630,"source_line":418,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":1844,"parent_record_offset":1816,"ancestor_record_offset":248,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":4892,"parent_record_offset":4868,"ancestor_record_offset":248,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":11944,"parent_record_offset":11860,"ancestor_record_offset":248,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":1436,"parent_record_offset":1320,"ancestor_record_offset":248,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$180::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":249,"record_offset":11256,"parent_record_offset":11228,"ancestor_record_offset":248,"inlinee":17824,"source_line":438,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":249,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":248,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":249,"record_offset":1148,"parent_record_offset":1112,"ancestor_record_offset":248,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":249,"record_offset":1288,"parent_record_offset":1256,"ancestor_record_offset":248,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":249,"record_offset":1256,"parent_record_offset":1220,"ancestor_record_offset":248,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":249,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":248,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":249,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":248,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":249,"record_offset":1172,"parent_record_offset":1148,"ancestor_record_offset":248,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":249,"record_offset":1760,"parent_record_offset":1220,"ancestor_record_offset":248,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":249,"record_offset":824,"parent_record_offset":792,"ancestor_record_offset":248,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":249,"record_offset":792,"parent_record_offset":764,"ancestor_record_offset":248,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":249,"record_offset":952,"parent_record_offset":920,"ancestor_record_offset":248,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":249,"record_offset":1008,"parent_record_offset":952,"ancestor_record_offset":248,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":249,"record_offset":920,"parent_record_offset":888,"ancestor_record_offset":248,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":249,"record_offset":764,"parent_record_offset":716,"ancestor_record_offset":248,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":249,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":248,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":249,"record_offset":1320,"parent_record_offset":1288,"ancestor_record_offset":248,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":249,"record_offset":1604,"parent_record_offset":1320,"ancestor_record_offset":248,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":249,"record_offset":1576,"parent_record_offset":1320,"ancestor_record_offset":248,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":249,"record_offset":1380,"parent_record_offset":1320,"ancestor_record_offset":248,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":249,"record_offset":18144,"parent_record_offset":18120,"ancestor_record_offset":248,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":249,"record_offset":18120,"parent_record_offset":18096,"ancestor_record_offset":248,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":17964,"parent_record_offset":17932,"ancestor_record_offset":248,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":17932,"parent_record_offset":17904,"ancestor_record_offset":248,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":17904,"parent_record_offset":17880,"ancestor_record_offset":248,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":17368,"parent_record_offset":17340,"ancestor_record_offset":248,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":17396,"parent_record_offset":17368,"ancestor_record_offset":248,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":17480,"parent_record_offset":17368,"ancestor_record_offset":248,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":249,"record_offset":12884,"parent_record_offset":12636,"ancestor_record_offset":248,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":249,"record_offset":14016,"parent_record_offset":13904,"ancestor_record_offset":248,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":2160,"parent_record_offset":2124,"ancestor_record_offset":248,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":3148,"parent_record_offset":3116,"ancestor_record_offset":248,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":3600,"parent_record_offset":3396,"ancestor_record_offset":248,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":249,"record_offset":2300,"parent_record_offset":2272,"ancestor_record_offset":248,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":2372,"parent_record_offset":2232,"ancestor_record_offset":248,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":249,"record_offset":2576,"parent_record_offset":2372,"ancestor_record_offset":248,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":249,"record_offset":17996,"parent_record_offset":17964,"ancestor_record_offset":248,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":249,"record_offset":2328,"parent_record_offset":2300,"ancestor_record_offset":248,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":249,"record_offset":2232,"parent_record_offset":2196,"ancestor_record_offset":248,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":249,"record_offset":2196,"parent_record_offset":2160,"ancestor_record_offset":248,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":249,"record_offset":4960,"parent_record_offset":4932,"ancestor_record_offset":248,"inlinee":11021,"source_line":142,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":249,"record_offset":20220,"parent_record_offset":20184,"ancestor_record_offset":248,"inlinee":10929,"source_line":338,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":249,"record_offset":16376,"parent_record_offset":16344,"ancestor_record_offset":248,"inlinee":17829,"source_line":528,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$9
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":249,"record_offset":17252,"parent_record_offset":17228,"ancestor_record_offset":248,"inlinee":17830,"source_line":630,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":249,"record_offset":17280,"parent_record_offset":17252,"ancestor_record_offset":248,"inlinee":14739,"source_line":803,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":249,"record_offset":17312,"parent_record_offset":17280,"ancestor_record_offset":248,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":249,"record_offset":13988,"parent_record_offset":13904,"ancestor_record_offset":248,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":249,"record_offset":13960,"parent_record_offset":13904,"ancestor_record_offset":248,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":249,"record_offset":12080,"parent_record_offset":12056,"ancestor_record_offset":248,"inlinee":17826,"source_line":6,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":249,"record_offset":17340,"parent_record_offset":17280,"ancestor_record_offset":248,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1493,"record_offset":280,"rva":21267968,"va":5389977088}
// 编译实例 RVAs: 0x1448600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-14299386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e806cf25ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5389978069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=22
//     evidence={"assembly":"\tcallq\t-14301140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e82cc825ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5389979839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=6
//     target_aliases=1699 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21255867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e845a9bbfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5389978054,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ff020
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18128393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8f761ebfe","size":5,"soft_fail":false,"target_va":5371850784,"va":5389979172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14149569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e83f1828ff","size":5,"soft_fail":false,"target_va":5375829056,"va":5389978620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11866667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8d5ed4aff","size":5,"soft_fail":false,"target_va":5378120480,"va":5389987142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6707339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e875a799ff","size":5,"soft_fail":false,"target_va":5383273968,"va":5389981302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6661316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e83c5b9aff","size":5,"soft_fail":false,"target_va":5383325024,"va":5389986335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6661761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e87f599aff","size":5,"soft_fail":false,"target_va":5383325056,"va":5389986812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141403be0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-291744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8608cfbff","size":5,"soft_fail":false,"target_va":5389695968,"va":5389987707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405de0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-281460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e88cb4fbff","size":5,"soft_fail":false,"target_va":5389704672,"va":5389986127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14142afd0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-126448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e81012feff","size":5,"soft_fail":false,"target_va":5389856720,"va":5389983163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146cea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t138565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8451d0200","size":5,"soft_fail":false,"target_va":5390126752,"va":5389988182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141478800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t195516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8bcfb0200","size":5,"soft_fail":false,"target_va":5390174208,"va":5389978687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26370070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e816609201","size":5,"soft_fail":false,"target_va":5416350720,"va":5389980645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26370514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8d2619201","size":5,"soft_fail":false,"target_va":5416353040,"va":5389982521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27076331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8eb269d01","size":5,"soft_fail":false,"target_va":5417057088,"va":5389980752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27078780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e87c309d01","size":5,"soft_fail":false,"target_va":5417057664,"va":5389978879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27075145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e849229d01","size":5,"soft_fail":false,"target_va":5417058032,"va":5389982882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bf80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27075262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8be229d01","size":5,"soft_fail":false,"target_va":5417058176,"va":5389982909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26632533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e855619601","size":5,"soft_fail":false,"target_va":5416620769,"va":5389988231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12611347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8ed903fff","size":5,"soft_fail":false,"target_va":5377374288,"va":5389985630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26384740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e864999201","size":5,"soft_fail":false,"target_va":5416366080,"va":5389981335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26641794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e882859601","size":5,"soft_fail":false,"target_va":5416620794,"va":5389978995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26633270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e836649601","size":5,"soft_fail":false,"target_va":5416621408,"va":5389988133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed::<usize, usize>
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26633166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8ce639601","size":5,"soft_fail":false,"target_va":5416621188,"va":5389988017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26634235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8fb679601","size":5,"soft_fail":false,"target_va":5416622208,"va":5389987968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26633393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8b1649601","size":5,"soft_fail":false,"target_va":5416621762,"va":5389988364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t158043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e85b690200","size":5,"soft_fail":false,"target_va":5390146576,"va":5389988528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t57437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e85de00000","size":5,"soft_fail":false,"target_va":5390034704,"va":5389977262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26632940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8ec629601","size":5,"soft_fail":false,"target_va":5416621376,"va":5389988431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::BrickTransform::encode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11506012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8a46e50ff","size":5,"soft_fail":false,"target_va":5378481680,"va":5389987687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::FinalizedWallHoles::wall_length
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8366477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8735680ff","size":5,"soft_fail":false,"target_va":5381612160,"va":5389978632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6600721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8ef479bff","size":5,"soft_fail":false,"target_va":5383383152,"va":5389983868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6013823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8813ca4ff","size":5,"soft_fail":false,"target_va":5383963792,"va":5389977610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6005316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8bc5da4ff","size":5,"soft_fail":false,"target_va":5383972224,"va":5389977535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6013079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8693fa4ff","size":5,"soft_fail":false,"target_va":5383964912,"va":5389977986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6014425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8273aa4ff","size":5,"soft_fail":false,"target_va":5383962816,"va":5389977236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-6579133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8439c9bff","size":5,"soft_fail":false,"target_va":5383408256,"va":5389987384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26708860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e87c8b9701","size":5,"soft_fail":false,"target_va":5416697008,"va":5389988143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-220734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8c2a1fcff","size":5,"soft_fail":false,"target_va":5389771136,"va":5389991865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26631066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e89a5b9601","size":5,"soft_fail":false,"target_va":5416619360,"va":5389988289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::perturb_row_ends
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12620916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e88c6b3fff","size":5,"soft_fail":false,"target_va":5377364624,"va":5389985535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::split_horizontally
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12621027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e81d6b3fff","size":5,"soft_fail":false,"target_va":5377365056,"va":5389986078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::split_into_random_rows::split_into_random_rows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12612666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8c68b3fff","size":5,"soft_fail":false,"target_va":5377365648,"va":5389978309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::trim_rows::trim_rows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12608404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e86c9c3fff","size":5,"soft_fail":false,"target_va":5377370208,"va":5389978607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::wall_constructor::split_brick_by_holes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12820913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e84f5e3cff","size":5,"soft_fail":false,"target_va":5377164912,"va":5389985820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6684431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8f1009aff","size":5,"soft_fail":false,"target_va":5383302240,"va":5389986666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::expanded
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6678230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e82a199aff","size":5,"soft_fail":false,"target_va":5383302352,"va":5389980577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6684370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e82e019aff","size":5,"soft_fail":false,"target_va":5383302288,"va":5389986653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6678873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8a7169aff","size":5,"soft_fail":false,"target_va":5383301744,"va":5389980612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6684421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e8fb009aff","size":5,"soft_fail":false,"target_va":5383302224,"va":5389986640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6705581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389977088,"raw_hex":"e853ae99ff","size":5,"soft_fail":false,"target_va":5383276416,"va":5389981992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1493,"record_offset":4156,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1493,"record_offset":3872,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":19388,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":13888,"source_line":1920,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":19448,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5253,"source_line":2205,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":5620,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":10044,"source_line":798,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":21132,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":14132,"source_line":3663,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\iter.rs
//     evidence={"module":1493,"record_offset":21788,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7433,"source_line":92,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":10548,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":5564,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":10576,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":10848,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11070,"source_line":3465,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":5952,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":18608,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":13992,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":12396,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1493,"record_offset":21020,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":22184,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":3092,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":11672,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1493,"record_offset":12000,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":1493,"record_offset":6632,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":11484,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1493,"record_offset":18524,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":4436,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":6492,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1493,"record_offset":12452,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1493,"record_offset":18704,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":18992,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":5272,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":19020,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1493,"record_offset":3132,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1493,"record_offset":15476,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":2980,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":15616,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1493,"record_offset":22212,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":21932,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":20804,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":21104,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":20748,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1493,"record_offset":12976,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1493,"record_offset":15716,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":2172,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":772,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":1160,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":5896,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_playermade_stairs/pillars_visual.rs
//     evidence={"module":1493,"record_offset":22464,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79753,"source_line":228,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::BrickRowSine::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":15884,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26643,"source_line":12,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::sine_wave_deform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":19224,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26645,"source_line":45,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::sine_wave_pack
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":19360,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26644,"source_line":73,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::WallConstructor::normalize_holes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":5300,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26334,"source_line":56,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$5
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":16524,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":17827,"source_line":379,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1493,"record_offset":20480,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":14739,"source_line":803,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1493,"record_offset":16496,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":280,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":280,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":7044,"parent_record_offset":7020,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":7068,"parent_record_offset":7044,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":280,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":7092,"parent_record_offset":7068,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":3936,"parent_record_offset":3904,"ancestor_record_offset":280,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":7020,"parent_record_offset":6928,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1493,"record_offset":3904,"parent_record_offset":3872,"ancestor_record_offset":280,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_slices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":21184,"parent_record_offset":21156,"ancestor_record_offset":280,"inlinee":11741,"source_line":1563,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_range
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":21404,"parent_record_offset":21184,"ancestor_record_offset":280,"inlinee":4789,"source_line":228,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":19532,"parent_record_offset":19508,"ancestor_record_offset":280,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":19508,"parent_record_offset":19472,"ancestor_record_offset":280,"inlinee":5255,"source_line":236,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":21156,"parent_record_offset":21132,"ancestor_record_offset":280,"inlinee":11740,"source_line":1499,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":19792,"parent_record_offset":19620,"ancestor_record_offset":280,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::push_back_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":19472,"parent_record_offset":19448,"ancestor_record_offset":280,"inlinee":5254,"source_line":2223,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::slice_ranges
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":21212,"parent_record_offset":21184,"ancestor_record_offset":280,"inlinee":4783,"source_line":1666,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":19620,"parent_record_offset":19472,"ancestor_record_offset":280,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::truncate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":19416,"parent_record_offset":19388,"ancestor_record_offset":280,"inlinee":13889,"source_line":1364,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":5648,"parent_record_offset":5620,"ancestor_record_offset":280,"inlinee":10045,"source_line":853,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":19656,"parent_record_offset":19620,"ancestor_record_offset":280,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":19556,"parent_record_offset":19532,"ancestor_record_offset":280,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":10668,"parent_record_offset":10644,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":10996,"parent_record_offset":10968,"ancestor_record_offset":280,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":4624,"parent_record_offset":4596,"ancestor_record_offset":280,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":11080,"parent_record_offset":11056,"ancestor_record_offset":280,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":6956,"parent_record_offset":6928,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":11056,"parent_record_offset":11024,"ancestor_record_offset":280,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":10692,"parent_record_offset":10668,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":11024,"parent_record_offset":10996,"ancestor_record_offset":280,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":4684,"parent_record_offset":4652,"ancestor_record_offset":280,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":4652,"parent_record_offset":4624,"ancestor_record_offset":280,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":6904,"parent_record_offset":6880,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":6116,"parent_record_offset":6072,"ancestor_record_offset":280,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1493,"record_offset":11940,"parent_record_offset":11916,"ancestor_record_offset":280,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1493,"record_offset":5380,"parent_record_offset":5300,"ancestor_record_offset":280,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1493,"record_offset":5416,"parent_record_offset":5380,"ancestor_record_offset":280,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":10644,"parent_record_offset":10604,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":5352,"parent_record_offset":5300,"ancestor_record_offset":280,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":4868,"parent_record_offset":4844,"ancestor_record_offset":280,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":10908,"parent_record_offset":10848,"ancestor_record_offset":280,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":11964,"parent_record_offset":11940,"ancestor_record_offset":280,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":10880,"parent_record_offset":10848,"ancestor_record_offset":280,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":10604,"parent_record_offset":10576,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":10968,"parent_record_offset":10908,"ancestor_record_offset":280,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":4568,"parent_record_offset":4532,"ancestor_record_offset":280,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":4596,"parent_record_offset":4568,"ancestor_record_offset":280,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":4468,"parent_record_offset":4436,"ancestor_record_offset":280,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":5132,"parent_record_offset":5104,"ancestor_record_offset":280,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1493,"record_offset":5196,"parent_record_offset":5132,"ancestor_record_offset":280,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1493,"record_offset":15144,"parent_record_offset":14056,"ancestor_record_offset":280,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1493,"record_offset":14976,"parent_record_offset":14952,"ancestor_record_offset":280,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1493,"record_offset":4844,"parent_record_offset":4532,"ancestor_record_offset":280,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1493,"record_offset":4500,"parent_record_offset":4468,"ancestor_record_offset":280,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1493,"record_offset":4532,"parent_record_offset":4500,"ancestor_record_offset":280,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1493,"record_offset":6144,"parent_record_offset":6116,"ancestor_record_offset":280,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":2500,"parent_record_offset":2472,"ancestor_record_offset":280,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":2528,"parent_record_offset":2500,"ancestor_record_offset":280,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":2448,"parent_record_offset":2420,"ancestor_record_offset":280,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":2472,"parent_record_offset":2448,"ancestor_record_offset":280,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":2588,"parent_record_offset":2472,"ancestor_record_offset":280,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":2700,"parent_record_offset":2676,"ancestor_record_offset":280,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":6588,"parent_record_offset":6564,"ancestor_record_offset":280,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":12500,"parent_record_offset":12476,"ancestor_record_offset":280,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":12524,"parent_record_offset":12500,"ancestor_record_offset":280,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":11352,"parent_record_offset":11320,"ancestor_record_offset":280,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":6564,"parent_record_offset":6540,"ancestor_record_offset":280,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":1493,"record_offset":22260,"parent_record_offset":22236,"ancestor_record_offset":280,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1493,"record_offset":22292,"parent_record_offset":22212,"ancestor_record_offset":280,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1493,"record_offset":22236,"parent_record_offset":22212,"ancestor_record_offset":280,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":5920,"parent_record_offset":5896,"ancestor_record_offset":280,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::fract
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":16008,"parent_record_offset":15984,"ancestor_record_offset":280,"inlinee":26641,"source_line":1873,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":20944,"parent_record_offset":20920,"ancestor_record_offset":280,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::trunc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":16032,"parent_record_offset":16008,"ancestor_record_offset":280,"inlinee":21635,"source_line":1844,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1493,"record_offset":5492,"parent_record_offset":5452,"ancestor_record_offset":280,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1493,"record_offset":6236,"parent_record_offset":6172,"ancestor_record_offset":280,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1493,"record_offset":13236,"parent_record_offset":13212,"ancestor_record_offset":280,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1493,"record_offset":13284,"parent_record_offset":13260,"ancestor_record_offset":280,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1493,"record_offset":13024,"parent_record_offset":12976,"ancestor_record_offset":280,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1493,"record_offset":16964,"parent_record_offset":16924,"ancestor_record_offset":280,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1493,"record_offset":6668,"parent_record_offset":6632,"ancestor_record_offset":280,"inlinee":17819,"source_line":180,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1493,"record_offset":12072,"parent_record_offset":12028,"ancestor_record_offset":280,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1493,"record_offset":12028,"parent_record_offset":12000,"ancestor_record_offset":280,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1493,"record_offset":12172,"parent_record_offset":12144,"ancestor_record_offset":280,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1493,"record_offset":12144,"parent_record_offset":12028,"ancestor_record_offset":280,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1493,"record_offset":4916,"parent_record_offset":4892,"ancestor_record_offset":280,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1493,"record_offset":12200,"parent_record_offset":12172,"ancestor_record_offset":280,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1493,"record_offset":4980,"parent_record_offset":4940,"ancestor_record_offset":280,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":1493,"record_offset":11548,"parent_record_offset":11516,"ancestor_record_offset":280,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":6516,"parent_record_offset":6492,"ancestor_record_offset":280,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":6704,"parent_record_offset":6668,"ancestor_record_offset":280,"inlinee":17821,"source_line":124,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":11516,"parent_record_offset":11484,"ancestor_record_offset":280,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":6540,"parent_record_offset":6516,"ancestor_record_offset":280,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":6740,"parent_record_offset":6704,"ancestor_record_offset":280,"inlinee":17822,"source_line":355,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1493,"record_offset":11320,"parent_record_offset":11288,"ancestor_record_offset":280,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1493,"record_offset":11288,"parent_record_offset":10908,"ancestor_record_offset":280,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1493,"record_offset":6456,"parent_record_offset":6432,"ancestor_record_offset":280,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":4892,"parent_record_offset":4868,"ancestor_record_offset":280,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":5104,"parent_record_offset":4980,"ancestor_record_offset":280,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":13308,"parent_record_offset":13284,"ancestor_record_offset":280,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1493,"record_offset":2552,"parent_record_offset":2528,"ancestor_record_offset":280,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1493,"record_offset":6172,"parent_record_offset":6144,"ancestor_record_offset":280,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1493,"record_offset":6204,"parent_record_offset":6172,"ancestor_record_offset":280,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1493,"record_offset":6988,"parent_record_offset":6956,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1493,"record_offset":5064,"parent_record_offset":5036,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1493,"record_offset":14584,"parent_record_offset":14508,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1493,"record_offset":5036,"parent_record_offset":5008,"ancestor_record_offset":280,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1493,"record_offset":14544,"parent_record_offset":14508,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1493,"record_offset":2320,"parent_record_offset":2280,"ancestor_record_offset":280,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1493,"record_offset":12252,"parent_record_offset":12228,"ancestor_record_offset":280,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1493,"record_offset":12476,"parent_record_offset":12452,"ancestor_record_offset":280,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":12104,"parent_record_offset":12072,"ancestor_record_offset":280,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":13176,"parent_record_offset":13024,"ancestor_record_offset":280,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":12228,"parent_record_offset":12200,"ancestor_record_offset":280,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":13260,"parent_record_offset":13236,"ancestor_record_offset":280,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":17376,"parent_record_offset":16924,"ancestor_record_offset":280,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1493,"record_offset":1108,"parent_record_offset":1080,"ancestor_record_offset":280,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1493,"record_offset":2936,"parent_record_offset":2280,"ancestor_record_offset":280,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1493,"record_offset":2652,"parent_record_offset":2420,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1493,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":280,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":3028,"parent_record_offset":2980,"ancestor_record_offset":280,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":280,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":280,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":1764,"parent_record_offset":1736,"ancestor_record_offset":280,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1493,"record_offset":5164,"parent_record_offset":5132,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1493,"record_offset":1864,"parent_record_offset":1836,"ancestor_record_offset":280,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1493,"record_offset":1836,"parent_record_offset":1808,"ancestor_record_offset":280,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1493,"record_offset":21556,"parent_record_offset":21156,"ancestor_record_offset":280,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1493,"record_offset":18636,"parent_record_offset":18608,"ancestor_record_offset":280,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1493,"record_offset":18664,"parent_record_offset":18636,"ancestor_record_offset":280,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1493,"record_offset":15500,"parent_record_offset":15476,"ancestor_record_offset":280,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":280,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":1532,"parent_record_offset":1508,"ancestor_record_offset":280,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":6784,"parent_record_offset":6740,"ancestor_record_offset":280,"inlinee":6630,"source_line":418,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":1892,"parent_record_offset":1864,"ancestor_record_offset":280,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":4940,"parent_record_offset":4916,"ancestor_record_offset":280,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":12284,"parent_record_offset":12200,"ancestor_record_offset":280,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":1484,"parent_record_offset":1368,"ancestor_record_offset":280,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$180::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":11580,"parent_record_offset":11548,"ancestor_record_offset":280,"inlinee":17824,"source_line":438,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1493,"record_offset":5452,"parent_record_offset":5416,"ancestor_record_offset":280,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1493,"record_offset":900,"parent_record_offset":876,"ancestor_record_offset":280,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1493,"record_offset":1196,"parent_record_offset":1160,"ancestor_record_offset":280,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1493,"record_offset":1336,"parent_record_offset":1304,"ancestor_record_offset":280,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1493,"record_offset":1304,"parent_record_offset":1268,"ancestor_record_offset":280,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":280,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":280,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":1220,"parent_record_offset":1196,"ancestor_record_offset":280,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":1808,"parent_record_offset":1268,"ancestor_record_offset":280,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":876,"parent_record_offset":844,"ancestor_record_offset":280,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":844,"parent_record_offset":816,"ancestor_record_offset":280,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":1000,"parent_record_offset":968,"ancestor_record_offset":280,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":1056,"parent_record_offset":1000,"ancestor_record_offset":280,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":968,"parent_record_offset":940,"ancestor_record_offset":280,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":816,"parent_record_offset":772,"ancestor_record_offset":280,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":280,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":1368,"parent_record_offset":1336,"ancestor_record_offset":280,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":1652,"parent_record_offset":1368,"ancestor_record_offset":280,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":1624,"parent_record_offset":1368,"ancestor_record_offset":280,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":1428,"parent_record_offset":1368,"ancestor_record_offset":280,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1493,"record_offset":21068,"parent_record_offset":21044,"ancestor_record_offset":280,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1493,"record_offset":21044,"parent_record_offset":21020,"ancestor_record_offset":280,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":20888,"parent_record_offset":20856,"ancestor_record_offset":280,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":20856,"parent_record_offset":20828,"ancestor_record_offset":280,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":20828,"parent_record_offset":20804,"ancestor_record_offset":280,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":20292,"parent_record_offset":20264,"ancestor_record_offset":280,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":20320,"parent_record_offset":20292,"ancestor_record_offset":280,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":20404,"parent_record_offset":20292,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1493,"record_offset":13212,"parent_record_offset":12976,"ancestor_record_offset":280,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1493,"record_offset":14508,"parent_record_offset":14472,"ancestor_record_offset":280,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":2208,"parent_record_offset":2172,"ancestor_record_offset":280,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":3196,"parent_record_offset":3164,"ancestor_record_offset":280,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":3648,"parent_record_offset":3444,"ancestor_record_offset":280,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1493,"record_offset":2348,"parent_record_offset":2320,"ancestor_record_offset":280,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":2420,"parent_record_offset":2280,"ancestor_record_offset":280,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":2624,"parent_record_offset":2420,"ancestor_record_offset":280,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody_core::shader::math::Affine3Packed::add_y
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/render-std/rhapsody-core/src/lib.rs
//     evidence={"module":1493,"record_offset":22504,"parent_record_offset":22464,"ancestor_record_offset":280,"inlinee":79754,"source_line":60,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::fract
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":15984,"parent_record_offset":15932,"ancestor_record_offset":280,"inlinee":26640,"source_line":178,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":15956,"parent_record_offset":15932,"ancestor_record_offset":280,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":20920,"parent_record_offset":20888,"ancestor_record_offset":280,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1493,"record_offset":2376,"parent_record_offset":2348,"ancestor_record_offset":280,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1493,"record_offset":2280,"parent_record_offset":2244,"ancestor_record_offset":280,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1493,"record_offset":2244,"parent_record_offset":2208,"ancestor_record_offset":280,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::sine_wave::random_f
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":15932,"parent_record_offset":15884,"ancestor_record_offset":280,"inlinee":26639,"source_line":82,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::sine_wave::sine_wave_deform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":19076,"parent_record_offset":19048,"ancestor_record_offset":280,"inlinee":26645,"source_line":45,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":5008,"parent_record_offset":4980,"ancestor_record_offset":280,"inlinee":11021,"source_line":142,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":14472,"parent_record_offset":14436,"ancestor_record_offset":280,"inlinee":10929,"source_line":338,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":19048,"parent_record_offset":19020,"ancestor_record_offset":280,"inlinee":17829,"source_line":528,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$9
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":20176,"parent_record_offset":20152,"ancestor_record_offset":280,"inlinee":17830,"source_line":630,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1493,"record_offset":20204,"parent_record_offset":20176,"ancestor_record_offset":280,"inlinee":14739,"source_line":803,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1493,"record_offset":20236,"parent_record_offset":20204,"ancestor_record_offset":280,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1493,"record_offset":16608,"parent_record_offset":16524,"ancestor_record_offset":280,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1493,"record_offset":16580,"parent_record_offset":16524,"ancestor_record_offset":280,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1493,"record_offset":12420,"parent_record_offset":12396,"ancestor_record_offset":280,"inlinee":17826,"source_line":6,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1493,"record_offset":20264,"parent_record_offset":20204,"ancestor_record_offset":280,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1493,"record_offset":50760,"rva":21296784,"va":5390005904}
// 编译实例 RVAs: 0x144f690
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-14328202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8765e25ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5390006885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=22
//     evidence={"assembly":"\tcallq\t-14329931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8b55725ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390008630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=6
//     target_aliases=1699 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21284683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8b538bbfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5390006870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ff020
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18157218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e85ef1eafe","size":5,"soft_fail":false,"target_va":5371850784,"va":5390007997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14178387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8ada727ff","size":5,"soft_fail":false,"target_va":5375829056,"va":5390007438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408f9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11895531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8157d4aff","size":5,"soft_fail":false,"target_va":5378120480,"va":5390016006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de3df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6736123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8053799ff","size":5,"soft_fail":false,"target_va":5383273968,"va":5390010086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6690098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8ceea99ff","size":5,"soft_fail":false,"target_va":5383325024,"va":5390015117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6690625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8bfe899ff","size":5,"soft_fail":false,"target_va":5383325056,"va":5390015676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141405de0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-310242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e81e44fbff","size":5,"soft_fail":false,"target_va":5389704672,"va":5390014909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14142afd0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-155232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8a0a1fdff","size":5,"soft_fail":false,"target_va":5389856720,"va":5390011947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146cea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t109783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8d7ac0100","size":5,"soft_fail":false,"target_va":5390126752,"va":5390016964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141478800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t166698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e82a8b0200","size":5,"soft_fail":false,"target_va":5390174208,"va":5390007505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26341286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8a6ef9101","size":5,"soft_fail":false,"target_va":5416350720,"va":5390009429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26341730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e862f19101","size":5,"soft_fail":false,"target_va":5416353040,"va":5390011305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27047547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e87bb69c01","size":5,"soft_fail":false,"target_va":5417057088,"va":5390009536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27049955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8e3bf9c01","size":5,"soft_fail":false,"target_va":5417057664,"va":5390007704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27046361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8d9b19c01","size":5,"soft_fail":false,"target_va":5417058032,"va":5390011666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bf80
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27046478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e84eb29c01","size":5,"soft_fail":false,"target_va":5417058176,"va":5390011693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26603751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8e7f09501","size":5,"soft_fail":false,"target_va":5416620769,"va":5390017013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::grow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12640130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e87e203fff","size":5,"soft_fail":false,"target_va":5377374288,"va":5390014413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26355956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8f4289201","size":5,"soft_fail":false,"target_va":5416366080,"va":5390010119,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26612969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8e9149601","size":5,"soft_fail":false,"target_va":5416620794,"va":5390007820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26604488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8c8f39501","size":5,"soft_fail":false,"target_va":5416621408,"va":5390016915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed::<usize, usize>
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26604384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e860f39501","size":5,"soft_fail":false,"target_va":5416621188,"va":5390016799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26605453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e88df79501","size":5,"soft_fail":false,"target_va":5416622208,"va":5390016750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26604611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e843f49501","size":5,"soft_fail":false,"target_va":5416621762,"va":5390017146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t129261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8edf80100","size":5,"soft_fail":false,"target_va":5390146576,"va":5390017310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8d16f0000","size":5,"soft_fail":false,"target_va":5390034704,"va":5390006074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26604158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e87ef29501","size":5,"soft_fail":false,"target_va":5416621376,"va":5390017213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::BrickTransform::encode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11534799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e831fe4fff","size":5,"soft_fail":false,"target_va":5378481680,"va":5390016474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::FinalizedWallHoles::wall_length
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8395295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8e1e57fff","size":5,"soft_fail":false,"target_va":5381612160,"va":5390007450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6629505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e87fd79aff","size":5,"soft_fail":false,"target_va":5383383152,"va":5390012652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6042639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8f1cba3ff","size":5,"soft_fail":false,"target_va":5383963792,"va":5390006426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6034132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e82ceda3ff","size":5,"soft_fail":false,"target_va":5383972224,"va":5390006351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6041895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8d9cea3ff","size":5,"soft_fail":false,"target_va":5383964912,"va":5390006802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6043237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e89bc9a3ff","size":5,"soft_fail":false,"target_va":5383962816,"va":5390006048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::_$LT$impl$u20$core..convert..From$LT$glam..f32..affine3a..Affine3A$GT$$u20$for$u20$rhapsody_core..shader..math..Affine3Packed$GT$::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t-6607993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8872b9bff","size":5,"soft_fail":false,"target_va":5383408256,"va":5390016244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26680078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e80e1b9701","size":5,"soft_fail":false,"target_va":5416697008,"va":5390016925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-249524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e84c31fcff","size":5,"soft_fail":false,"target_va":5389771136,"va":5390020655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26602284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e82ceb9501","size":5,"soft_fail":false,"target_va":5416619360,"va":5390017071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::perturb_row_ends
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12649699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e81dfb3eff","size":5,"soft_fail":false,"target_va":5377364624,"va":5390014318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::split_horizontally
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12649809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8affa3eff","size":5,"soft_fail":false,"target_va":5377365056,"va":5390014860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::split_into_random_rows::split_into_random_rows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12641468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8441b3fff","size":5,"soft_fail":false,"target_va":5377365648,"va":5390007111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::trim_rows::trim_rows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12637222","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8da2b3fff","size":5,"soft_fail":false,"target_va":5377370208,"va":5390007425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::wall_constructor::split_brick_by_holes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12849695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8e1ed3bff","size":5,"soft_fail":false,"target_va":5377164912,"va":5390014602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6713295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8319099ff","size":5,"soft_fail":false,"target_va":5383302240,"va":5390015530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::expanded
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6707014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8baa899ff","size":5,"soft_fail":false,"target_va":5383302352,"va":5390009361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6713234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e86e9099ff","size":5,"soft_fail":false,"target_va":5383302288,"va":5390015517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6707657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e837a699ff","size":5,"soft_fail":false,"target_va":5383301744,"va":5390009396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6713285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e83b9099ff","size":5,"soft_fail":false,"target_va":5383302224,"va":5390015504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6734366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390005904,"raw_hex":"e8e23d99ff","size":5,"soft_fail":false,"target_va":5383276416,"va":5390010777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1493,"record_offset":54580,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1493,"record_offset":54296,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":69816,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":13888,"source_line":1920,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":69876,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":5253,"source_line":2205,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":56044,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":10044,"source_line":798,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":71560,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":14132,"source_line":3663,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\iter.rs
//     evidence={"module":1493,"record_offset":72476,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":7433,"source_line":92,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":60972,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":55988,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":61000,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":61272,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":11070,"source_line":3465,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":56376,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":69032,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":64416,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":62820,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1493,"record_offset":71448,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":72872,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":53516,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":62096,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1493,"record_offset":62424,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":10940,"source_line":63,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":1493,"record_offset":57056,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":61908,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1493,"record_offset":68948,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":54860,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":56916,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1493,"record_offset":62876,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":11437,"source_line":123,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1493,"record_offset":69128,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":69416,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":55696,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":69444,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1493,"record_offset":53556,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1493,"record_offset":65900,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":53404,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":66040,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1493,"record_offset":72900,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":72620,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":71232,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":71532,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":71176,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1493,"record_offset":63400,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1493,"record_offset":66140,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":52596,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":51196,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":51584,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":56320,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_walls::construct_walls::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":1493,"record_offset":72216,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":79492,"source_line":158,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::BrickRowSine::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":66308,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":26643,"source_line":12,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::sine_wave_deform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":69648,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":26645,"source_line":45,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::sine_wave_pack
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":69788,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":26644,"source_line":73,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::WallConstructor::normalize_holes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":55724,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":26334,"source_line":56,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$5
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":66948,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":17827,"source_line":379,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1493,"record_offset":70908,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":14739,"source_line":803,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1493,"record_offset":66920,"parent_record_offset":50760,"ancestor_record_offset":50760,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":54384,"parent_record_offset":54360,"ancestor_record_offset":50760,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":54408,"parent_record_offset":54384,"ancestor_record_offset":50760,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":57468,"parent_record_offset":57444,"ancestor_record_offset":50760,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":57492,"parent_record_offset":57468,"ancestor_record_offset":50760,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":54432,"parent_record_offset":54408,"ancestor_record_offset":50760,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":57516,"parent_record_offset":57492,"ancestor_record_offset":50760,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":54360,"parent_record_offset":54328,"ancestor_record_offset":50760,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1493,"record_offset":57444,"parent_record_offset":57352,"ancestor_record_offset":50760,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1493,"record_offset":54328,"parent_record_offset":54296,"ancestor_record_offset":50760,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_slices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":71612,"parent_record_offset":71584,"ancestor_record_offset":50760,"inlinee":11741,"source_line":1563,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_range
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":71832,"parent_record_offset":71612,"ancestor_record_offset":50760,"inlinee":4789,"source_line":228,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":69960,"parent_record_offset":69936,"ancestor_record_offset":50760,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":69936,"parent_record_offset":69900,"ancestor_record_offset":50760,"inlinee":5255,"source_line":236,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":71584,"parent_record_offset":71560,"ancestor_record_offset":50760,"inlinee":11740,"source_line":1499,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":70220,"parent_record_offset":70048,"ancestor_record_offset":50760,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::push_back_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":69900,"parent_record_offset":69876,"ancestor_record_offset":50760,"inlinee":5254,"source_line":2223,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::slice_ranges
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":71640,"parent_record_offset":71612,"ancestor_record_offset":50760,"inlinee":4783,"source_line":1666,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":70048,"parent_record_offset":69900,"ancestor_record_offset":50760,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::truncate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":69844,"parent_record_offset":69816,"ancestor_record_offset":50760,"inlinee":13889,"source_line":1364,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":56072,"parent_record_offset":56044,"ancestor_record_offset":50760,"inlinee":10045,"source_line":853,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1493,"record_offset":70084,"parent_record_offset":70048,"ancestor_record_offset":50760,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":69984,"parent_record_offset":69960,"ancestor_record_offset":50760,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":61092,"parent_record_offset":61068,"ancestor_record_offset":50760,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":61420,"parent_record_offset":61392,"ancestor_record_offset":50760,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":55048,"parent_record_offset":55020,"ancestor_record_offset":50760,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":61504,"parent_record_offset":61480,"ancestor_record_offset":50760,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":57380,"parent_record_offset":57352,"ancestor_record_offset":50760,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":57352,"parent_record_offset":57328,"ancestor_record_offset":50760,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":61480,"parent_record_offset":61448,"ancestor_record_offset":50760,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":61140,"parent_record_offset":61116,"ancestor_record_offset":50760,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":61116,"parent_record_offset":61092,"ancestor_record_offset":50760,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":61448,"parent_record_offset":61420,"ancestor_record_offset":50760,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":55108,"parent_record_offset":55076,"ancestor_record_offset":50760,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":55076,"parent_record_offset":55048,"ancestor_record_offset":50760,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":57328,"parent_record_offset":57304,"ancestor_record_offset":50760,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1493,"record_offset":56540,"parent_record_offset":56496,"ancestor_record_offset":50760,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1493,"record_offset":62364,"parent_record_offset":62340,"ancestor_record_offset":50760,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1493,"record_offset":55804,"parent_record_offset":55724,"ancestor_record_offset":50760,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1493,"record_offset":55840,"parent_record_offset":55804,"ancestor_record_offset":50760,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":61068,"parent_record_offset":61028,"ancestor_record_offset":50760,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":55776,"parent_record_offset":55724,"ancestor_record_offset":50760,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":55292,"parent_record_offset":55268,"ancestor_record_offset":50760,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":61332,"parent_record_offset":61272,"ancestor_record_offset":50760,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":62388,"parent_record_offset":62364,"ancestor_record_offset":50760,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":61304,"parent_record_offset":61272,"ancestor_record_offset":50760,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":72244,"parent_record_offset":72216,"ancestor_record_offset":50760,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":61028,"parent_record_offset":61000,"ancestor_record_offset":50760,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":61392,"parent_record_offset":61332,"ancestor_record_offset":50760,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":54992,"parent_record_offset":54956,"ancestor_record_offset":50760,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":55020,"parent_record_offset":54992,"ancestor_record_offset":50760,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":54892,"parent_record_offset":54860,"ancestor_record_offset":50760,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1493,"record_offset":55556,"parent_record_offset":55528,"ancestor_record_offset":50760,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1493,"record_offset":55620,"parent_record_offset":55556,"ancestor_record_offset":50760,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1493,"record_offset":65568,"parent_record_offset":64480,"ancestor_record_offset":50760,"inlinee":5125,"source_line":13,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1493,"record_offset":65400,"parent_record_offset":65376,"ancestor_record_offset":50760,"inlinee":5117,"source_line":30,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1493,"record_offset":55268,"parent_record_offset":54956,"ancestor_record_offset":50760,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1493,"record_offset":54924,"parent_record_offset":54892,"ancestor_record_offset":50760,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1493,"record_offset":54956,"parent_record_offset":54924,"ancestor_record_offset":50760,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1493,"record_offset":56568,"parent_record_offset":56540,"ancestor_record_offset":50760,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":52924,"parent_record_offset":52896,"ancestor_record_offset":50760,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":52952,"parent_record_offset":52924,"ancestor_record_offset":50760,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":52872,"parent_record_offset":52844,"ancestor_record_offset":50760,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":52896,"parent_record_offset":52872,"ancestor_record_offset":50760,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":53012,"parent_record_offset":52896,"ancestor_record_offset":50760,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":1493,"record_offset":53124,"parent_record_offset":53100,"ancestor_record_offset":50760,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":57012,"parent_record_offset":56988,"ancestor_record_offset":50760,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":62924,"parent_record_offset":62900,"ancestor_record_offset":50760,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":62948,"parent_record_offset":62924,"ancestor_record_offset":50760,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":61776,"parent_record_offset":61744,"ancestor_record_offset":50760,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1493,"record_offset":56988,"parent_record_offset":56964,"ancestor_record_offset":50760,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":1493,"record_offset":72948,"parent_record_offset":72924,"ancestor_record_offset":50760,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1493,"record_offset":72980,"parent_record_offset":72900,"ancestor_record_offset":50760,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1493,"record_offset":72924,"parent_record_offset":72900,"ancestor_record_offset":50760,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":56344,"parent_record_offset":56320,"ancestor_record_offset":50760,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::fract
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":66432,"parent_record_offset":66408,"ancestor_record_offset":50760,"inlinee":26641,"source_line":1873,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":71372,"parent_record_offset":71348,"ancestor_record_offset":50760,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::trunc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1493,"record_offset":66456,"parent_record_offset":66432,"ancestor_record_offset":50760,"inlinee":21635,"source_line":1844,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1493,"record_offset":55916,"parent_record_offset":55876,"ancestor_record_offset":50760,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1493,"record_offset":56660,"parent_record_offset":56596,"ancestor_record_offset":50760,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1493,"record_offset":63660,"parent_record_offset":63636,"ancestor_record_offset":50760,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":1493,"record_offset":63708,"parent_record_offset":63684,"ancestor_record_offset":50760,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1493,"record_offset":63448,"parent_record_offset":63400,"ancestor_record_offset":50760,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1493,"record_offset":67388,"parent_record_offset":67348,"ancestor_record_offset":50760,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1493,"record_offset":57092,"parent_record_offset":57056,"ancestor_record_offset":50760,"inlinee":17819,"source_line":180,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1493,"record_offset":62496,"parent_record_offset":62452,"ancestor_record_offset":50760,"inlinee":10749,"source_line":752,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$20::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1493,"record_offset":62452,"parent_record_offset":62424,"ancestor_record_offset":50760,"inlinee":10747,"source_line":518,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1493,"record_offset":62596,"parent_record_offset":62568,"ancestor_record_offset":50760,"inlinee":10753,"source_line":380,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1493,"record_offset":62568,"parent_record_offset":62452,"ancestor_record_offset":50760,"inlinee":10751,"source_line":49,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1493,"record_offset":55340,"parent_record_offset":55316,"ancestor_record_offset":50760,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1493,"record_offset":62624,"parent_record_offset":62596,"ancestor_record_offset":50760,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1493,"record_offset":55404,"parent_record_offset":55364,"ancestor_record_offset":50760,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":1493,"record_offset":61972,"parent_record_offset":61940,"ancestor_record_offset":50760,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":56940,"parent_record_offset":56916,"ancestor_record_offset":50760,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":57128,"parent_record_offset":57092,"ancestor_record_offset":50760,"inlinee":17821,"source_line":124,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":61940,"parent_record_offset":61908,"ancestor_record_offset":50760,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":56964,"parent_record_offset":56940,"ancestor_record_offset":50760,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1493,"record_offset":57164,"parent_record_offset":57128,"ancestor_record_offset":50760,"inlinee":17822,"source_line":355,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1493,"record_offset":61744,"parent_record_offset":61712,"ancestor_record_offset":50760,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1493,"record_offset":61712,"parent_record_offset":61332,"ancestor_record_offset":50760,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1493,"record_offset":56880,"parent_record_offset":56856,"ancestor_record_offset":50760,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":55316,"parent_record_offset":55292,"ancestor_record_offset":50760,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":55528,"parent_record_offset":55404,"ancestor_record_offset":50760,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1493,"record_offset":63732,"parent_record_offset":63708,"ancestor_record_offset":50760,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1493,"record_offset":52976,"parent_record_offset":52952,"ancestor_record_offset":50760,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1493,"record_offset":56596,"parent_record_offset":56568,"ancestor_record_offset":50760,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1493,"record_offset":56628,"parent_record_offset":56596,"ancestor_record_offset":50760,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1493,"record_offset":57412,"parent_record_offset":57380,"ancestor_record_offset":50760,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1493,"record_offset":55488,"parent_record_offset":55460,"ancestor_record_offset":50760,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1493,"record_offset":65008,"parent_record_offset":64932,"ancestor_record_offset":50760,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":1493,"record_offset":55460,"parent_record_offset":55432,"ancestor_record_offset":50760,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1493,"record_offset":64968,"parent_record_offset":64932,"ancestor_record_offset":50760,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1493,"record_offset":52744,"parent_record_offset":52704,"ancestor_record_offset":50760,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1493,"record_offset":62676,"parent_record_offset":62652,"ancestor_record_offset":50760,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1493,"record_offset":62900,"parent_record_offset":62876,"ancestor_record_offset":50760,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":62528,"parent_record_offset":62496,"ancestor_record_offset":50760,"inlinee":4102,"source_line":766,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":63600,"parent_record_offset":63448,"ancestor_record_offset":50760,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":62652,"parent_record_offset":62624,"ancestor_record_offset":50760,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":63684,"parent_record_offset":63660,"ancestor_record_offset":50760,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1493,"record_offset":67800,"parent_record_offset":67348,"ancestor_record_offset":50760,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1493,"record_offset":51532,"parent_record_offset":51504,"ancestor_record_offset":50760,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1493,"record_offset":53360,"parent_record_offset":52704,"ancestor_record_offset":50760,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1493,"record_offset":53076,"parent_record_offset":52844,"ancestor_record_offset":50760,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1493,"record_offset":57232,"parent_record_offset":57208,"ancestor_record_offset":50760,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":53452,"parent_record_offset":53404,"ancestor_record_offset":50760,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":52004,"parent_record_offset":51980,"ancestor_record_offset":50760,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":51980,"parent_record_offset":51956,"ancestor_record_offset":50760,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1493,"record_offset":52188,"parent_record_offset":52160,"ancestor_record_offset":50760,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1493,"record_offset":55588,"parent_record_offset":55556,"ancestor_record_offset":50760,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1493,"record_offset":52288,"parent_record_offset":52260,"ancestor_record_offset":50760,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1493,"record_offset":52260,"parent_record_offset":52232,"ancestor_record_offset":50760,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1493,"record_offset":71984,"parent_record_offset":71584,"ancestor_record_offset":50760,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1493,"record_offset":69060,"parent_record_offset":69032,"ancestor_record_offset":50760,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1493,"record_offset":69088,"parent_record_offset":69060,"ancestor_record_offset":50760,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1493,"record_offset":65924,"parent_record_offset":65900,"ancestor_record_offset":50760,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":51932,"parent_record_offset":51908,"ancestor_record_offset":50760,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":51956,"parent_record_offset":51932,"ancestor_record_offset":50760,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":57208,"parent_record_offset":57164,"ancestor_record_offset":50760,"inlinee":6630,"source_line":418,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":52316,"parent_record_offset":52288,"ancestor_record_offset":50760,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":55364,"parent_record_offset":55340,"ancestor_record_offset":50760,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":62708,"parent_record_offset":62624,"ancestor_record_offset":50760,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":51908,"parent_record_offset":51792,"ancestor_record_offset":50760,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$180::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1493,"record_offset":62004,"parent_record_offset":61972,"ancestor_record_offset":50760,"inlinee":17824,"source_line":438,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":1493,"record_offset":55876,"parent_record_offset":55840,"ancestor_record_offset":50760,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1493,"record_offset":51324,"parent_record_offset":51300,"ancestor_record_offset":50760,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1493,"record_offset":51620,"parent_record_offset":51584,"ancestor_record_offset":50760,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1493,"record_offset":51760,"parent_record_offset":51728,"ancestor_record_offset":50760,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1493,"record_offset":51728,"parent_record_offset":51692,"ancestor_record_offset":50760,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":51668,"parent_record_offset":51644,"ancestor_record_offset":50760,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":51692,"parent_record_offset":51668,"ancestor_record_offset":50760,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":51644,"parent_record_offset":51620,"ancestor_record_offset":50760,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1493,"record_offset":52232,"parent_record_offset":51692,"ancestor_record_offset":50760,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":51300,"parent_record_offset":51268,"ancestor_record_offset":50760,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":51268,"parent_record_offset":51240,"ancestor_record_offset":50760,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":51424,"parent_record_offset":51392,"ancestor_record_offset":50760,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":51480,"parent_record_offset":51424,"ancestor_record_offset":50760,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":51392,"parent_record_offset":51364,"ancestor_record_offset":50760,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":51240,"parent_record_offset":51196,"ancestor_record_offset":50760,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1493,"record_offset":51504,"parent_record_offset":51480,"ancestor_record_offset":50760,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":51792,"parent_record_offset":51760,"ancestor_record_offset":50760,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":52076,"parent_record_offset":51792,"ancestor_record_offset":50760,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":52048,"parent_record_offset":51792,"ancestor_record_offset":50760,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":1493,"record_offset":51852,"parent_record_offset":51792,"ancestor_record_offset":50760,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1493,"record_offset":71496,"parent_record_offset":71472,"ancestor_record_offset":50760,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":1493,"record_offset":71472,"parent_record_offset":71448,"ancestor_record_offset":50760,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":71316,"parent_record_offset":71284,"ancestor_record_offset":50760,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":71284,"parent_record_offset":71256,"ancestor_record_offset":50760,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":71256,"parent_record_offset":71232,"ancestor_record_offset":50760,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":70720,"parent_record_offset":70692,"ancestor_record_offset":50760,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":70748,"parent_record_offset":70720,"ancestor_record_offset":50760,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1493,"record_offset":70832,"parent_record_offset":70720,"ancestor_record_offset":50760,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":1493,"record_offset":63636,"parent_record_offset":63400,"ancestor_record_offset":50760,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1493,"record_offset":64932,"parent_record_offset":64896,"ancestor_record_offset":50760,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":52632,"parent_record_offset":52596,"ancestor_record_offset":50760,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":53620,"parent_record_offset":53588,"ancestor_record_offset":50760,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":54072,"parent_record_offset":53868,"ancestor_record_offset":50760,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":1493,"record_offset":52772,"parent_record_offset":52744,"ancestor_record_offset":50760,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":52844,"parent_record_offset":52704,"ancestor_record_offset":50760,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1493,"record_offset":53048,"parent_record_offset":52844,"ancestor_record_offset":50760,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::fract
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":66408,"parent_record_offset":66356,"ancestor_record_offset":50760,"inlinee":26640,"source_line":178,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":66380,"parent_record_offset":66356,"ancestor_record_offset":50760,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":71348,"parent_record_offset":71316,"ancestor_record_offset":50760,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":1493,"record_offset":52800,"parent_record_offset":52772,"ancestor_record_offset":50760,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1493,"record_offset":52704,"parent_record_offset":52668,"ancestor_record_offset":50760,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1493,"record_offset":52668,"parent_record_offset":52632,"ancestor_record_offset":50760,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::sine_wave::random_f
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":66356,"parent_record_offset":66308,"ancestor_record_offset":50760,"inlinee":26639,"source_line":82,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::sine_wave::sine_wave_deform
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":69500,"parent_record_offset":69472,"ancestor_record_offset":50760,"inlinee":26645,"source_line":45,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":55432,"parent_record_offset":55404,"ancestor_record_offset":50760,"inlinee":11021,"source_line":142,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":64896,"parent_record_offset":64860,"ancestor_record_offset":50760,"inlinee":10929,"source_line":338,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$7
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":69472,"parent_record_offset":69444,"ancestor_record_offset":50760,"inlinee":17829,"source_line":528,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::wall_constructor::impl$0::from_curve::closure$9
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/wall_constructor.rs
//     evidence={"module":1493,"record_offset":70604,"parent_record_offset":70580,"ancestor_record_offset":50760,"inlinee":17830,"source_line":630,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1493,"record_offset":70632,"parent_record_offset":70604,"ancestor_record_offset":50760,"inlinee":14739,"source_line":803,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1493,"record_offset":70664,"parent_record_offset":70632,"ancestor_record_offset":50760,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1493,"record_offset":67032,"parent_record_offset":66948,"ancestor_record_offset":50760,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1493,"record_offset":67004,"parent_record_offset":66948,"ancestor_record_offset":50760,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1493,"record_offset":62844,"parent_record_offset":62820,"ancestor_record_offset":50760,"inlinee":17826,"source_line":6,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1493,"record_offset":70692,"parent_record_offset":70632,"ancestor_record_offset":50760,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$7::default
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D94
// 首条位置: {"module":94,"inlinee":11668}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::WallConstructor::normalize_holes
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x66DE
// 首条位置: {"module":409,"record_offset":280,"rva":8455632,"va":5377164752}
// 编译实例 RVAs: 0x8105d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14146cea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12961856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377164752,"raw_hex":"e840c8c500","size":5,"soft_fail":false,"target_va":5390126752,"va":5377164891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141478800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: wall_constructor.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13009375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377164752,"raw_hex":"e8df81c600","size":5,"soft_fail":false,"target_va":5390174208,"va":5377164828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t12981668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377164752,"raw_hex":"e8a415c600","size":5,"soft_fail":false,"target_va":5390146576,"va":5377164903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::FinalizedWallHoles::wall_length
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4447372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377164752,"raw_hex":"e88cdc4300","size":5,"soft_fail":false,"target_va":5381612160,"va":5377164783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":409,"record_offset":472,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":13088,"source_line":192,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":409,"record_offset":444,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":409,"record_offset":500,"parent_record_offset":472,"ancestor_record_offset":280,"inlinee":8433,"source_line":856,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":409,"record_offset":564,"parent_record_offset":528,"ancestor_record_offset":280,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":409,"record_offset":528,"parent_record_offset":500,"ancestor_record_offset":280,"inlinee":8435,"source_line":29,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13840
// 首条位置: {"module":1509,"inlinee":79936}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1509,"record_offset":628,"parent_record_offset":540,"ancestor_record_offset":540,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01509.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$22::mul_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1509,"record_offset":576,"parent_record_offset":540,"ancestor_record_offset":540,"inlinee":30716,"source_line":1293,"archive":"_pdb/C/modules/01509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1509,"record_offset":732,"parent_record_offset":700,"ancestor_record_offset":540,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1509,"record_offset":780,"parent_record_offset":756,"ancestor_record_offset":540,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1509,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":540,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1509,"record_offset":700,"parent_record_offset":660,"ancestor_record_offset":540,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1509,"record_offset":660,"parent_record_offset":628,"ancestor_record_offset":540,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01509.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1509,"record_offset":600,"parent_record_offset":576,"ancestor_record_offset":540,"inlinee":11807,"source_line":924,"archive":"_pdb/C/modules/01509.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1509,"record_offset":820,"parent_record_offset":660,"ancestor_record_offset":540,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01509.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=43
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13805
// 首条位置: {"module":1500,"inlinee":79877}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=38
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1500,"record_offset":6808,"parent_record_offset":6788,"ancestor_record_offset":6788,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01500.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 142
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2B0D
// 首条位置: {"module":81,"inlinee":11021}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":249,"record_offset":4988,"parent_record_offset":4960,"ancestor_record_offset":4960,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":249,"record_offset":5016,"parent_record_offset":4988,"ancestor_record_offset":4960,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 206
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2ADF
// 首条位置: {"module":81,"inlinee":10975}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 338
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2AB1
// 首条位置: {"module":81,"inlinee":10929}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":249,"record_offset":20256,"parent_record_offset":20220,"ancestor_record_offset":20220,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":249,"record_offset":20416,"parent_record_offset":20256,"ancestor_record_offset":20220,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":249,"record_offset":20332,"parent_record_offset":20256,"ancestor_record_offset":20220,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":249,"record_offset":20292,"parent_record_offset":20256,"ancestor_record_offset":20220,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 375
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2684
// 首条位置: {"module":63,"inlinee":9860}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":63,"record_offset":2744,"parent_record_offset":2724,"ancestor_record_offset":2724,"inlinee":9861,"source_line":125,"archive":"_pdb/C/modules/00063.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=24
// 源码行: 379
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x45A3
// 首条位置: {"module":249,"inlinee":17827}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=48
//     packages=[{"name":"lerp","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lerp-0.4.0/src/lib.rs
//     evidence={"module":249,"record_offset":14016,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":8968,"source_line":171,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=18
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":249,"record_offset":13988,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=18
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":249,"record_offset":13960,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":249,"record_offset":14100,"parent_record_offset":14016,"ancestor_record_offset":13904,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=74
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":249,"record_offset":14072,"parent_record_offset":14016,"ancestor_record_offset":13904,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":249,"record_offset":14044,"parent_record_offset":14016,"ancestor_record_offset":13904,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 528
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x45A5
// 首条位置: {"module":249,"inlinee":17829}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::utils::sine_wave::sine_wave_deform
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/sine_wave.rs
//     evidence={"module":1493,"record_offset":19076,"parent_record_offset":19048,"ancestor_record_offset":19048,"inlinee":26645,"source_line":45,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1493,"record_offset":19128,"parent_record_offset":19076,"ancestor_record_offset":19048,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/01493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$9
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 630
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x45A6
// 首条位置: {"module":249,"inlinee":17830}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord_unchecked
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":249,"record_offset":17280,"parent_record_offset":17252,"ancestor_record_offset":17252,"inlinee":14739,"source_line":803,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":249,"record_offset":17504,"parent_record_offset":17480,"ancestor_record_offset":17252,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":249,"record_offset":17420,"parent_record_offset":17396,"ancestor_record_offset":17252,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":17368,"parent_record_offset":17340,"ancestor_record_offset":17252,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":17396,"parent_record_offset":17368,"ancestor_record_offset":17252,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":249,"record_offset":17480,"parent_record_offset":17368,"ancestor_record_offset":17252,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":249,"record_offset":17312,"parent_record_offset":17280,"ancestor_record_offset":17252,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":249,"record_offset":17340,"parent_record_offset":17280,"ancestor_record_offset":17252,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::wall_constructor::split_brick_by_holes
// 记录形式: compiled_procedure; instances=1
// 源码行: 767
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":409,"record_offset":636,"rva":8455792,"va":5377164912}
// 编译实例 RVAs: 0x810670
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39456805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377164912,"raw_hex":"e825105a02","size":5,"soft_fail":false,"target_va":5416622320,"va":5377165510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::resolve_hole_overlap::resolve_hole_overlap
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13009731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377164912,"raw_hex":"e84383c600","size":5,"soft_fail":false,"target_va":5390175104,"va":5377165368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::expanded
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6137195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377164912,"raw_hex":"e86ba55d00","size":5,"soft_fail":false,"target_va":5383302352,"va":5377165152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t6136576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377164912,"raw_hex":"e800a35d00","size":5,"soft_fail":false,"target_va":5383301744,"va":5377165163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":409,"record_offset":996,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":26330,"source_line":1522,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::iter_mut::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\iter_mut.rs
//     evidence={"module":409,"record_offset":1540,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":26332,"source_line":157,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":409,"record_offset":940,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":409,"record_offset":800,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":409,"record_offset":1748,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::as_mut_slices
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":409,"record_offset":1024,"parent_record_offset":996,"ancestor_record_offset":636,"inlinee":4782,"source_line":1612,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_range
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":409,"record_offset":1244,"parent_record_offset":1024,"ancestor_record_offset":636,"inlinee":4789,"source_line":228,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":409,"record_offset":1140,"parent_record_offset":1112,"ancestor_record_offset":636,"inlinee":4786,"source_line":991,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":409,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":636,"inlinee":4790,"source_line":173,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::slice_ranges
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":409,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":636,"inlinee":4783,"source_line":1666,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":409,"record_offset":1088,"parent_record_offset":1052,"ancestor_record_offset":636,"inlinee":4784,"source_line":248,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":409,"record_offset":1112,"parent_record_offset":1088,"ancestor_record_offset":636,"inlinee":4785,"source_line":243,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":409,"record_offset":1172,"parent_record_offset":1140,"ancestor_record_offset":636,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":409,"record_offset":1300,"parent_record_offset":1268,"ancestor_record_offset":636,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":409,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":636,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":409,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":636,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":409,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":636,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":409,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":636,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":409,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":636,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":409,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":636,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":409,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":636,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":409,"record_offset":964,"parent_record_offset":940,"ancestor_record_offset":636,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":409,"record_offset":1444,"parent_record_offset":1420,"ancestor_record_offset":636,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":409,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":636,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":409,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":636,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":409,"record_offset":1396,"parent_record_offset":996,"ancestor_record_offset":636,"inlinee":6145,"source_line":1060,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":409,"record_offset":1812,"parent_record_offset":1776,"ancestor_record_offset":636,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":409,"record_offset":1776,"parent_record_offset":1748,"ancestor_record_offset":636,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":409,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":636,"inlinee":6147,"source_line":221,"archive":"_pdb/C/modules/00409.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":409,"record_offset":1576,"parent_record_offset":1540,"ancestor_record_offset":636,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00409.jsonl.gz"}
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
// 完整结构及原始类型记录: wall_constructor.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::wall_constructor::WallConstructor
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >", "constructor": "system_wall_constructor::wall_constructor::WallConstructor::from_curve", "arguments": ["system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >", "constructor": "system_wall_constructor::wall_constructor::WallConstructor::from_curve", "arguments": ["system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>", "constructor": "system_wall_constructor::wall_constructor::WallConstructor::from_curve", "arguments": ["system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>", "constructor": "system_wall_constructor::wall_constructor::WallConstructor::from_curve", "arguments": ["system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3>", "constructor": "system_wall_constructor::wall_constructor::WallConstructor::from_curve", "arguments": ["system_wall_constructor::construct_walls::construct_walls::closure_env$3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>", "constructor": "system_wall_constructor::wall_constructor::WallConstructor::from_curve", "arguments": ["system_wall_constructor::construct_gates::place_brick_steps::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_constructor.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-production/findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/wall_constructor.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_constructor.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 19; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<f32>,ref$<f32>)
// 候选参数类型: ["ref$<f32>", "ref$<f32>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>,ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>)
// 候选参数类型: ["ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>", "ref$<country_core::resources::stairs::stairs_state::StairGraphEdge>"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>
// 信息 ID: bff33ee441c2bef2e7807891
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::wall_constructor::WallConstructor::from_curve -> ["system_wall_constructor::construct_gates::place_brick_steps::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0x1431acd -> 0x14530a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>
// 信息 ID: e580c01cc0561b8cad7ceb4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::wall_constructor::WallConstructor::from_curve -> ["system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x141f7f7 -> 0x144c020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >
// 信息 ID: 5f46ed742ccabe56cb2c5788
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b8e6acdd33b13c966488eca7: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// [类型关系] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9 -> ["false"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::wall_constructor::WallConstructor::from_curve -> ["system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x340aab -> 0x678c70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x3456d4 -> 0x678c70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >
// 信息 ID: 1e02cf1e6286f5609bcd9203
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b8e6acdd33b13c966488eca7: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// [类型关系] system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9 -> ["true"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::wall_constructor::WallConstructor::from_curve -> ["system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x340aab -> 0x678c70 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)> @ 0x3456d4 -> 0x678c70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>
// 信息 ID: 25d4cee9fcb505dc8ee48b5f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::wall_constructor::WallConstructor::from_curve -> ["system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals @ 0x81fb50 -> 0x1448600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3>
// 信息 ID: 53edb157734b18cee35986b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::wall_constructor::WallConstructor::from_curve -> ["system_wall_constructor::construct_walls::construct_walls::closure_env$3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x8057b5 -> 0x144f690 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::WallConstructor::normalize_holes
// 信息 ID: ba752a4875dec8123ec4b520
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x679285 -> 0x8105d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>; module=1493, inline_record=5300; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1493, inline_record=30868; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1493, inline_record=55724; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>; module=1493, inline_record=81468; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$0
// 信息 ID: 5d09352266db958e41522295
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::from_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=4960; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=29308; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>; module=1493, inline_record=5008; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1493, inline_record=30576; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1493, inline_record=55432; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>; module=1493, inline_record=81176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$1
// 信息 ID: ececf8370b75c36107f7e344
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::from_curve", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$1<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$1<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > > >; module=81, inline_record=120812; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > >; module=1455, inline_record=75696; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$3
// 信息 ID: 2aa78c3f5bdd294f3aa6e7fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::from_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=20220; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=44568; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>; module=1493, inline_record=14472; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1493, inline_record=36248; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1493, inline_record=64896; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>; module=1493, inline_record=86924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$4
// 信息 ID: 8c23272c773b72a3c1f8e551
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::from_curve", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec2::Vec2> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,tuple$<tuple$<usize,f32>,tuple$<usize,f32> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec2::Vec2> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,tuple$<tuple$<usize,f32>,tuple$<usize,f32> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec2::Vec2> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,tuple$<tuple$<usize,f32>,tuple$<usize,f32> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4 <- itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec2::Vec2> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,tuple$<tuple$<usize,f32>,tuple$<usize,f32> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4 <- utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4 <- utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4 <- utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4 <- utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 14 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec2::Vec2> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,tuple$<tuple$<usize,f32>,tuple$<usize,f32> > >; module=63, inline_record=2724; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec2::Vec2> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > > > >,tuple$<tuple$<usize,f32>,tuple$<usize,f32> > >; module=63, inline_record=3380; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> > >; module=1478, inline_record=8444; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> > >; module=1478, inline_record=9044; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec2::Vec2> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > >,tuple$<tuple$<usize,f32>,tuple$<usize,f32> > >; module=1504, inline_record=2652; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$7::next<core::iter::adapters::map::Map<take_until::take_until::TakeUntil<core::iter::adapters::zip::Zip<core::ops::range::RangeFrom<usize>,core::iter::adapters::skip::Skip<core::slice::iter::Iter<glam::f32::vec2::Vec2> > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > >,utils::curve::curve_slice::impl$6::extend_right_until::closure_env$1<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>,utils::curve::curve_slice::impl$6::extend_right::closure_env$0<glam::f32::vec2::Vec2,slice2$<glam::f32::vec2::Vec2>,utils::curve::CurveCoordSemantic2D,f32,system_wall_constructor::wall_constructor::impl$0::from_curve::closure_env$4<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> > > >,tuple$<tuple$<usize,f32>,tuple$<usize,f32> > >; module=1504, inline_record=3296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$5
// 信息 ID: a83550897f348639fcb6168e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::from_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=13904; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=14816; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=15196; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=38252; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=39164; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=39544; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>; module=1493, inline_record=16524; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>; module=1493, inline_record=17436; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$7
// 信息 ID: 6f4f5e436be4c3d06f349ba9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::from_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=16376; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=40724; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>; module=1493, inline_record=19048; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1493, inline_record=40272; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1493, inline_record=69472; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>; module=1493, inline_record=90948; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::from_curve::closure$9
// 信息 ID: 9ebbd0f32cf8a20dcd63752e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::from_curve", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> >; module=249, inline_record=17252; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> >; module=249, inline_record=41600; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3>; module=1493, inline_record=20176; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1493, inline_record=41404; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1493, inline_record=70604; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0>; module=1493, inline_record=92080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure$0
// 信息 ID: 636d3a5e3a84a865405fa2d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::normalize_holes", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$0 <- country_core::resources::walls::wall_holes::FinalizedWallHoles::iter_holes_with_padding<system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] country_core::resources::walls::wall_holes::FinalizedWallHoles::iter_holes_with_padding<system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$0>; module=1509, inline_record=540; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_holes::FinalizedWallHoles::iter_holes_with_padding<system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$0>; module=1509, inline_record=1036; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_holes::FinalizedWallHoles::iter_holes_with_padding<system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$0>; module=1509, inline_record=1452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure$1
// 信息 ID: fd3d03888bf1b19b3eaad641
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::wall_constructor::impl$0::normalize_holes", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::shared::smallsort::sort8_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::shared::smallsort::small_sort_general_with_scratch<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::stable::driftsort_main<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::stable::drift::sort<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::stable::quicksort::quicksort<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1 <- core::slice::sort::stable::merge::merge<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 38 条；闭包环境类型引用 9 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1500, inline_record=6788; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1500, inline_record=6888; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1500, inline_record=7052; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1500, inline_record=7172; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1500, inline_record=7408; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort4_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1500, inline_record=7744; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1500, inline_record=29384; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::sort8_stable<utils::geometry::aabb::Aabb2,alloc::slice::impl$0::sort_by::closure_env$0<utils::geometry::aabb::Aabb2,system_wall_constructor::wall_constructor::impl$0::normalize_holes::closure_env$1> >; module=1500, inline_record=29800; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::impl$7::default
// 信息 ID: 991275c3634aea9875af1065
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b8e6acdd33b13c966488eca7: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)>; module=94, inline_record=78856; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> > >,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)>; module=94, inline_record=128284; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=403, inline_record=7064; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals; module=417, inline_record=22008; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0>; module=1473, inline_record=808; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1487, inline_record=16360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::wall_constructor::split_brick_by_holes
// 信息 ID: 5281a2de03f92c496af75809
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<true> > | system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::closure_env$9<false> > @ 0x67a7be -> 0x810670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals::closure_env$3> @ 0x144a81c -> 0x810670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x144dec0 -> 0x810670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x145188a -> 0x810670 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve<system_wall_constructor::construct_gates::place_brick_steps::closure_env$0> @ 0x1454f50 -> 0x810670 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
