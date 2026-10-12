// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/decorator/src/decorator_visual/trap_doors.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 88dac99b1d7e26588ef7516f99dae9916456311e224fdd76c0951745a03063c9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: trap_doors.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 447
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use parry3d; // namespace candidate
//   package identities: [{"name":"parry3d","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.0"}]
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
// 函数符号: system_decorator::decorator_visual::trap_doors::generate_trap_doors
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":447,"record_offset":272,"rva":8888128,"va":5377597248}
// 编译实例 RVAs: 0x879f40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-1920643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e87db1e2ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5377599326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-1924558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e832a2e2ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5377603257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=6
//     target_aliases=1699 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8877126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e8ba8b78ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5377599313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400ba370
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8130549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e80bf083ff","size":5,"soft_fail":false,"target_va":5369471856,"va":5377602400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4118874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e8a626c1ff","size":5,"soft_fail":false,"target_va":5373479776,"va":5377598645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14094d4f0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t863842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e8622e0d00","size":5,"soft_fail":false,"target_va":5378462960,"va":5377599113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a44730
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1877441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e8c1a51c00","size":5,"soft_fail":false,"target_va":5379475248,"va":5377597802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c5a060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4062985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e809ff3d00","size":5,"soft_fail":false,"target_va":5381660768,"va":5377597778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c915a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4284503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e857604100","size":5,"soft_fail":false,"target_va":5381887392,"va":5377602884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140deb2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5704851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e8930c5700","size":5,"soft_fail":false,"target_va":5383303904,"va":5377599048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: trap_doors.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39457669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e885135a02","size":5,"soft_fail":false,"target_va":5417057056,"va":5377599382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39019068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e83c625302","size":5,"soft_fail":false,"target_va":5416620769,"va":5377601696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38781715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e813c34f02","size":5,"soft_fail":false,"target_va":5416382400,"va":5377600680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t39019228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e8dc625302","size":5,"soft_fail":false,"target_va":5416620794,"va":5377601561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39019775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e8ff645302","size":5,"soft_fail":false,"target_va":5416621408,"va":5377601628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39020054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e816665302","size":5,"soft_fail":false,"target_va":5416621600,"va":5377601541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t39020487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e8c7675302","size":5,"soft_fail":false,"target_va":5416622240,"va":5377601748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t7084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e8ac1b0000","size":5,"soft_fail":false,"target_va":5377604496,"va":5377597407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39019585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e841645302","size":5,"soft_fail":false,"target_va":5416621376,"va":5377601786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3599103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e8ffea3600","size":5,"soft_fail":false,"target_va":5381197456,"va":5377598348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4056867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e823e73d00","size":5,"soft_fail":false,"target_va":5381655760,"va":5377598888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::Decorators::iter_modified
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1876799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e83fa31c00","size":5,"soft_fail":false,"target_va":5379475184,"va":5377598380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4055495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e8c7e13d00","size":5,"soft_fail":false,"target_va":5381654320,"va":5377598820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::dormers::DormerHoleSsbo::cottage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2775382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e856592a00","size":5,"soft_fail":false,"target_va":5380376608,"va":5377601221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3305462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e8f66f3200","size":5,"soft_fail":false,"target_va":5380903536,"va":5377598069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3304333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e88d6b3200","size":5,"soft_fail":false,"target_va":5380902448,"va":5377598110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastMeshData::new_cuboid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1568091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e85bed1700","size":5,"soft_fail":false,"target_va":5379167328,"va":5377599232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6366041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e859236100","size":5,"soft_fail":false,"target_va":5383963792,"va":5377597746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6374548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e894446100","size":5,"soft_fail":false,"target_va":5383972224,"va":5377597671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6363487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e85f196100","size":5,"soft_fail":false,"target_va":5383964912,"va":5377601420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t6365434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e8fa206100","size":5,"soft_fail":false,"target_va":5383962816,"va":5377597377,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t39116053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e815dd5402","size":5,"soft_fail":false,"target_va":5416717696,"va":5377601638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t39017675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e8cb5c5302","size":5,"soft_fail":false,"target_va":5416619360,"va":5377601680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprint::set_colliders
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t469821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e83d2b0700","size":5,"soft_fail":false,"target_va":5378070256,"va":5377600430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::blueprint::DecoratorBlueprints::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t472670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e85e360700","size":5,"soft_fail":false,"target_va":5378071840,"va":5377599165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::intersect_circle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5706027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e82b115700","size":5,"soft_fail":false,"target_va":5383305104,"va":5377599072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point_padding
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5717808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377597248,"raw_hex":"e8303f5700","size":5,"soft_fail":false,"target_va":5383316784,"va":5377598971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":447,"record_offset":9812,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":447,"record_offset":10552,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":447,"record_offset":6564,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":447,"record_offset":6620,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":447,"record_offset":2800,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":447,"record_offset":2772,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":447,"record_offset":3368,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":447,"record_offset":6532,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":447,"record_offset":3336,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":447,"record_offset":11652,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::color_id::impl$22::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":447,"record_offset":10524,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":14661,"source_line":28,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":447,"record_offset":2716,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26241,"source_line":273,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$133::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":447,"record_offset":6648,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":27061,"source_line":1559,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":447,"record_offset":10808,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":447,"record_offset":10780,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":447,"record_offset":1908,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":447,"record_offset":504,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":447,"record_offset":896,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::blueprint::DecoratorBlueprint::set_outline
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/blueprint.rs
//     evidence={"module":447,"record_offset":10368,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26690,"source_line":270,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::custom_shader
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":447,"record_offset":10496,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26695,"source_line":133,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/utils/visual_entity_builder.rs
//     evidence={"module":447,"record_offset":10468,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26692,"source_line":55,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":447,"record_offset":7120,"parent_record_offset":7096,"ancestor_record_offset":272,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":447,"record_offset":7144,"parent_record_offset":7120,"ancestor_record_offset":272,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":447,"record_offset":9364,"parent_record_offset":9340,"ancestor_record_offset":272,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":447,"record_offset":9388,"parent_record_offset":9364,"ancestor_record_offset":272,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":447,"record_offset":7168,"parent_record_offset":7144,"ancestor_record_offset":272,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":447,"record_offset":9412,"parent_record_offset":9388,"ancestor_record_offset":272,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":447,"record_offset":7096,"parent_record_offset":6916,"ancestor_record_offset":272,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":447,"record_offset":9340,"parent_record_offset":9256,"ancestor_record_offset":272,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":447,"record_offset":9892,"parent_record_offset":9848,"ancestor_record_offset":272,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":447,"record_offset":9848,"parent_record_offset":9812,"ancestor_record_offset":272,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":447,"record_offset":9256,"parent_record_offset":9232,"ancestor_record_offset":272,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$16::clone
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":447,"record_offset":6724,"parent_record_offset":6648,"ancestor_record_offset":272,"inlinee":27063,"source_line":2041,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":447,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":447,"record_offset":6856,"parent_record_offset":6828,"ancestor_record_offset":272,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":447,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":447,"record_offset":4044,"parent_record_offset":4020,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":447,"record_offset":6916,"parent_record_offset":6884,"ancestor_record_offset":272,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":447,"record_offset":6884,"parent_record_offset":6856,"ancestor_record_offset":272,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":447,"record_offset":6952,"parent_record_offset":6916,"ancestor_record_offset":272,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":447,"record_offset":6756,"parent_record_offset":6724,"ancestor_record_offset":272,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":447,"record_offset":6788,"parent_record_offset":6756,"ancestor_record_offset":272,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":447,"record_offset":9092,"parent_record_offset":9068,"ancestor_record_offset":272,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":447,"record_offset":12556,"parent_record_offset":12532,"ancestor_record_offset":272,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":447,"record_offset":10624,"parent_record_offset":10580,"ancestor_record_offset":272,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":447,"record_offset":3996,"parent_record_offset":3972,"ancestor_record_offset":272,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":447,"record_offset":3972,"parent_record_offset":3940,"ancestor_record_offset":272,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":447,"record_offset":10580,"parent_record_offset":10552,"ancestor_record_offset":272,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":447,"record_offset":6828,"parent_record_offset":6788,"ancestor_record_offset":272,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":447,"record_offset":3940,"parent_record_offset":3916,"ancestor_record_offset":272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":447,"record_offset":3728,"parent_record_offset":3444,"ancestor_record_offset":272,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":447,"record_offset":6588,"parent_record_offset":6564,"ancestor_record_offset":272,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":447,"record_offset":3492,"parent_record_offset":3444,"ancestor_record_offset":272,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":447,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":272,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":447,"record_offset":3632,"parent_record_offset":3600,"ancestor_record_offset":272,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":447,"record_offset":3656,"parent_record_offset":3632,"ancestor_record_offset":272,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":447,"record_offset":4704,"parent_record_offset":4680,"ancestor_record_offset":272,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":447,"record_offset":4148,"parent_record_offset":3444,"ancestor_record_offset":272,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":447,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":272,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":447,"record_offset":4680,"parent_record_offset":3444,"ancestor_record_offset":272,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":447,"record_offset":4728,"parent_record_offset":4704,"ancestor_record_offset":272,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":447,"record_offset":4628,"parent_record_offset":4604,"ancestor_record_offset":272,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":447,"record_offset":4272,"parent_record_offset":4248,"ancestor_record_offset":272,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":447,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":272,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":447,"record_offset":4452,"parent_record_offset":4428,"ancestor_record_offset":272,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":447,"record_offset":4224,"parent_record_offset":4196,"ancestor_record_offset":272,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":447,"record_offset":4196,"parent_record_offset":4172,"ancestor_record_offset":272,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":447,"record_offset":4752,"parent_record_offset":4728,"ancestor_record_offset":272,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":447,"record_offset":3916,"parent_record_offset":3444,"ancestor_record_offset":272,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":447,"record_offset":4604,"parent_record_offset":4580,"ancestor_record_offset":272,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":447,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":272,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":447,"record_offset":3420,"parent_record_offset":3396,"ancestor_record_offset":272,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":447,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":272,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":447,"record_offset":9284,"parent_record_offset":9256,"ancestor_record_offset":272,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":447,"record_offset":6976,"parent_record_offset":6952,"ancestor_record_offset":272,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":447,"record_offset":3764,"parent_record_offset":3728,"ancestor_record_offset":272,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":447,"record_offset":2236,"parent_record_offset":2208,"ancestor_record_offset":272,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":447,"record_offset":2264,"parent_record_offset":2236,"ancestor_record_offset":272,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":447,"record_offset":2184,"parent_record_offset":2156,"ancestor_record_offset":272,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":447,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":272,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":447,"record_offset":2324,"parent_record_offset":2208,"ancestor_record_offset":272,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":447,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":272,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":447,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":272,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":447,"record_offset":2740,"parent_record_offset":2716,"ancestor_record_offset":272,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":447,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":272,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":447,"record_offset":5816,"parent_record_offset":5792,"ancestor_record_offset":272,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":447,"record_offset":5900,"parent_record_offset":5876,"ancestor_record_offset":272,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":447,"record_offset":11252,"parent_record_offset":10832,"ancestor_record_offset":272,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":447,"record_offset":11160,"parent_record_offset":11104,"ancestor_record_offset":272,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":447,"record_offset":10928,"parent_record_offset":10904,"ancestor_record_offset":272,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":447,"record_offset":10960,"parent_record_offset":10876,"ancestor_record_offset":272,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":447,"record_offset":11024,"parent_record_offset":10832,"ancestor_record_offset":272,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":447,"record_offset":6316,"parent_record_offset":5680,"ancestor_record_offset":272,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":447,"record_offset":7056,"parent_record_offset":7000,"ancestor_record_offset":272,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":447,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":272,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":447,"record_offset":9308,"parent_record_offset":9284,"ancestor_record_offset":272,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":447,"record_offset":7000,"parent_record_offset":6976,"ancestor_record_offset":272,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":447,"record_offset":7028,"parent_record_offset":7000,"ancestor_record_offset":272,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":447,"record_offset":6048,"parent_record_offset":6020,"ancestor_record_offset":272,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":447,"record_offset":3024,"parent_record_offset":3000,"ancestor_record_offset":272,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":447,"record_offset":2056,"parent_record_offset":2016,"ancestor_record_offset":272,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":447,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":272,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":447,"record_offset":4388,"parent_record_offset":4364,"ancestor_record_offset":272,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":447,"record_offset":3600,"parent_record_offset":3516,"ancestor_record_offset":272,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":447,"record_offset":10404,"parent_record_offset":10368,"ancestor_record_offset":272,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":447,"record_offset":10428,"parent_record_offset":10404,"ancestor_record_offset":272,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":447,"record_offset":3276,"parent_record_offset":2800,"ancestor_record_offset":272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":447,"record_offset":3700,"parent_record_offset":3444,"ancestor_record_offset":272,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":447,"record_offset":6488,"parent_record_offset":5512,"ancestor_record_offset":272,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":447,"record_offset":7856,"parent_record_offset":6648,"ancestor_record_offset":272,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":447,"record_offset":844,"parent_record_offset":816,"ancestor_record_offset":272,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":447,"record_offset":2672,"parent_record_offset":2016,"ancestor_record_offset":272,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":447,"record_offset":7224,"parent_record_offset":6788,"ancestor_record_offset":272,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":447,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":272,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":447,"record_offset":2388,"parent_record_offset":2156,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":447,"record_offset":3868,"parent_record_offset":3836,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":447,"record_offset":3144,"parent_record_offset":3120,"ancestor_record_offset":272,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":447,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":272,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":447,"record_offset":1292,"parent_record_offset":1268,"ancestor_record_offset":272,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":447,"record_offset":1500,"parent_record_offset":1472,"ancestor_record_offset":272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":447,"record_offset":10744,"parent_record_offset":10580,"ancestor_record_offset":272,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":447,"record_offset":1600,"parent_record_offset":1572,"ancestor_record_offset":272,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":447,"record_offset":1572,"parent_record_offset":1544,"ancestor_record_offset":272,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":447,"record_offset":3544,"parent_record_offset":3516,"ancestor_record_offset":272,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":447,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":272,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":447,"record_offset":4508,"parent_record_offset":4476,"ancestor_record_offset":272,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":447,"record_offset":1244,"parent_record_offset":1220,"ancestor_record_offset":272,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":447,"record_offset":1268,"parent_record_offset":1244,"ancestor_record_offset":272,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":447,"record_offset":1628,"parent_record_offset":1600,"ancestor_record_offset":272,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":447,"record_offset":1220,"parent_record_offset":1104,"ancestor_record_offset":272,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":447,"record_offset":632,"parent_record_offset":608,"ancestor_record_offset":272,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":447,"record_offset":932,"parent_record_offset":896,"ancestor_record_offset":272,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":447,"record_offset":1072,"parent_record_offset":1040,"ancestor_record_offset":272,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":447,"record_offset":1040,"parent_record_offset":1004,"ancestor_record_offset":272,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":447,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":272,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":447,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":272,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":447,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":272,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":447,"record_offset":1544,"parent_record_offset":1004,"ancestor_record_offset":272,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":447,"record_offset":608,"parent_record_offset":576,"ancestor_record_offset":272,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":447,"record_offset":576,"parent_record_offset":548,"ancestor_record_offset":272,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":447,"record_offset":736,"parent_record_offset":704,"ancestor_record_offset":272,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":447,"record_offset":792,"parent_record_offset":736,"ancestor_record_offset":272,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":447,"record_offset":704,"parent_record_offset":672,"ancestor_record_offset":272,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":447,"record_offset":548,"parent_record_offset":504,"ancestor_record_offset":272,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":447,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":272,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":447,"record_offset":1104,"parent_record_offset":1072,"ancestor_record_offset":272,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":447,"record_offset":1388,"parent_record_offset":1104,"ancestor_record_offset":272,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":447,"record_offset":1360,"parent_record_offset":1104,"ancestor_record_offset":272,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":447,"record_offset":1164,"parent_record_offset":1104,"ancestor_record_offset":272,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":447,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":272,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":447,"record_offset":12592,"parent_record_offset":12556,"ancestor_record_offset":272,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":447,"record_offset":9140,"parent_record_offset":9116,"ancestor_record_offset":272,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":447,"record_offset":12616,"parent_record_offset":12592,"ancestor_record_offset":272,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":447,"record_offset":12648,"parent_record_offset":12556,"ancestor_record_offset":272,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$33::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":447,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":272,"inlinee":7427,"source_line":155,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::impl$1::iter_modified::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":447,"record_offset":5512,"parent_record_offset":5476,"ancestor_record_offset":272,"inlinee":26676,"source_line":77,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":447,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":272,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":447,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":272,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":447,"record_offset":3788,"parent_record_offset":3764,"ancestor_record_offset":272,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":447,"record_offset":10832,"parent_record_offset":10808,"ancestor_record_offset":272,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":447,"record_offset":11280,"parent_record_offset":10832,"ancestor_record_offset":272,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":447,"record_offset":10904,"parent_record_offset":10876,"ancestor_record_offset":272,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":447,"record_offset":10876,"parent_record_offset":10832,"ancestor_record_offset":272,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":447,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":272,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":447,"record_offset":3000,"parent_record_offset":2968,"ancestor_record_offset":272,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":447,"record_offset":2940,"parent_record_offset":2900,"ancestor_record_offset":272,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":447,"record_offset":5792,"parent_record_offset":5680,"ancestor_record_offset":272,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":447,"record_offset":3064,"parent_record_offset":2900,"ancestor_record_offset":272,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":447,"record_offset":6344,"parent_record_offset":5680,"ancestor_record_offset":272,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":447,"record_offset":5876,"parent_record_offset":5680,"ancestor_record_offset":272,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":447,"record_offset":5736,"parent_record_offset":5680,"ancestor_record_offset":272,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":447,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":272,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":447,"record_offset":5572,"parent_record_offset":5544,"ancestor_record_offset":272,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":447,"record_offset":6200,"parent_record_offset":6092,"ancestor_record_offset":272,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":447,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":272,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":447,"record_offset":4980,"parent_record_offset":4956,"ancestor_record_offset":272,"inlinee":9034,"source_line":3365,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":447,"record_offset":5604,"parent_record_offset":5572,"ancestor_record_offset":272,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":447,"record_offset":6140,"parent_record_offset":6116,"ancestor_record_offset":272,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":447,"record_offset":3120,"parent_record_offset":2900,"ancestor_record_offset":272,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":447,"record_offset":6436,"parent_record_offset":5680,"ancestor_record_offset":272,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":447,"record_offset":2900,"parent_record_offset":2872,"ancestor_record_offset":272,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":447,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":272,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":447,"record_offset":5656,"parent_record_offset":5632,"ancestor_record_offset":272,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":447,"record_offset":5632,"parent_record_offset":5572,"ancestor_record_offset":272,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":447,"record_offset":5680,"parent_record_offset":5656,"ancestor_record_offset":272,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":447,"record_offset":5764,"parent_record_offset":5680,"ancestor_record_offset":272,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":447,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":272,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":447,"record_offset":6092,"parent_record_offset":5680,"ancestor_record_offset":272,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":447,"record_offset":4956,"parent_record_offset":4932,"ancestor_record_offset":272,"inlinee":11498,"source_line":1798,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":447,"record_offset":4784,"parent_record_offset":4752,"ancestor_record_offset":272,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":447,"record_offset":4476,"parent_record_offset":4452,"ancestor_record_offset":272,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] parry3d::shape::shared_shape::impl$4::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parry3d-0.17.0/src/shape/shared_shape.rs
//     evidence={"module":447,"record_offset":9068,"parent_record_offset":6648,"ancestor_record_offset":272,"inlinee":27065,"source_line":21,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":447,"record_offset":1944,"parent_record_offset":1908,"ancestor_record_offset":272,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":447,"record_offset":11716,"parent_record_offset":11684,"ancestor_record_offset":272,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":447,"record_offset":12168,"parent_record_offset":11964,"ancestor_record_offset":272,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":447,"record_offset":2084,"parent_record_offset":2056,"ancestor_record_offset":272,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":447,"record_offset":2156,"parent_record_offset":2016,"ancestor_record_offset":272,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":447,"record_offset":2360,"parent_record_offset":2156,"ancestor_record_offset":272,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":447,"record_offset":5544,"parent_record_offset":5512,"ancestor_record_offset":272,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":447,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":272,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$23::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":447,"record_offset":4932,"parent_record_offset":4900,"ancestor_record_offset":272,"inlinee":16842,"source_line":1676,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":447,"record_offset":2112,"parent_record_offset":2084,"ancestor_record_offset":272,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":447,"record_offset":2016,"parent_record_offset":1980,"ancestor_record_offset":272,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":447,"record_offset":1980,"parent_record_offset":1944,"ancestor_record_offset":272,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::trap_doors::generate_trap_doors::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/trap_doors.rs
//     evidence={"module":447,"record_offset":3396,"parent_record_offset":3368,"ancestor_record_offset":272,"inlinee":27059,"source_line":44,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$21::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":447,"record_offset":9180,"parent_record_offset":6648,"ancestor_record_offset":272,"inlinee":27067,"source_line":682,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_visual::trap_doors::generate_trap_doors::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69B3
// 首条位置: {"module":447,"inlinee":27059}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":447,"record_offset":3420,"parent_record_offset":3396,"ancestor_record_offset":3396,"inlinee":11704,"source_line":1388,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":447,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":3396,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":447,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":3396,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":447,"record_offset":4044,"parent_record_offset":4020,"ancestor_record_offset":3396,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":447,"record_offset":3996,"parent_record_offset":3972,"ancestor_record_offset":3396,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":447,"record_offset":3972,"parent_record_offset":3940,"ancestor_record_offset":3396,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":447,"record_offset":3940,"parent_record_offset":3916,"ancestor_record_offset":3396,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeId::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":447,"record_offset":3728,"parent_record_offset":3444,"ancestor_record_offset":3396,"inlinee":8565,"source_line":122,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":447,"record_offset":3492,"parent_record_offset":3444,"ancestor_record_offset":3396,"inlinee":8602,"source_line":948,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get_id_location
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":447,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":3396,"inlinee":8603,"source_line":955,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$18::get_id_location::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":447,"record_offset":3632,"parent_record_offset":3600,"ancestor_record_offset":3396,"inlinee":8620,"source_line":958,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::impl$37::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/entity/mod.rs
//     evidence={"module":447,"record_offset":3656,"parent_record_offset":3632,"ancestor_record_offset":3396,"inlinee":8622,"source_line":214,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":447,"record_offset":4704,"parent_record_offset":4680,"ancestor_record_offset":3396,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":447,"record_offset":4148,"parent_record_offset":3444,"ancestor_record_offset":3396,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":447,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":3396,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":447,"record_offset":4680,"parent_record_offset":3444,"ancestor_record_offset":3396,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":447,"record_offset":4728,"parent_record_offset":4704,"ancestor_record_offset":3396,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":447,"record_offset":4628,"parent_record_offset":4604,"ancestor_record_offset":3396,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":447,"record_offset":4272,"parent_record_offset":4248,"ancestor_record_offset":3396,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":447,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":3396,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":447,"record_offset":4452,"parent_record_offset":4428,"ancestor_record_offset":3396,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":447,"record_offset":4224,"parent_record_offset":4196,"ancestor_record_offset":3396,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":447,"record_offset":4196,"parent_record_offset":4172,"ancestor_record_offset":3396,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":447,"record_offset":4752,"parent_record_offset":4728,"ancestor_record_offset":3396,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":447,"record_offset":3916,"parent_record_offset":3444,"ancestor_record_offset":3396,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":447,"record_offset":4604,"parent_record_offset":4580,"ancestor_record_offset":3396,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":447,"record_offset":4580,"parent_record_offset":4556,"ancestor_record_offset":3396,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":447,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":3396,"inlinee":11705,"source_line":1539,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":447,"record_offset":3764,"parent_record_offset":3728,"ancestor_record_offset":3396,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":447,"record_offset":4364,"parent_record_offset":4300,"ancestor_record_offset":3396,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":447,"record_offset":4388,"parent_record_offset":4364,"ancestor_record_offset":3396,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":447,"record_offset":3600,"parent_record_offset":3516,"ancestor_record_offset":3396,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":447,"record_offset":4428,"parent_record_offset":4248,"ancestor_record_offset":3396,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":447,"record_offset":3700,"parent_record_offset":3444,"ancestor_record_offset":3396,"inlinee":5838,"source_line":1337,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":447,"record_offset":3868,"parent_record_offset":3836,"ancestor_record_offset":3396,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":447,"record_offset":3544,"parent_record_offset":3516,"ancestor_record_offset":3396,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":447,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":3396,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":447,"record_offset":4508,"parent_record_offset":4476,"ancestor_record_offset":3396,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":447,"record_offset":3812,"parent_record_offset":3788,"ancestor_record_offset":3396,"inlinee":4279,"source_line":408,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":447,"record_offset":3836,"parent_record_offset":3812,"ancestor_record_offset":3396,"inlinee":4280,"source_line":171,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":447,"record_offset":3788,"parent_record_offset":3764,"ancestor_record_offset":3396,"inlinee":4278,"source_line":395,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":447,"record_offset":4784,"parent_record_offset":4752,"ancestor_record_offset":3396,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/00447.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":447,"record_offset":4476,"parent_record_offset":4452,"ancestor_record_offset":3396,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/00447.jsonl.gz"}
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
// 完整结构及原始类型记录: trap_doors.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: trap_doors.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: trap_doors.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::trap_doors::generate_trap_doors
// 信息 ID: bb86268ac0b854386bfdd643
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 6e909eca477b9135f3588367: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x2aee58 -> 0x879f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x30c109 -> 0x879f40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_visual::trap_doors::generate_trap_doors::closure$1
// 信息 ID: 6ef0645c76a2b5d0debd3b92
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::decorator_visual::trap_doors::generate_trap_doors", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_visual::trap_doors::generate_trap_doors; module=447, inline_record=3396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
