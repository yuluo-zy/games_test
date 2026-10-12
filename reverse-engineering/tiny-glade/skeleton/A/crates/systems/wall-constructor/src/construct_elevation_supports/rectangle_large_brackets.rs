// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/rectangle_large_brackets.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 96114ae1d0aeb4b7e9f83649faabbe86044fa06dbea5fc54212e4905d537fa46
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: rectangle_large_brackets.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1225
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
// 函数符号: system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1225,"record_offset":3068,"rva":13266608,"va":5381975728}
// 编译实例 RVAs: 0xca6eb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=3
//     target_aliases=12 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12166777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8875946ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5381980324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=2
//     target_aliases=16 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9351781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e89b4d71ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5381976800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9228077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8d33073ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5381977768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9230806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e82a2673ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5381980513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b09b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6255003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8658ea0ff","size":5,"soft_fail":false,"target_va":5375723952,"va":5381978950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5035370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8962ab3ff","size":5,"soft_fail":false,"target_va":5376944000,"va":5381979365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a6f6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2325705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e83783dcff","size":5,"soft_fail":false,"target_va":5379651280,"va":5381976980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8d2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-110077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e80352feff","size":5,"soft_fail":false,"target_va":5381870240,"va":5381980312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-91482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8a69afeff","size":5,"soft_fail":false,"target_va":5381886208,"va":5381977685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c96f30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-66900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8acfafeff","size":5,"soft_fail":false,"target_va":5381910320,"va":5381977215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c98240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-61328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e87010ffff","size":5,"soft_fail":false,"target_va":5381915200,"va":5381976523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-60718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8d212ffff","size":5,"soft_fail":false,"target_va":5381919744,"va":5381980457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb50c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t55506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8d2d80000","size":5,"soft_fail":false,"target_va":5382033600,"va":5381978089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb6a70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t61876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8b4f10000","size":5,"soft_fail":false,"target_va":5382040176,"va":5381978295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21616740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e864d84901","size":5,"soft_fail":false,"target_va":5403593344,"va":5381976599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21615449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e859d34901","size":5,"soft_fail":false,"target_va":5403593360,"va":5381977906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rectangle_large_brackets.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21615498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e88ad34901","size":5,"soft_fail":false,"target_va":5403594144,"va":5381978641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t20996997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e885634001","size":5,"soft_fail":false,"target_va":5402978144,"va":5381981142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10672235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e895275dff","size":5,"soft_fail":false,"target_va":5371305408,"va":5381977638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-4453995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e89509bcff","size":5,"soft_fail":false,"target_va":5377527120,"va":5381981110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20997312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8c0644001","size":5,"soft_fail":false,"target_va":5402978688,"va":5381981371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t20998028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e88c674001","size":5,"soft_fail":false,"target_va":5402978880,"va":5381980847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21073290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e88a8d4101","size":5,"soft_fail":false,"target_va":5403054624,"va":5381981329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t20998158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e80e684001","size":5,"soft_fail":false,"target_va":5402979152,"va":5381980989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20997979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e85b674001","size":5,"soft_fail":false,"target_va":5402978944,"va":5381980960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e86ae7ffff","size":5,"soft_fail":false,"target_va":5381974448,"va":5381980737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21003814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8267e4001","size":5,"soft_fail":false,"target_va":5402980128,"va":5381976309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e817fbffff","size":5,"soft_fail":false,"target_va":5381974640,"va":5381975892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t21004682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e88a814001","size":5,"soft_fail":false,"target_va":5402980656,"va":5381975969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-108140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e89459feff","size":5,"soft_fail":false,"target_va":5381870304,"va":5381978439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-44349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8c352ffff","size":5,"soft_fail":false,"target_va":5381932784,"va":5381977128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6542137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8c72c9cff","size":5,"soft_fail":false,"target_va":5375434976,"va":5381977108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_raycast_w_params
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7470490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e866028eff","size":5,"soft_fail":false,"target_va":5374509168,"va":5381979653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6254017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e83f92a0ff","size":5,"soft_fail":false,"target_va":5375724704,"va":5381978716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t20992309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e835514001","size":5,"soft_fail":false,"target_va":5402968992,"va":5381976678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t20990077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e87d484001","size":5,"soft_fail":false,"target_va":5402970800,"va":5381980718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5619568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e89040aaff","size":5,"soft_fail":false,"target_va":5376356912,"va":5381976475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5615685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8bb4faaff","size":5,"soft_fail":false,"target_va":5376360560,"va":5381976240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5619733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8eb3faaff","size":5,"soft_fail":false,"target_va":5376356144,"va":5381975872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4610559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e801a6b9ff","size":5,"soft_fail":false,"target_va":5377366224,"va":5381976778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h5f74f2ca8f801a08 (.llvm.9829343827527839789)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-110841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8074ffeff","size":5,"soft_fail":false,"target_va":5381865424,"va":5381976260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2440786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8523e2500","size":5,"soft_fail":false,"target_va":5384420000,"va":5381979209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersect_ray
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2441215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8ff3f2500","size":5,"soft_fail":false,"target_va":5384420448,"va":5381979228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::RayBoxIntersection::is_hit
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2442923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8ab462500","size":5,"soft_fail":false,"target_va":5384422192,"va":5381979264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4017961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8d7b0c2ff","size":5,"soft_fail":false,"target_va":5377959584,"va":5381977540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4019098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e866acc2ff","size":5,"soft_fail":false,"target_va":5377958496,"va":5381977589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4021875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e88da1c2ff","size":5,"soft_fail":false,"target_va":5377955632,"va":5381977502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4021731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e81da2c2ff","size":5,"soft_fail":false,"target_va":5377955840,"va":5381977566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4023576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8e89ac2ff","size":5,"soft_fail":false,"target_va":5377955440,"va":5381979011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5038642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8ce1db3ff","size":5,"soft_fail":false,"target_va":5376941312,"va":5381979949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5036053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381975728,"raw_hex":"e8eb27b3ff","size":5,"soft_fail":false,"target_va":5376942944,"va":5381978992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381975728,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5381976619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1225,"record_offset":8248,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1225,"record_offset":9136,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1225,"record_offset":14504,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":17672,"source_line":1658,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1225,"record_offset":17032,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1225,"record_offset":15352,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1225,"record_offset":12268,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1225,"record_offset":9276,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1225,"record_offset":12184,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1225,"record_offset":6304,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1225,"record_offset":17088,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1225,"record_offset":7856,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1225,"record_offset":7968,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1225,"record_offset":6064,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1225,"record_offset":6340,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":14713,"source_line":158,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::inverse
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":1225,"record_offset":14680,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":45796,"source_line":255,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":1225,"record_offset":14888,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1225,"record_offset":15764,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1225,"record_offset":15296,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1225,"record_offset":14624,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1225,"record_offset":15240,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1225,"record_offset":15528,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1225,"record_offset":16856,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1225,"record_offset":15380,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1225,"record_offset":16944,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1225,"record_offset":4996,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1225,"record_offset":3408,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1225,"record_offset":3984,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":1225,"record_offset":8276,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":8702,"source_line":670,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":1225,"record_offset":8092,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":8705,"source_line":889,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":1225,"record_offset":13620,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":22728,"source_line":191,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":1225,"record_offset":6816,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":8743,"source_line":126,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$19::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":1225,"record_offset":14008,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":22730,"source_line":1499,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/rectangle_large_brackets.rs
//     evidence={"module":1225,"record_offset":17060,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":59567,"source_line":71,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1225,"record_offset":16272,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":23062,"source_line":28,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::points_with_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1225,"record_offset":12156,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":23578,"source_line":109,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1225,"record_offset":9416,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":22042,"source_line":46,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1225,"record_offset":9868,"parent_record_offset":9844,"ancestor_record_offset":3068,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1225,"record_offset":9892,"parent_record_offset":9868,"ancestor_record_offset":3068,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1225,"record_offset":10752,"parent_record_offset":10728,"ancestor_record_offset":3068,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1225,"record_offset":9844,"parent_record_offset":9736,"ancestor_record_offset":3068,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1225,"record_offset":10728,"parent_record_offset":10636,"ancestor_record_offset":3068,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1225,"record_offset":10664,"parent_record_offset":10636,"ancestor_record_offset":3068,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1225,"record_offset":9212,"parent_record_offset":9180,"ancestor_record_offset":3068,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1225,"record_offset":9736,"parent_record_offset":9696,"ancestor_record_offset":3068,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1225,"record_offset":9696,"parent_record_offset":9656,"ancestor_record_offset":3068,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1225,"record_offset":9964,"parent_record_offset":9696,"ancestor_record_offset":3068,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1225,"record_offset":10636,"parent_record_offset":10612,"ancestor_record_offset":3068,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1225,"record_offset":9608,"parent_record_offset":9564,"ancestor_record_offset":3068,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1225,"record_offset":9564,"parent_record_offset":9520,"ancestor_record_offset":3068,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1225,"record_offset":9520,"parent_record_offset":9476,"ancestor_record_offset":3068,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1225,"record_offset":9180,"parent_record_offset":9136,"ancestor_record_offset":3068,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1225,"record_offset":9656,"parent_record_offset":9608,"ancestor_record_offset":3068,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1225,"record_offset":9476,"parent_record_offset":9416,"ancestor_record_offset":3068,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1225,"record_offset":12208,"parent_record_offset":12184,"ancestor_record_offset":3068,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1225,"record_offset":17184,"parent_record_offset":17160,"ancestor_record_offset":3068,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1225,"record_offset":17232,"parent_record_offset":17208,"ancestor_record_offset":3068,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1225,"record_offset":13512,"parent_record_offset":13488,"ancestor_record_offset":3068,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1225,"record_offset":9780,"parent_record_offset":9736,"ancestor_record_offset":3068,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1225,"record_offset":9808,"parent_record_offset":9780,"ancestor_record_offset":3068,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1225,"record_offset":5416,"parent_record_offset":5388,"ancestor_record_offset":3068,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1225,"record_offset":7160,"parent_record_offset":7132,"ancestor_record_offset":3068,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1225,"record_offset":5468,"parent_record_offset":5444,"ancestor_record_offset":3068,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1225,"record_offset":5444,"parent_record_offset":5416,"ancestor_record_offset":3068,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1225,"record_offset":5356,"parent_record_offset":5324,"ancestor_record_offset":3068,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1225,"record_offset":5388,"parent_record_offset":5356,"ancestor_record_offset":3068,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1225,"record_offset":5560,"parent_record_offset":5388,"ancestor_record_offset":3068,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1225,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":3068,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1225,"record_offset":14528,"parent_record_offset":14504,"ancestor_record_offset":3068,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1225,"record_offset":13308,"parent_record_offset":13284,"ancestor_record_offset":3068,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1225,"record_offset":13332,"parent_record_offset":13308,"ancestor_record_offset":3068,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1225,"record_offset":9324,"parent_record_offset":9300,"ancestor_record_offset":3068,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1225,"record_offset":6132,"parent_record_offset":6092,"ancestor_record_offset":3068,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1225,"record_offset":13156,"parent_record_offset":12540,"ancestor_record_offset":3068,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1225,"record_offset":13856,"parent_record_offset":13828,"ancestor_record_offset":3068,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1225,"record_offset":8552,"parent_record_offset":8528,"ancestor_record_offset":3068,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1225,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":3068,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1225,"record_offset":14828,"parent_record_offset":14680,"ancestor_record_offset":3068,"inlinee":21870,"source_line":100,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1225,"record_offset":14772,"parent_record_offset":14680,"ancestor_record_offset":3068,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1225,"record_offset":16536,"parent_record_offset":16512,"ancestor_record_offset":3068,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1225,"record_offset":15180,"parent_record_offset":15152,"ancestor_record_offset":3068,"inlinee":21773,"source_line":943,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1225,"record_offset":14716,"parent_record_offset":14680,"ancestor_record_offset":3068,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1225,"record_offset":12804,"parent_record_offset":12776,"ancestor_record_offset":3068,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1225,"record_offset":13400,"parent_record_offset":13220,"ancestor_record_offset":3068,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1225,"record_offset":8576,"parent_record_offset":8552,"ancestor_record_offset":3068,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1225,"record_offset":12336,"parent_record_offset":12304,"ancestor_record_offset":3068,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1225,"record_offset":12304,"parent_record_offset":12268,"ancestor_record_offset":3068,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1225,"record_offset":13488,"parent_record_offset":13464,"ancestor_record_offset":3068,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1225,"record_offset":12432,"parent_record_offset":12400,"ancestor_record_offset":3068,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1225,"record_offset":9300,"parent_record_offset":9276,"ancestor_record_offset":3068,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1225,"record_offset":5492,"parent_record_offset":5468,"ancestor_record_offset":3068,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1225,"record_offset":18176,"parent_record_offset":18148,"ancestor_record_offset":3068,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1225,"record_offset":6712,"parent_record_offset":6684,"ancestor_record_offset":3068,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1225,"record_offset":18200,"parent_record_offset":18176,"ancestor_record_offset":3068,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1225,"record_offset":6736,"parent_record_offset":6712,"ancestor_record_offset":3068,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1225,"record_offset":10692,"parent_record_offset":10664,"ancestor_record_offset":3068,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1225,"record_offset":3620,"parent_record_offset":3524,"ancestor_record_offset":3068,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1225,"record_offset":8748,"parent_record_offset":8724,"ancestor_record_offset":3068,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1225,"record_offset":12688,"parent_record_offset":12636,"ancestor_record_offset":3068,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1225,"record_offset":12744,"parent_record_offset":12636,"ancestor_record_offset":3068,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1225,"record_offset":16972,"parent_record_offset":16944,"ancestor_record_offset":3068,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1225,"record_offset":12660,"parent_record_offset":12636,"ancestor_record_offset":3068,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1225,"record_offset":12400,"parent_record_offset":12336,"ancestor_record_offset":3068,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1225,"record_offset":12504,"parent_record_offset":12468,"ancestor_record_offset":3068,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1225,"record_offset":13284,"parent_record_offset":13260,"ancestor_record_offset":3068,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1225,"record_offset":13260,"parent_record_offset":13220,"ancestor_record_offset":3068,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1225,"record_offset":12372,"parent_record_offset":12336,"ancestor_record_offset":3068,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1225,"record_offset":5248,"parent_record_offset":5220,"ancestor_record_offset":3068,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1225,"record_offset":6620,"parent_record_offset":6340,"ancestor_record_offset":3068,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1225,"record_offset":4580,"parent_record_offset":4120,"ancestor_record_offset":3068,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1225,"record_offset":5296,"parent_record_offset":5116,"ancestor_record_offset":3068,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1225,"record_offset":13464,"parent_record_offset":12304,"ancestor_record_offset":3068,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1225,"record_offset":6408,"parent_record_offset":6340,"ancestor_record_offset":3068,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1225,"record_offset":6652,"parent_record_offset":6340,"ancestor_record_offset":3068,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1225,"record_offset":3932,"parent_record_offset":3904,"ancestor_record_offset":3068,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1225,"record_offset":10008,"parent_record_offset":9608,"ancestor_record_offset":3068,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1225,"record_offset":5624,"parent_record_offset":5324,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1225,"record_offset":7928,"parent_record_offset":7904,"ancestor_record_offset":3068,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1225,"record_offset":14416,"parent_record_offset":14392,"ancestor_record_offset":3068,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1225,"record_offset":14392,"parent_record_offset":14368,"ancestor_record_offset":3068,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1225,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":3068,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1225,"record_offset":4356,"parent_record_offset":4332,"ancestor_record_offset":3068,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1225,"record_offset":4332,"parent_record_offset":4308,"ancestor_record_offset":3068,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1225,"record_offset":4540,"parent_record_offset":4508,"ancestor_record_offset":3068,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1225,"record_offset":9920,"parent_record_offset":9892,"ancestor_record_offset":3068,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1225,"record_offset":5516,"parent_record_offset":5492,"ancestor_record_offset":3068,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1225,"record_offset":6024,"parent_record_offset":5076,"ancestor_record_offset":3068,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1225,"record_offset":6532,"parent_record_offset":6444,"ancestor_record_offset":3068,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1225,"record_offset":6556,"parent_record_offset":6532,"ancestor_record_offset":3068,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1225,"record_offset":6216,"parent_record_offset":6188,"ancestor_record_offset":3068,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1225,"record_offset":6244,"parent_record_offset":6216,"ancestor_record_offset":3068,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1225,"record_offset":3556,"parent_record_offset":3524,"ancestor_record_offset":3068,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1225,"record_offset":7880,"parent_record_offset":7856,"ancestor_record_offset":3068,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1225,"record_offset":6592,"parent_record_offset":6340,"ancestor_record_offset":3068,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1225,"record_offset":3584,"parent_record_offset":3556,"ancestor_record_offset":3068,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1225,"record_offset":7904,"parent_record_offset":7880,"ancestor_record_offset":3068,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1225,"record_offset":8032,"parent_record_offset":7968,"ancestor_record_offset":3068,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1225,"record_offset":4308,"parent_record_offset":4284,"ancestor_record_offset":3068,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1225,"record_offset":4636,"parent_record_offset":4612,"ancestor_record_offset":3068,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1225,"record_offset":4284,"parent_record_offset":4176,"ancestor_record_offset":3068,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1225,"record_offset":3524,"parent_record_offset":3496,"ancestor_record_offset":3068,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1225,"record_offset":4016,"parent_record_offset":3984,"ancestor_record_offset":3068,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1225,"record_offset":4148,"parent_record_offset":4120,"ancestor_record_offset":3068,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1225,"record_offset":4120,"parent_record_offset":4088,"ancestor_record_offset":3068,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1225,"record_offset":4064,"parent_record_offset":4040,"ancestor_record_offset":3068,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1225,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":3068,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1225,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":3068,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1225,"record_offset":4612,"parent_record_offset":4088,"ancestor_record_offset":3068,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1225,"record_offset":3720,"parent_record_offset":3688,"ancestor_record_offset":3068,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1225,"record_offset":3880,"parent_record_offset":3720,"ancestor_record_offset":3068,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1225,"record_offset":3688,"parent_record_offset":3664,"ancestor_record_offset":3068,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1225,"record_offset":6160,"parent_record_offset":6132,"ancestor_record_offset":3068,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1225,"record_offset":3440,"parent_record_offset":3408,"ancestor_record_offset":3068,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1225,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":3068,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1225,"record_offset":3496,"parent_record_offset":3464,"ancestor_record_offset":3068,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1225,"record_offset":3464,"parent_record_offset":3440,"ancestor_record_offset":3068,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1225,"record_offset":4176,"parent_record_offset":4148,"ancestor_record_offset":3068,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1225,"record_offset":4424,"parent_record_offset":4176,"ancestor_record_offset":3068,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1225,"record_offset":4396,"parent_record_offset":4176,"ancestor_record_offset":3068,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1225,"record_offset":4228,"parent_record_offset":4176,"ancestor_record_offset":3068,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1225,"record_offset":18952,"parent_record_offset":18916,"ancestor_record_offset":3068,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1225,"record_offset":6476,"parent_record_offset":6444,"ancestor_record_offset":3068,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1225,"record_offset":18976,"parent_record_offset":18952,"ancestor_record_offset":3068,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1225,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":3068,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1225,"record_offset":6092,"parent_record_offset":6064,"ancestor_record_offset":3068,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1225,"record_offset":6684,"parent_record_offset":6652,"ancestor_record_offset":3068,"inlinee":14715,"source_line":175,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1225,"record_offset":18892,"parent_record_offset":18868,"ancestor_record_offset":3068,"inlinee":14717,"source_line":435,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1225,"record_offset":8936,"parent_record_offset":8912,"ancestor_record_offset":3068,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1225,"record_offset":16300,"parent_record_offset":16272,"ancestor_record_offset":3068,"inlinee":23063,"source_line":231,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1225,"record_offset":16344,"parent_record_offset":16300,"ancestor_record_offset":3068,"inlinee":23064,"source_line":168,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1225,"record_offset":16416,"parent_record_offset":16344,"ancestor_record_offset":3068,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1225,"record_offset":16512,"parent_record_offset":16300,"ancestor_record_offset":3068,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1225,"record_offset":16788,"parent_record_offset":16764,"ancestor_record_offset":3068,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1225,"record_offset":15876,"parent_record_offset":15788,"ancestor_record_offset":3068,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1225,"record_offset":15788,"parent_record_offset":15764,"ancestor_record_offset":3068,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1225,"record_offset":15628,"parent_record_offset":15592,"ancestor_record_offset":3068,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1225,"record_offset":15592,"parent_record_offset":15556,"ancestor_record_offset":3068,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1225,"record_offset":15556,"parent_record_offset":15528,"ancestor_record_offset":3068,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1225,"record_offset":8864,"parent_record_offset":8840,"ancestor_record_offset":3068,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1225,"record_offset":8324,"parent_record_offset":8300,"ancestor_record_offset":3068,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1225,"record_offset":8348,"parent_record_offset":8324,"ancestor_record_offset":3068,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1225,"record_offset":13692,"parent_record_offset":13668,"ancestor_record_offset":3068,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1225,"record_offset":13668,"parent_record_offset":13644,"ancestor_record_offset":3068,"inlinee":11730,"source_line":652,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1225,"record_offset":8840,"parent_record_offset":8816,"ancestor_record_offset":3068,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1225,"record_offset":14080,"parent_record_offset":14056,"ancestor_record_offset":3068,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$71::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1225,"record_offset":14056,"parent_record_offset":14032,"ancestor_record_offset":3068,"inlinee":22734,"source_line":4848,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$8::default
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1225,"record_offset":6888,"parent_record_offset":6864,"ancestor_record_offset":3068,"inlinee":8749,"source_line":2227,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1225,"record_offset":8380,"parent_record_offset":8348,"ancestor_record_offset":3068,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":14368,"parent_record_offset":14132,"ancestor_record_offset":3068,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":9076,"parent_record_offset":8456,"ancestor_record_offset":3068,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":13800,"parent_record_offset":13740,"ancestor_record_offset":3068,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":14132,"parent_record_offset":14104,"ancestor_record_offset":3068,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":8432,"parent_record_offset":8408,"ancestor_record_offset":3068,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":8408,"parent_record_offset":8348,"ancestor_record_offset":3068,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":13716,"parent_record_offset":13692,"ancestor_record_offset":3068,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":18116,"parent_record_offset":18084,"ancestor_record_offset":3068,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":13772,"parent_record_offset":13740,"ancestor_record_offset":3068,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":18028,"parent_record_offset":18004,"ancestor_record_offset":3068,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":3068,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":18084,"parent_record_offset":18028,"ancestor_record_offset":3068,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":18056,"parent_record_offset":18028,"ancestor_record_offset":3068,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":13740,"parent_record_offset":13716,"ancestor_record_offset":3068,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":18148,"parent_record_offset":18116,"ancestor_record_offset":3068,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1225,"record_offset":8696,"parent_record_offset":8668,"ancestor_record_offset":3068,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1225,"record_offset":8724,"parent_record_offset":8696,"ancestor_record_offset":3068,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1225,"record_offset":8784,"parent_record_offset":8668,"ancestor_record_offset":3068,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1225,"record_offset":8668,"parent_record_offset":8456,"ancestor_record_offset":3068,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":8500,"parent_record_offset":8456,"ancestor_record_offset":3068,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":18004,"parent_record_offset":17980,"ancestor_record_offset":3068,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":14104,"parent_record_offset":14080,"ancestor_record_offset":3068,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1225,"record_offset":8816,"parent_record_offset":8456,"ancestor_record_offset":3068,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1225,"record_offset":8528,"parent_record_offset":8456,"ancestor_record_offset":3068,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1225,"record_offset":13828,"parent_record_offset":13800,"ancestor_record_offset":3068,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1225,"record_offset":8612,"parent_record_offset":8456,"ancestor_record_offset":3068,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1225,"record_offset":8984,"parent_record_offset":8456,"ancestor_record_offset":3068,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1225,"record_offset":13884,"parent_record_offset":13828,"ancestor_record_offset":3068,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":1225,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":3068,"inlinee":8703,"source_line":861,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":1225,"record_offset":8116,"parent_record_offset":8092,"ancestor_record_offset":3068,"inlinee":6708,"source_line":1114,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":1225,"record_offset":13644,"parent_record_offset":13620,"ancestor_record_offset":3068,"inlinee":11729,"source_line":291,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$13::default
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":1225,"record_offset":6864,"parent_record_offset":6840,"ancestor_record_offset":3068,"inlinee":8747,"source_line":1402,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$21::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\set.rs
//     evidence={"module":1225,"record_offset":14032,"parent_record_offset":14008,"ancestor_record_offset":3068,"inlinee":22732,"source_line":1692,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1225,"record_offset":12636,"parent_record_offset":12584,"ancestor_record_offset":3068,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1225,"record_offset":6444,"parent_record_offset":6340,"ancestor_record_offset":3068,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1225,"record_offset":18916,"parent_record_offset":18892,"ancestor_record_offset":3068,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1225,"record_offset":5036,"parent_record_offset":4996,"ancestor_record_offset":3068,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1225,"record_offset":5160,"parent_record_offset":5116,"ancestor_record_offset":3068,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1225,"record_offset":5324,"parent_record_offset":5116,"ancestor_record_offset":3068,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1225,"record_offset":5596,"parent_record_offset":5324,"ancestor_record_offset":3068,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\set.rs
//     evidence={"module":1225,"record_offset":6840,"parent_record_offset":6816,"ancestor_record_offset":3068,"inlinee":8745,"source_line":1116,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1225,"record_offset":15664,"parent_record_offset":15628,"ancestor_record_offset":3068,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1225,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":3068,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1225,"record_offset":7012,"parent_record_offset":6984,"ancestor_record_offset":3068,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1225,"record_offset":7132,"parent_record_offset":6984,"ancestor_record_offset":3068,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1225,"record_offset":6912,"parent_record_offset":6888,"ancestor_record_offset":3068,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1225,"record_offset":5188,"parent_record_offset":5160,"ancestor_record_offset":3068,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1225,"record_offset":5220,"parent_record_offset":5188,"ancestor_record_offset":3068,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1225,"record_offset":5116,"parent_record_offset":5076,"ancestor_record_offset":3068,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1225,"record_offset":5076,"parent_record_offset":5036,"ancestor_record_offset":3068,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/rectangle_large_brackets.rs
//     evidence={"module":1225,"record_offset":12540,"parent_record_offset":12504,"ancestor_record_offset":3068,"inlinee":59566,"source_line":134,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1225,"record_offset":13084,"parent_record_offset":13060,"ancestor_record_offset":3068,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1225,"record_offset":13220,"parent_record_offset":13188,"ancestor_record_offset":3068,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1225,"record_offset":13128,"parent_record_offset":12540,"ancestor_record_offset":3068,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1225,"record_offset":12776,"parent_record_offset":12584,"ancestor_record_offset":3068,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1225,"record_offset":13060,"parent_record_offset":13036,"ancestor_record_offset":3068,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1225,"record_offset":12584,"parent_record_offset":12540,"ancestor_record_offset":3068,"inlinee":21574,"source_line":55,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1225,"record_offset":13188,"parent_record_offset":13156,"ancestor_record_offset":3068,"inlinee":26825,"source_line":115,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8AF
// 首条位置: {"module":1225,"inlinee":59567}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8AE
// 首条位置: {"module":1225,"inlinee":59566}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1225,"record_offset":13156,"parent_record_offset":12540,"ancestor_record_offset":12540,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1225,"record_offset":13128,"parent_record_offset":12540,"ancestor_record_offset":12540,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1225,"record_offset":12584,"parent_record_offset":12540,"ancestor_record_offset":12540,"inlinee":21574,"source_line":55,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1225,"record_offset":13308,"parent_record_offset":13284,"ancestor_record_offset":12540,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1225,"record_offset":13332,"parent_record_offset":13308,"ancestor_record_offset":12540,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1225,"record_offset":12804,"parent_record_offset":12776,"ancestor_record_offset":12540,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1225,"record_offset":13400,"parent_record_offset":13220,"ancestor_record_offset":12540,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1225,"record_offset":12688,"parent_record_offset":12636,"ancestor_record_offset":12540,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1225,"record_offset":12744,"parent_record_offset":12636,"ancestor_record_offset":12540,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1225,"record_offset":12660,"parent_record_offset":12636,"ancestor_record_offset":12540,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1225,"record_offset":13284,"parent_record_offset":13260,"ancestor_record_offset":12540,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1225,"record_offset":13260,"parent_record_offset":13220,"ancestor_record_offset":12540,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1225,"record_offset":12636,"parent_record_offset":12584,"ancestor_record_offset":12540,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1225,"record_offset":13084,"parent_record_offset":13060,"ancestor_record_offset":12540,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1225,"record_offset":13220,"parent_record_offset":13188,"ancestor_record_offset":12540,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1225,"record_offset":12776,"parent_record_offset":12584,"ancestor_record_offset":12540,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1225,"record_offset":13060,"parent_record_offset":13036,"ancestor_record_offset":12540,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/01225.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1225,"record_offset":13188,"parent_record_offset":13156,"ancestor_record_offset":12540,"inlinee":26825,"source_line":115,"archive":"_pdb/A/modules/01225.jsonl.gz"}
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
// 完整结构及原始类型记录: rectangle_large_brackets.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: rectangle_large_brackets.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: rectangle_large_brackets.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets
// 信息 ID: 79cc64f9e2375c874f4c7deb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::construct_rectangle_brackets @ 0x4d67f5 -> 0xca6eb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets::closure$0
// 信息 ID: 4b8f4b68ef87fddbe8fe6391
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets; module=1225, inline_record=17060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets::closure$1
// 信息 ID: bf5a18c20be75850a6dd52a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets; module=1225, inline_record=12540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
