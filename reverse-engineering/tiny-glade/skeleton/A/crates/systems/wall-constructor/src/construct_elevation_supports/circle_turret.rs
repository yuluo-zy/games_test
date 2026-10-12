// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/circle_turret.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 3c77bbb2f8a4a053758ea8cd32d9c0b1a2ae570954b245daf43e88cf40f16bac
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: circle_turret.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1220
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1220,"record_offset":3068,"rva":13237504,"va":5381946624}
// 编译实例 RVAs: 0xc9fd00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=4
//     target_aliases=12 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12134689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8dfd646ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5381948236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9199000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e868a273ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5381948691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9200706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8be9b73ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5381950413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14051aa00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7886930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8aea787ff","size":5,"soft_fail":false,"target_va":5374061056,"va":5381947981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b09b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6223795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e84d08a1ff","size":5,"soft_fail":false,"target_va":5375723952,"va":5381947742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6072404","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8ac57a3ff","size":5,"soft_fail":false,"target_va":5375881440,"va":5381953839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5009707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8d58eb3ff","size":5,"soft_fail":false,"target_va":5376939904,"va":5381949606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5006208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8809cb3ff","size":5,"soft_fail":false,"target_va":5376943968,"va":5381950171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5006087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8f99cb3ff","size":5,"soft_fail":false,"target_va":5376944000,"va":5381950082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8d2a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-80721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8afc4feff","size":5,"soft_fail":false,"target_va":5381870240,"va":5381950956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c98240
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8cc7fffff","size":5,"soft_fail":false,"target_va":5381915200,"va":5381948015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-32601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8a780ffff","size":5,"soft_fail":false,"target_va":5381918976,"va":5381951572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8a485ffff","size":5,"soft_fail":false,"target_va":5381919744,"va":5381951063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t83608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e898460100","size":5,"soft_fail":false,"target_va":5382031392,"va":5381947779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21645248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8c0474a01","size":5,"soft_fail":false,"target_va":5403593344,"va":5381948091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21645392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e850484a01","size":5,"soft_fail":false,"target_va":5403593968,"va":5381948571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: circle_turret.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21645808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8f0494a01","size":5,"soft_fail":false,"target_va":5403594240,"va":5381948427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21025725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8bdd34001","size":5,"soft_fail":false,"target_va":5402978144,"va":5381952414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-84627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e86db5feff","size":5,"soft_fail":false,"target_va":5381865744,"va":5381950366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4425309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8a379bcff","size":5,"soft_fail":false,"target_va":5377527120,"va":5381952424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21026082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e822d54001","size":5,"soft_fail":false,"target_va":5402978688,"va":5381952601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t21026632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e848d74001","size":5,"soft_fail":false,"target_va":5402978880,"va":5381952243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t21102239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e89ffe4101","size":5,"soft_fail":false,"target_va":5403054560,"va":5381952316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t21027063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8f7d84001","size":5,"soft_fail":false,"target_va":5402979152,"va":5381952084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t21027548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8dcda4001","size":5,"soft_fail":false,"target_va":5402979344,"va":5381951791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t21026884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e844d84001","size":5,"soft_fail":false,"target_va":5402978944,"va":5381952055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e83de7ffff","size":5,"soft_fail":false,"target_va":5381945344,"va":5381951678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21032930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8e2ef4001","size":5,"soft_fail":false,"target_va":5402980128,"va":5381947193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e820fbffff","size":5,"soft_fail":false,"target_va":5381945536,"va":5381946779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t21033795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e843f34001","size":5,"soft_fail":false,"target_va":5402980656,"va":5381946856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8b3c5ffff","size":5,"soft_fail":false,"target_va":5381932784,"va":5381947704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6512709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8bb9f9cff","size":5,"soft_fail":false,"target_va":5375434976,"va":5381947680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t21020800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e880c04001","size":5,"soft_fail":false,"target_va":5402968992,"va":5381948187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t21019641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8f9bb4001","size":5,"soft_fail":false,"target_va":5402970800,"va":5381951154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5590453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e84bb2aaff","size":5,"soft_fail":false,"target_va":5376356912,"va":5381947360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5586566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e87ac1aaff","size":5,"soft_fail":false,"target_va":5376360560,"va":5381947121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5590621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8a3b1aaff","size":5,"soft_fail":false,"target_va":5376356144,"va":5381946760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h5f74f2ca8f801a08 (.llvm.9829343827527839789)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-81722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8c6c0feff","size":5,"soft_fail":false,"target_va":5381865424,"va":5381947141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2470806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e896b32500","size":5,"soft_fail":false,"target_va":5384420192,"va":5381949381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2471385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8d9b52500","size":5,"soft_fail":false,"target_va":5384420752,"va":5381949362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2471385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8d9b52500","size":5,"soft_fail":false,"target_va":5384420736,"va":5381949346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3979188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e84c48c3ff","size":5,"soft_fail":false,"target_va":5377969360,"va":5381948543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3980662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e88a42c3ff","size":5,"soft_fail":false,"target_va":5377967856,"va":5381948513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3980092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8c444c3ff","size":5,"soft_fail":false,"target_va":5377968464,"va":5381948551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5009865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381946624,"raw_hex":"e8378eb3ff","size":5,"soft_fail":false,"target_va":5376938912,"va":5381948772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5381946624,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5381948122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":6032,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":9128,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":8548,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":6088,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1220,"record_offset":13840,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1220,"record_offset":11816,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1220,"record_offset":7128,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1220,"record_offset":6060,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1220,"record_offset":8088,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1220,"record_offset":6448,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1220,"record_offset":12396,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1220,"record_offset":8896,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1220,"record_offset":6200,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1220,"record_offset":7336,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1220,"record_offset":7616,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":14713,"source_line":158,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1220,"record_offset":12312,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1220,"record_offset":6296,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1220,"record_offset":7160,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1220,"record_offset":11956,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1220,"record_offset":12228,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1220,"record_offset":11704,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1220,"record_offset":11900,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1220,"record_offset":11760,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1220,"record_offset":8980,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1220,"record_offset":9268,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1220,"record_offset":8200,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1220,"record_offset":4964,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1220,"record_offset":3376,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1220,"record_offset":3952,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1220,"record_offset":8520,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1220,"record_offset":8172,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":6118,"source_line":412,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/circle_turret.rs
//     evidence={"module":1220,"record_offset":13940,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":59530,"source_line":166,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/circle_turret.rs
//     evidence={"module":1220,"record_offset":14536,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":59531,"source_line":189,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1220,"record_offset":6484,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1220,"record_offset":9460,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":8756,"parent_record_offset":8732,"ancestor_record_offset":3068,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":8780,"parent_record_offset":8756,"ancestor_record_offset":3068,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":12572,"parent_record_offset":12548,"ancestor_record_offset":3068,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":8732,"parent_record_offset":8632,"ancestor_record_offset":3068,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1220,"record_offset":12548,"parent_record_offset":12448,"ancestor_record_offset":3068,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1220,"record_offset":12480,"parent_record_offset":12448,"ancestor_record_offset":3068,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1220,"record_offset":9204,"parent_record_offset":9172,"ancestor_record_offset":3068,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1220,"record_offset":8632,"parent_record_offset":8604,"ancestor_record_offset":3068,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1220,"record_offset":8604,"parent_record_offset":8576,"ancestor_record_offset":3068,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1220,"record_offset":8852,"parent_record_offset":8604,"ancestor_record_offset":3068,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1220,"record_offset":12448,"parent_record_offset":12420,"ancestor_record_offset":3068,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":9172,"parent_record_offset":9128,"ancestor_record_offset":3068,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":8576,"parent_record_offset":8548,"ancestor_record_offset":3068,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1220,"record_offset":6572,"parent_record_offset":6512,"ancestor_record_offset":3068,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1220,"record_offset":13992,"parent_record_offset":13968,"ancestor_record_offset":3068,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1220,"record_offset":14040,"parent_record_offset":14016,"ancestor_record_offset":3068,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1220,"record_offset":8668,"parent_record_offset":8632,"ancestor_record_offset":3068,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1220,"record_offset":8696,"parent_record_offset":8668,"ancestor_record_offset":3068,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1220,"record_offset":5384,"parent_record_offset":5356,"ancestor_record_offset":3068,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1220,"record_offset":5436,"parent_record_offset":5412,"ancestor_record_offset":3068,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1220,"record_offset":5412,"parent_record_offset":5384,"ancestor_record_offset":3068,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1220,"record_offset":5324,"parent_record_offset":5292,"ancestor_record_offset":3068,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1220,"record_offset":5356,"parent_record_offset":5324,"ancestor_record_offset":3068,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1220,"record_offset":5528,"parent_record_offset":5356,"ancestor_record_offset":3068,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1220,"record_offset":5640,"parent_record_offset":5616,"ancestor_record_offset":3068,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1220,"record_offset":9548,"parent_record_offset":9524,"ancestor_record_offset":3068,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1220,"record_offset":9572,"parent_record_offset":9548,"ancestor_record_offset":3068,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1220,"record_offset":8136,"parent_record_offset":8112,"ancestor_record_offset":3068,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1220,"record_offset":7408,"parent_record_offset":7368,"ancestor_record_offset":3068,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1220,"record_offset":12364,"parent_record_offset":12312,"ancestor_record_offset":3068,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1220,"record_offset":12336,"parent_record_offset":12312,"ancestor_record_offset":3068,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1220,"record_offset":10160,"parent_record_offset":9716,"ancestor_record_offset":3068,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1220,"record_offset":9612,"parent_record_offset":9460,"ancestor_record_offset":3068,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1220,"record_offset":9640,"parent_record_offset":9460,"ancestor_record_offset":3068,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1220,"record_offset":9028,"parent_record_offset":9004,"ancestor_record_offset":3068,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1220,"record_offset":8112,"parent_record_offset":8088,"ancestor_record_offset":3068,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1220,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":3068,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1220,"record_offset":7984,"parent_record_offset":7956,"ancestor_record_offset":3068,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1220,"record_offset":8008,"parent_record_offset":7984,"ancestor_record_offset":3068,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1220,"record_offset":12512,"parent_record_offset":12480,"ancestor_record_offset":3068,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1220,"record_offset":3588,"parent_record_offset":3492,"ancestor_record_offset":3068,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1220,"record_offset":6900,"parent_record_offset":6840,"ancestor_record_offset":3068,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1220,"record_offset":6928,"parent_record_offset":6840,"ancestor_record_offset":3068,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1220,"record_offset":11784,"parent_record_offset":11760,"ancestor_record_offset":3068,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1220,"record_offset":6348,"parent_record_offset":6320,"ancestor_record_offset":3068,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1220,"record_offset":9524,"parent_record_offset":9500,"ancestor_record_offset":3068,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1220,"record_offset":9500,"parent_record_offset":9460,"ancestor_record_offset":3068,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1220,"record_offset":5216,"parent_record_offset":5188,"ancestor_record_offset":3068,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1220,"record_offset":7896,"parent_record_offset":7616,"ancestor_record_offset":3068,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1220,"record_offset":4548,"parent_record_offset":4088,"ancestor_record_offset":3068,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1220,"record_offset":5264,"parent_record_offset":5084,"ancestor_record_offset":3068,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1220,"record_offset":7688,"parent_record_offset":7616,"ancestor_record_offset":3068,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1220,"record_offset":7928,"parent_record_offset":7616,"ancestor_record_offset":3068,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1220,"record_offset":3900,"parent_record_offset":3872,"ancestor_record_offset":3068,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1220,"record_offset":5592,"parent_record_offset":5292,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1220,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":3068,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1220,"record_offset":9416,"parent_record_offset":9392,"ancestor_record_offset":3068,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1220,"record_offset":4324,"parent_record_offset":4300,"ancestor_record_offset":3068,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1220,"record_offset":4300,"parent_record_offset":4276,"ancestor_record_offset":3068,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1220,"record_offset":4508,"parent_record_offset":4476,"ancestor_record_offset":3068,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1220,"record_offset":8808,"parent_record_offset":8780,"ancestor_record_offset":3068,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1220,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":3068,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1220,"record_offset":5992,"parent_record_offset":5044,"ancestor_record_offset":3068,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1220,"record_offset":7808,"parent_record_offset":7724,"ancestor_record_offset":3068,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1220,"record_offset":7832,"parent_record_offset":7808,"ancestor_record_offset":3068,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1220,"record_offset":7492,"parent_record_offset":7464,"ancestor_record_offset":3068,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1220,"record_offset":7520,"parent_record_offset":7492,"ancestor_record_offset":3068,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1220,"record_offset":9812,"parent_record_offset":9788,"ancestor_record_offset":3068,"inlinee":18231,"source_line":2788,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1220,"record_offset":3524,"parent_record_offset":3492,"ancestor_record_offset":3068,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1220,"record_offset":6112,"parent_record_offset":6088,"ancestor_record_offset":3068,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1220,"record_offset":7868,"parent_record_offset":7616,"ancestor_record_offset":3068,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1220,"record_offset":9788,"parent_record_offset":9716,"ancestor_record_offset":3068,"inlinee":21349,"source_line":4188,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1220,"record_offset":9852,"parent_record_offset":9812,"ancestor_record_offset":3068,"inlinee":21351,"source_line":4192,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1220,"record_offset":6600,"parent_record_offset":6572,"ancestor_record_offset":3068,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1220,"record_offset":3552,"parent_record_offset":3524,"ancestor_record_offset":3068,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1220,"record_offset":6628,"parent_record_offset":6600,"ancestor_record_offset":3068,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1220,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":3068,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1220,"record_offset":9392,"parent_record_offset":9328,"ancestor_record_offset":3068,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1220,"record_offset":4276,"parent_record_offset":4252,"ancestor_record_offset":3068,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1220,"record_offset":4604,"parent_record_offset":4580,"ancestor_record_offset":3068,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1220,"record_offset":9052,"parent_record_offset":9028,"ancestor_record_offset":3068,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1220,"record_offset":4252,"parent_record_offset":4144,"ancestor_record_offset":3068,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1220,"record_offset":3492,"parent_record_offset":3464,"ancestor_record_offset":3068,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1220,"record_offset":3984,"parent_record_offset":3952,"ancestor_record_offset":3068,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1220,"record_offset":4116,"parent_record_offset":4088,"ancestor_record_offset":3068,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1220,"record_offset":4088,"parent_record_offset":4056,"ancestor_record_offset":3068,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1220,"record_offset":4032,"parent_record_offset":4008,"ancestor_record_offset":3068,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1220,"record_offset":4056,"parent_record_offset":4032,"ancestor_record_offset":3068,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1220,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":3068,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1220,"record_offset":4580,"parent_record_offset":4056,"ancestor_record_offset":3068,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1220,"record_offset":3688,"parent_record_offset":3656,"ancestor_record_offset":3068,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1220,"record_offset":3848,"parent_record_offset":3688,"ancestor_record_offset":3068,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1220,"record_offset":3656,"parent_record_offset":3632,"ancestor_record_offset":3068,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1220,"record_offset":7436,"parent_record_offset":7408,"ancestor_record_offset":3068,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1220,"record_offset":3408,"parent_record_offset":3376,"ancestor_record_offset":3068,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1220,"record_offset":3872,"parent_record_offset":3848,"ancestor_record_offset":3068,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1220,"record_offset":3464,"parent_record_offset":3432,"ancestor_record_offset":3068,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1220,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":3068,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1220,"record_offset":4144,"parent_record_offset":4116,"ancestor_record_offset":3068,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1220,"record_offset":4392,"parent_record_offset":4144,"ancestor_record_offset":3068,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1220,"record_offset":4364,"parent_record_offset":4144,"ancestor_record_offset":3068,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1220,"record_offset":4196,"parent_record_offset":4144,"ancestor_record_offset":3068,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1220,"record_offset":14340,"parent_record_offset":14308,"ancestor_record_offset":3068,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1220,"record_offset":7752,"parent_record_offset":7724,"ancestor_record_offset":3068,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1220,"record_offset":14364,"parent_record_offset":14340,"ancestor_record_offset":3068,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1220,"record_offset":7776,"parent_record_offset":7752,"ancestor_record_offset":3068,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1220,"record_offset":7368,"parent_record_offset":7336,"ancestor_record_offset":3068,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1220,"record_offset":7956,"parent_record_offset":7928,"ancestor_record_offset":3068,"inlinee":14715,"source_line":175,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1220,"record_offset":14284,"parent_record_offset":14260,"ancestor_record_offset":3068,"inlinee":14717,"source_line":435,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1220,"record_offset":11864,"parent_record_offset":11840,"ancestor_record_offset":3068,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1220,"record_offset":11840,"parent_record_offset":11816,"ancestor_record_offset":3068,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1220,"record_offset":6376,"parent_record_offset":6320,"ancestor_record_offset":3068,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1220,"record_offset":12008,"parent_record_offset":11980,"ancestor_record_offset":3068,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1220,"record_offset":11980,"parent_record_offset":11956,"ancestor_record_offset":3068,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1220,"record_offset":6816,"parent_record_offset":6792,"ancestor_record_offset":3068,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1220,"record_offset":6320,"parent_record_offset":6296,"ancestor_record_offset":3068,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1220,"record_offset":8364,"parent_record_offset":8340,"ancestor_record_offset":3068,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1220,"record_offset":9004,"parent_record_offset":8980,"ancestor_record_offset":3068,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1220,"record_offset":7724,"parent_record_offset":7616,"ancestor_record_offset":3068,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1220,"record_offset":14308,"parent_record_offset":14284,"ancestor_record_offset":3068,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1220,"record_offset":5004,"parent_record_offset":4964,"ancestor_record_offset":3068,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1220,"record_offset":5128,"parent_record_offset":5084,"ancestor_record_offset":3068,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1220,"record_offset":5292,"parent_record_offset":5084,"ancestor_record_offset":3068,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1220,"record_offset":5564,"parent_record_offset":5292,"ancestor_record_offset":3068,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1220,"record_offset":6408,"parent_record_offset":6376,"ancestor_record_offset":3068,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1220,"record_offset":5156,"parent_record_offset":5128,"ancestor_record_offset":3068,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1220,"record_offset":5188,"parent_record_offset":5156,"ancestor_record_offset":3068,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1220,"record_offset":5084,"parent_record_offset":5044,"ancestor_record_offset":3068,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1220,"record_offset":5044,"parent_record_offset":5004,"ancestor_record_offset":3068,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1220,"record_offset":9716,"parent_record_offset":9672,"ancestor_record_offset":3068,"inlinee":21348,"source_line":525,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1220,"record_offset":6512,"parent_record_offset":6484,"ancestor_record_offset":3068,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1220,"record_offset":6544,"parent_record_offset":6512,"ancestor_record_offset":3068,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1220,"record_offset":6668,"parent_record_offset":6512,"ancestor_record_offset":3068,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1220,"record_offset":10008,"parent_record_offset":9716,"ancestor_record_offset":3068,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$4::get_coord_at_u::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1220,"record_offset":9876,"parent_record_offset":9852,"ancestor_record_offset":3068,"inlinee":21353,"source_line":544,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1220,"record_offset":6792,"parent_record_offset":6512,"ancestor_record_offset":3068,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/01220.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE88A
// 首条位置: {"module":1220,"inlinee":59530}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 189
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE88B
// 首条位置: {"module":1220,"inlinee":59531}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret
// 信息 ID: dca17505a2f6a33da541033b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_circle_supports @ 0x4d578a -> 0xc9fd00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret::closure$0
// 信息 ID: 59d43fa98cbd76a8ebec192c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret; module=1220, inline_record=13940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret::closure$1
// 信息 ID: 4112568cd32f0214127461bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret; module=1220, inline_record=14536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
