// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f2070a8faa9c4a91871b124266f909fdc4cf92c101d633aef221fb9b853e18ee
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: find_overlap_segments.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1206, 1226, 1227, 1262, 1263, 1264
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
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
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1206,"record_offset":6036,"rva":13183168,"va":5381892288}
// 编译实例 RVAs: 0xc928c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9144489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8577774ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5381894180,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9146088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8187174ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5381895795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b09b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6171106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e81ed6a1ff","size":5,"soft_fail":false,"target_va":5375723952,"va":5381895053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=8
//     target_aliases=14 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6018052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8fc2ba4ff","size":5,"soft_fail":false,"target_va":5375881440,"va":5381899487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4954965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8ab64b4ff","size":5,"soft_fail":false,"target_va":5376939904,"va":5381894864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97640
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e89c270000","size":5,"soft_fail":false,"target_va":5381912128,"va":5381901983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabe10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t102201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8398f0100","size":5,"soft_fail":false,"target_va":5381996048,"va":5381893842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140caeee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t114491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e83bbf0100","size":5,"soft_fail":false,"target_va":5382008544,"va":5381894048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb42d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t134765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e86d0e0200","size":5,"soft_fail":false,"target_va":5382030032,"va":5381895262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cebc00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t363334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8468b0500","size":5,"soft_fail":false,"target_va":5382257664,"va":5381894325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cec970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t363637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8758c0500","size":5,"soft_fail":false,"target_va":5382261104,"va":5381897462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21697932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e88c154b01","size":5,"soft_fail":false,"target_va":5403593360,"va":5381895423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21700029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8bd1d4b01","size":5,"soft_fail":false,"target_va":5403594144,"va":5381894110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t21079049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e809a44101","size":5,"soft_fail":false,"target_va":5402978144,"va":5381899090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t-10588403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e80d6f5eff","size":5,"soft_fail":false,"target_va":5371304880,"va":5381893278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4371982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8f249bdff","size":5,"soft_fail":false,"target_va":5377527120,"va":5381899097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21079536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8f0a54101","size":5,"soft_fail":false,"target_va":5402978688,"va":5381899147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t21080148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e854a84101","size":5,"soft_fail":false,"target_va":5402978880,"va":5381898727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21155532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8ccce4201","size":5,"soft_fail":false,"target_va":5403054560,"va":5381899023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t21080391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e847a94101","size":5,"soft_fail":false,"target_va":5402979152,"va":5381898756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t21080666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e85aaa4101","size":5,"soft_fail":false,"target_va":5402979344,"va":5381898673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21080073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e809a84101","size":5,"soft_fail":false,"target_va":5402978944,"va":5381898866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-8309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e88bdfffff","size":5,"soft_fail":false,"target_va":5381890256,"va":5381898560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21087251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e813c44101","size":5,"soft_fail":false,"target_va":5402980128,"va":5381892872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e828f8ffff","size":5,"soft_fail":false,"target_va":5381890448,"va":5381892451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t21088123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e87bc74101","size":5,"soft_fail":false,"target_va":5402980656,"va":5381892528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::gable_sides
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6132427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8356da2ff","size":5,"soft_fail":false,"target_va":5375763616,"va":5381896038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6131311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e89171a2ff","size":5,"soft_fail":false,"target_va":5375764720,"va":5381896026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t36533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8b58e0000","size":5,"soft_fail":false,"target_va":5381932784,"va":5381896246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t39244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e84c990000","size":5,"soft_fail":false,"target_va":5381932864,"va":5381893615,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6458602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e816739dff","size":5,"soft_fail":false,"target_va":5375434976,"va":5381893573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::utils_intersect::line_x_shape_overlap
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3870962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8f2103b00","size":5,"soft_fail":false,"target_va":5385767248,"va":5381896281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5536131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e87d86abff","size":5,"soft_fail":false,"target_va":5376356912,"va":5381893038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5532245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8ab95abff","size":5,"soft_fail":false,"target_va":5376360560,"va":5381892800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5536289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8df85abff","size":5,"soft_fail":false,"target_va":5376356144,"va":5381892428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h5f74f2ca8f801a08 (.llvm.9829343827527839789)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-27401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8f794ffff","size":5,"soft_fail":false,"target_va":5381865424,"va":5381892820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve2
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3924431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8311ec4ff","size":5,"soft_fail":false,"target_va":5377969360,"va":5381893786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::contains_point
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3927299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8fd12c4ff","size":5,"soft_fail":false,"target_va":5377970048,"va":5381897342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3929849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e80709c4ff","size":5,"soft_fail":false,"target_va":5377968464,"va":5381898308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve2
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-3939686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e89ae2c3ff","size":5,"soft_fail":false,"target_va":5377954064,"va":5381893745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-3937639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e899eac3ff","size":5,"soft_fail":false,"target_va":5377959584,"va":5381897218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-3940523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e855dfc3ff","size":5,"soft_fail":false,"target_va":5377955632,"va":5381896150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-3938132","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8ace8c3ff","size":5,"soft_fail":false,"target_va":5377955840,"va":5381893967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-3940891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e8e5ddc3ff","size":5,"soft_fail":false,"target_va":5377955424,"va":5381896310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3938149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381892288,"raw_hex":"e89be8c3ff","size":5,"soft_fail":false,"target_va":5377960080,"va":5381898224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1206,"record_offset":11200,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":6488,"source_line":263,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1206,"record_offset":9008,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1206,"record_offset":10096,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1206,"record_offset":10440,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1206,"record_offset":17164,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1206,"record_offset":10496,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1206,"record_offset":17428,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1206,"record_offset":14380,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1206,"record_offset":11084,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":7622,"source_line":1248,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1206,"record_offset":18804,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1206,"record_offset":19056,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1206,"record_offset":10872,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::enumerate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1206,"record_offset":18972,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":16076,"source_line":976,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1206,"record_offset":10676,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1206,"record_offset":10612,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1206,"record_offset":16184,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1206,"record_offset":10788,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1206,"record_offset":17276,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1206,"record_offset":10040,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1206,"record_offset":10648,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1206,"record_offset":20376,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::circular_tuple_windows
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1206,"record_offset":12820,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":45815,"source_line":734,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$8::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1206,"record_offset":13452,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":45817,"source_line":234,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1206,"record_offset":7960,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1206,"record_offset":6372,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1206,"record_offset":6948,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1206,"record_offset":9036,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1206,"record_offset":9396,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1206,"record_offset":20348,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1206,"record_offset":14660,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1206,"record_offset":15800,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1206,"record_offset":15828,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":24269,"source_line":41,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::distance
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1206,"record_offset":15772,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":59439,"source_line":65,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1206,"record_offset":11256,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::subtract
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1206,"record_offset":15920,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":24314,"source_line":48,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1206,"record_offset":13900,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":21574,"source_line":55,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1206,"record_offset":11380,"parent_record_offset":6036,"ancestor_record_offset":6036,"inlinee":22042,"source_line":46,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1206,"record_offset":11808,"parent_record_offset":11784,"ancestor_record_offset":6036,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1206,"record_offset":11832,"parent_record_offset":11808,"ancestor_record_offset":6036,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1206,"record_offset":12688,"parent_record_offset":12664,"ancestor_record_offset":6036,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1206,"record_offset":11784,"parent_record_offset":11676,"ancestor_record_offset":6036,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1206,"record_offset":12664,"parent_record_offset":12572,"ancestor_record_offset":6036,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1206,"record_offset":12600,"parent_record_offset":12572,"ancestor_record_offset":6036,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1206,"record_offset":10160,"parent_record_offset":10128,"ancestor_record_offset":6036,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1206,"record_offset":11676,"parent_record_offset":11640,"ancestor_record_offset":6036,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1206,"record_offset":11640,"parent_record_offset":11604,"ancestor_record_offset":6036,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1206,"record_offset":11904,"parent_record_offset":11640,"ancestor_record_offset":6036,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1206,"record_offset":12572,"parent_record_offset":12548,"ancestor_record_offset":6036,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1206,"record_offset":11560,"parent_record_offset":11520,"ancestor_record_offset":6036,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1206,"record_offset":11520,"parent_record_offset":11480,"ancestor_record_offset":6036,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1206,"record_offset":11480,"parent_record_offset":11440,"ancestor_record_offset":6036,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1206,"record_offset":11224,"parent_record_offset":11200,"ancestor_record_offset":6036,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1206,"record_offset":10128,"parent_record_offset":10096,"ancestor_record_offset":6036,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1206,"record_offset":11604,"parent_record_offset":11560,"ancestor_record_offset":6036,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1206,"record_offset":11440,"parent_record_offset":11380,"ancestor_record_offset":6036,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1206,"record_offset":15016,"parent_record_offset":14696,"ancestor_record_offset":6036,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1206,"record_offset":10896,"parent_record_offset":10872,"ancestor_record_offset":6036,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1206,"record_offset":16896,"parent_record_offset":16872,"ancestor_record_offset":6036,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1206,"record_offset":16944,"parent_record_offset":16920,"ancestor_record_offset":6036,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1206,"record_offset":11720,"parent_record_offset":11676,"ancestor_record_offset":6036,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1206,"record_offset":11748,"parent_record_offset":11720,"ancestor_record_offset":6036,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1206,"record_offset":17516,"parent_record_offset":17452,"ancestor_record_offset":6036,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1206,"record_offset":8360,"parent_record_offset":8332,"ancestor_record_offset":6036,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1206,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":6036,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1206,"record_offset":8388,"parent_record_offset":8360,"ancestor_record_offset":6036,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1206,"record_offset":8300,"parent_record_offset":8272,"ancestor_record_offset":6036,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1206,"record_offset":8332,"parent_record_offset":8300,"ancestor_record_offset":6036,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1206,"record_offset":8504,"parent_record_offset":8332,"ancestor_record_offset":6036,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1206,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":6036,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1206,"record_offset":10724,"parent_record_offset":10700,"ancestor_record_offset":6036,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1206,"record_offset":10748,"parent_record_offset":10724,"ancestor_record_offset":6036,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1206,"record_offset":11164,"parent_record_offset":11108,"ancestor_record_offset":6036,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1206,"record_offset":9248,"parent_record_offset":9220,"ancestor_record_offset":6036,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1206,"record_offset":9308,"parent_record_offset":9276,"ancestor_record_offset":6036,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1206,"record_offset":15140,"parent_record_offset":14696,"ancestor_record_offset":6036,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1206,"record_offset":11292,"parent_record_offset":11256,"ancestor_record_offset":6036,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1206,"record_offset":14620,"parent_record_offset":14444,"ancestor_record_offset":6036,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1206,"record_offset":11968,"parent_record_offset":11944,"ancestor_record_offset":6036,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1206,"record_offset":13260,"parent_record_offset":13236,"ancestor_record_offset":6036,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cycle::Cycle::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\cycle.rs
//     evidence={"module":1206,"record_offset":12896,"parent_record_offset":12872,"ancestor_record_offset":6036,"inlinee":51353,"source_line":20,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::cycle::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\cycle.rs
//     evidence={"module":1206,"record_offset":12964,"parent_record_offset":12928,"ancestor_record_offset":6036,"inlinee":45819,"source_line":33,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::Enumerate::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1206,"record_offset":18996,"parent_record_offset":18972,"ancestor_record_offset":6036,"inlinee":16078,"source_line":23,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1206,"record_offset":19208,"parent_record_offset":19176,"ancestor_record_offset":6036,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1206,"record_offset":12992,"parent_record_offset":12964,"ancestor_record_offset":6036,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\take.rs
//     evidence={"module":1206,"record_offset":13476,"parent_record_offset":13452,"ancestor_record_offset":6036,"inlinee":7708,"source_line":38,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1206,"record_offset":11108,"parent_record_offset":11084,"ancestor_record_offset":6036,"inlinee":7624,"source_line":1153,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1206,"record_offset":18828,"parent_record_offset":18804,"ancestor_record_offset":6036,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1206,"record_offset":19256,"parent_record_offset":19232,"ancestor_record_offset":6036,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1206,"record_offset":10920,"parent_record_offset":10896,"ancestor_record_offset":6036,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1206,"record_offset":13236,"parent_record_offset":12928,"ancestor_record_offset":6036,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::cycle
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1206,"record_offset":12872,"parent_record_offset":12844,"ancestor_record_offset":6036,"inlinee":59438,"source_line":3497,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1206,"record_offset":17540,"parent_record_offset":17516,"ancestor_record_offset":6036,"inlinee":6097,"source_line":697,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1206,"record_offset":8436,"parent_record_offset":8412,"ancestor_record_offset":6036,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1206,"record_offset":14896,"parent_record_offset":14696,"ancestor_record_offset":6036,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1206,"record_offset":14920,"parent_record_offset":14896,"ancestor_record_offset":6036,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1206,"record_offset":12628,"parent_record_offset":12600,"ancestor_record_offset":6036,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1206,"record_offset":6584,"parent_record_offset":6488,"ancestor_record_offset":6036,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1206,"record_offset":9628,"parent_record_offset":9604,"ancestor_record_offset":6036,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1206,"record_offset":13968,"parent_record_offset":13944,"ancestor_record_offset":6036,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1206,"record_offset":14024,"parent_record_offset":13944,"ancestor_record_offset":6036,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1206,"record_offset":15584,"parent_record_offset":15544,"ancestor_record_offset":6036,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1206,"record_offset":13196,"parent_record_offset":13108,"ancestor_record_offset":6036,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1206,"record_offset":17488,"parent_record_offset":17452,"ancestor_record_offset":6036,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1206,"record_offset":17452,"parent_record_offset":17428,"ancestor_record_offset":6036,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1206,"record_offset":10700,"parent_record_offset":10676,"ancestor_record_offset":6036,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1206,"record_offset":14480,"parent_record_offset":14444,"ancestor_record_offset":6036,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1206,"record_offset":11136,"parent_record_offset":11108,"ancestor_record_offset":6036,"inlinee":7625,"source_line":545,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1206,"record_offset":8196,"parent_record_offset":8168,"ancestor_record_offset":6036,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1206,"record_offset":7544,"parent_record_offset":7084,"ancestor_record_offset":6036,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1206,"record_offset":8244,"parent_record_offset":8068,"ancestor_record_offset":6036,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1206,"record_offset":14952,"parent_record_offset":14696,"ancestor_record_offset":6036,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1206,"record_offset":6896,"parent_record_offset":6868,"ancestor_record_offset":6036,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1206,"record_offset":11944,"parent_record_offset":11560,"ancestor_record_offset":6036,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1206,"record_offset":17564,"parent_record_offset":17540,"ancestor_record_offset":6036,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1206,"record_offset":8568,"parent_record_offset":8272,"ancestor_record_offset":6036,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1206,"record_offset":10464,"parent_record_offset":10440,"ancestor_record_offset":6036,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1206,"record_offset":9876,"parent_record_offset":9852,"ancestor_record_offset":6036,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1206,"record_offset":9852,"parent_record_offset":9828,"ancestor_record_offset":6036,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1206,"record_offset":13164,"parent_record_offset":13140,"ancestor_record_offset":6036,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1206,"record_offset":7320,"parent_record_offset":7296,"ancestor_record_offset":6036,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1206,"record_offset":10552,"parent_record_offset":10496,"ancestor_record_offset":6036,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1206,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":6036,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1206,"record_offset":7504,"parent_record_offset":7472,"ancestor_record_offset":6036,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1206,"record_offset":10576,"parent_record_offset":10552,"ancestor_record_offset":6036,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1206,"record_offset":11860,"parent_record_offset":11832,"ancestor_record_offset":6036,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1206,"record_offset":8460,"parent_record_offset":8436,"ancestor_record_offset":6036,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1206,"record_offset":8968,"parent_record_offset":8032,"ancestor_record_offset":6036,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1206,"record_offset":14792,"parent_record_offset":14768,"ancestor_record_offset":6036,"inlinee":18231,"source_line":2788,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1206,"record_offset":6520,"parent_record_offset":6488,"ancestor_record_offset":6036,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1206,"record_offset":17188,"parent_record_offset":17164,"ancestor_record_offset":6036,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1206,"record_offset":14768,"parent_record_offset":14696,"ancestor_record_offset":6036,"inlinee":21349,"source_line":4188,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1206,"record_offset":14832,"parent_record_offset":14792,"ancestor_record_offset":6036,"inlinee":21351,"source_line":4192,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1206,"record_offset":15044,"parent_record_offset":15016,"ancestor_record_offset":6036,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1206,"record_offset":6548,"parent_record_offset":6520,"ancestor_record_offset":6036,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1206,"record_offset":15072,"parent_record_offset":15044,"ancestor_record_offset":6036,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1206,"record_offset":10812,"parent_record_offset":10788,"ancestor_record_offset":6036,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1206,"record_offset":13140,"parent_record_offset":13108,"ancestor_record_offset":6036,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1206,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":6036,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1206,"record_offset":7600,"parent_record_offset":7576,"ancestor_record_offset":6036,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1206,"record_offset":13016,"parent_record_offset":12992,"ancestor_record_offset":6036,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1206,"record_offset":7248,"parent_record_offset":7140,"ancestor_record_offset":6036,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1206,"record_offset":6488,"parent_record_offset":6460,"ancestor_record_offset":6036,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1206,"record_offset":6980,"parent_record_offset":6948,"ancestor_record_offset":6036,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1206,"record_offset":7112,"parent_record_offset":7084,"ancestor_record_offset":6036,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1206,"record_offset":7084,"parent_record_offset":7052,"ancestor_record_offset":6036,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1206,"record_offset":7028,"parent_record_offset":7004,"ancestor_record_offset":6036,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1206,"record_offset":7052,"parent_record_offset":7028,"ancestor_record_offset":6036,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1206,"record_offset":7004,"parent_record_offset":6980,"ancestor_record_offset":6036,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1206,"record_offset":7576,"parent_record_offset":7052,"ancestor_record_offset":6036,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1206,"record_offset":6684,"parent_record_offset":6652,"ancestor_record_offset":6036,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1206,"record_offset":6844,"parent_record_offset":6684,"ancestor_record_offset":6036,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1206,"record_offset":6652,"parent_record_offset":6628,"ancestor_record_offset":6036,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1206,"record_offset":6404,"parent_record_offset":6372,"ancestor_record_offset":6036,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1206,"record_offset":6868,"parent_record_offset":6844,"ancestor_record_offset":6036,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1206,"record_offset":6460,"parent_record_offset":6428,"ancestor_record_offset":6036,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1206,"record_offset":6428,"parent_record_offset":6404,"ancestor_record_offset":6036,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1206,"record_offset":7140,"parent_record_offset":7112,"ancestor_record_offset":6036,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1206,"record_offset":7388,"parent_record_offset":7140,"ancestor_record_offset":6036,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1206,"record_offset":7360,"parent_record_offset":7140,"ancestor_record_offset":6036,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1206,"record_offset":7192,"parent_record_offset":7140,"ancestor_record_offset":6036,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1206,"record_offset":20456,"parent_record_offset":20400,"ancestor_record_offset":6036,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1206,"record_offset":15520,"parent_record_offset":15496,"ancestor_record_offset":6036,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1206,"record_offset":15544,"parent_record_offset":15520,"ancestor_record_offset":6036,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1206,"record_offset":9084,"parent_record_offset":9060,"ancestor_record_offset":6036,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1206,"record_offset":9424,"parent_record_offset":9396,"ancestor_record_offset":6036,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1206,"record_offset":9828,"parent_record_offset":9488,"ancestor_record_offset":6036,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1206,"record_offset":9192,"parent_record_offset":9132,"ancestor_record_offset":6036,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1206,"record_offset":9488,"parent_record_offset":9452,"ancestor_record_offset":6036,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1206,"record_offset":9108,"parent_record_offset":9084,"ancestor_record_offset":6036,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1206,"record_offset":9164,"parent_record_offset":9132,"ancestor_record_offset":6036,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1206,"record_offset":9132,"parent_record_offset":9108,"ancestor_record_offset":6036,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1206,"record_offset":9572,"parent_record_offset":9536,"ancestor_record_offset":6036,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1206,"record_offset":9604,"parent_record_offset":9572,"ancestor_record_offset":6036,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1206,"record_offset":9664,"parent_record_offset":9536,"ancestor_record_offset":6036,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1206,"record_offset":9536,"parent_record_offset":9488,"ancestor_record_offset":6036,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1206,"record_offset":9452,"parent_record_offset":9424,"ancestor_record_offset":6036,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1206,"record_offset":9220,"parent_record_offset":9192,"ancestor_record_offset":6036,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1206,"record_offset":9276,"parent_record_offset":9220,"ancestor_record_offset":6036,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::circular_tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1206,"record_offset":12844,"parent_record_offset":12820,"ancestor_record_offset":6036,"inlinee":59437,"source_line":213,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$26::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1206,"record_offset":13804,"parent_record_offset":13504,"ancestor_record_offset":6036,"inlinee":8082,"source_line":318,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1206,"record_offset":13504,"parent_record_offset":13476,"ancestor_record_offset":6036,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1206,"record_offset":12928,"parent_record_offset":12844,"ancestor_record_offset":6036,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1206,"record_offset":13944,"parent_record_offset":13900,"ancestor_record_offset":6036,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1206,"record_offset":7996,"parent_record_offset":7960,"ancestor_record_offset":6036,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1206,"record_offset":8108,"parent_record_offset":8068,"ancestor_record_offset":6036,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1206,"record_offset":8272,"parent_record_offset":8068,"ancestor_record_offset":6036,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1206,"record_offset":8540,"parent_record_offset":8272,"ancestor_record_offset":6036,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1206,"record_offset":9060,"parent_record_offset":9036,"ancestor_record_offset":6036,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1206,"record_offset":20488,"parent_record_offset":20456,"ancestor_record_offset":6036,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1206,"record_offset":8136,"parent_record_offset":8108,"ancestor_record_offset":6036,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1206,"record_offset":8168,"parent_record_offset":8136,"ancestor_record_offset":6036,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1206,"record_offset":8068,"parent_record_offset":8032,"ancestor_record_offset":6036,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1206,"record_offset":8032,"parent_record_offset":7996,"ancestor_record_offset":6036,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$3
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs
//     evidence={"module":1206,"record_offset":19388,"parent_record_offset":19364,"ancestor_record_offset":6036,"inlinee":59441,"source_line":311,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1206,"record_offset":11344,"parent_record_offset":11320,"ancestor_record_offset":6036,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1206,"record_offset":14696,"parent_record_offset":14660,"ancestor_record_offset":6036,"inlinee":21348,"source_line":525,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1206,"record_offset":15176,"parent_record_offset":14660,"ancestor_record_offset":6036,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1206,"record_offset":15216,"parent_record_offset":15176,"ancestor_record_offset":6036,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1206,"record_offset":15372,"parent_record_offset":15176,"ancestor_record_offset":6036,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1206,"record_offset":14444,"parent_record_offset":14412,"ancestor_record_offset":6036,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1206,"record_offset":14988,"parent_record_offset":14696,"ancestor_record_offset":6036,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1206,"record_offset":15944,"parent_record_offset":15920,"ancestor_record_offset":6036,"inlinee":24269,"source_line":41,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1206,"record_offset":14056,"parent_record_offset":13900,"ancestor_record_offset":6036,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1206,"record_offset":11320,"parent_record_offset":11256,"ancestor_record_offset":6036,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::impl$3::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1206,"record_offset":14412,"parent_record_offset":14380,"ancestor_record_offset":6036,"inlinee":26825,"source_line":115,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$4::get_coord_at_u::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1206,"record_offset":14856,"parent_record_offset":14832,"ancestor_record_offset":6036,"inlinee":21353,"source_line":544,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1206,"record_offset":15496,"parent_record_offset":15176,"ancestor_record_offset":6036,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1206,"record_offset":3068,"rva":13182416,"va":5381891536}
// 编译实例 RVAs: 0xc925d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: find_overlap_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9142517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381891536,"raw_hex":"e80b7f74ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5381892224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10587146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381891536,"raw_hex":"e8f6735eff","size":5,"soft_fail":false,"target_va":5371304880,"va":5381892021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6128435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381891536,"raw_hex":"e8cd7ca2ff","size":5,"soft_fail":false,"target_va":5375763568,"va":5381891998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6455775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381891536,"raw_hex":"e8217e9dff","size":5,"soft_fail":false,"target_va":5375436032,"va":5381891802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1206,"record_offset":3288,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1206,"record_offset":4292,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1206,"record_offset":4428,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1206,"record_offset":5592,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1206,"record_offset":5564,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1206,"record_offset":3344,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1206,"record_offset":3684,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1206,"record_offset":5768,"parent_record_offset":5744,"ancestor_record_offset":3068,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1206,"record_offset":5744,"parent_record_offset":5644,"ancestor_record_offset":3068,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1206,"record_offset":5676,"parent_record_offset":5644,"ancestor_record_offset":3068,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1206,"record_offset":4364,"parent_record_offset":4332,"ancestor_record_offset":3068,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1206,"record_offset":5644,"parent_record_offset":5616,"ancestor_record_offset":3068,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1206,"record_offset":4332,"parent_record_offset":4292,"ancestor_record_offset":3068,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":1206,"record_offset":4692,"parent_record_offset":4548,"ancestor_record_offset":3068,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":1206,"record_offset":4548,"parent_record_offset":4504,"ancestor_record_offset":3068,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1206,"record_offset":5336,"parent_record_offset":4504,"ancestor_record_offset":3068,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1206,"record_offset":4968,"parent_record_offset":4504,"ancestor_record_offset":3068,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1206,"record_offset":4992,"parent_record_offset":4968,"ancestor_record_offset":3068,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":1206,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":3068,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":1206,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":3068,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1206,"record_offset":5068,"parent_record_offset":5044,"ancestor_record_offset":3068,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1206,"record_offset":5044,"parent_record_offset":5020,"ancestor_record_offset":3068,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1206,"record_offset":5116,"parent_record_offset":5092,"ancestor_record_offset":3068,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1206,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":3068,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1206,"record_offset":5272,"parent_record_offset":4992,"ancestor_record_offset":3068,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1206,"record_offset":5020,"parent_record_offset":4992,"ancestor_record_offset":3068,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1206,"record_offset":4940,"parent_record_offset":4504,"ancestor_record_offset":3068,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":1206,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":3068,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":1206,"record_offset":5360,"parent_record_offset":5336,"ancestor_record_offset":3068,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":1206,"record_offset":4756,"parent_record_offset":4724,"ancestor_record_offset":3068,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1206,"record_offset":4660,"parent_record_offset":4636,"ancestor_record_offset":3068,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1206,"record_offset":3568,"parent_record_offset":3540,"ancestor_record_offset":3068,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1206,"record_offset":3628,"parent_record_offset":3596,"ancestor_record_offset":3068,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1206,"record_offset":5708,"parent_record_offset":5676,"ancestor_record_offset":3068,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1206,"record_offset":3908,"parent_record_offset":3884,"ancestor_record_offset":3068,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1206,"record_offset":4636,"parent_record_offset":4548,"ancestor_record_offset":3068,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1206,"record_offset":5140,"parent_record_offset":5116,"ancestor_record_offset":3068,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1206,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":3068,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1206,"record_offset":3512,"parent_record_offset":3488,"ancestor_record_offset":3068,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1206,"record_offset":5616,"parent_record_offset":5592,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1206,"record_offset":4864,"parent_record_offset":4832,"ancestor_record_offset":3068,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1206,"record_offset":4128,"parent_record_offset":4104,"ancestor_record_offset":3068,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1206,"record_offset":4104,"parent_record_offset":4080,"ancestor_record_offset":3068,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1206,"record_offset":4396,"parent_record_offset":4292,"ancestor_record_offset":3068,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1206,"record_offset":4912,"parent_record_offset":4504,"ancestor_record_offset":3068,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1206,"record_offset":4580,"parent_record_offset":4548,"ancestor_record_offset":3068,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1206,"record_offset":4604,"parent_record_offset":4580,"ancestor_record_offset":3068,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1206,"record_offset":4724,"parent_record_offset":4504,"ancestor_record_offset":3068,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1206,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":3068,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1206,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":3068,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1206,"record_offset":4784,"parent_record_offset":4756,"ancestor_record_offset":3068,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1206,"record_offset":3372,"parent_record_offset":3344,"ancestor_record_offset":3068,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1206,"record_offset":3712,"parent_record_offset":3684,"ancestor_record_offset":3068,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1206,"record_offset":4080,"parent_record_offset":3772,"ancestor_record_offset":3068,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1206,"record_offset":3488,"parent_record_offset":3428,"ancestor_record_offset":3068,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1206,"record_offset":3772,"parent_record_offset":3740,"ancestor_record_offset":3068,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1206,"record_offset":3400,"parent_record_offset":3372,"ancestor_record_offset":3068,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1206,"record_offset":3460,"parent_record_offset":3428,"ancestor_record_offset":3068,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1206,"record_offset":3428,"parent_record_offset":3400,"ancestor_record_offset":3068,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1206,"record_offset":3852,"parent_record_offset":3820,"ancestor_record_offset":3068,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1206,"record_offset":3884,"parent_record_offset":3852,"ancestor_record_offset":3068,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1206,"record_offset":3820,"parent_record_offset":3772,"ancestor_record_offset":3068,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1206,"record_offset":3740,"parent_record_offset":3712,"ancestor_record_offset":3068,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1206,"record_offset":3540,"parent_record_offset":3488,"ancestor_record_offset":3068,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1206,"record_offset":3596,"parent_record_offset":3540,"ancestor_record_offset":3068,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1206,"record_offset":5228,"parent_record_offset":5204,"ancestor_record_offset":3068,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_elevation_supports::find_overlap_segments::impl$0::new::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_elevation_supports/find_overlap_segments.rs
//     evidence={"module":1206,"record_offset":4456,"parent_record_offset":4428,"ancestor_record_offset":3068,"inlinee":59436,"source_line":43,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::impl$0::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE82C
// 首条位置: {"module":1206,"inlinee":59436}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":1206,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":4456,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":1206,"record_offset":4692,"parent_record_offset":4548,"ancestor_record_offset":4456,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":1206,"record_offset":4548,"parent_record_offset":4504,"ancestor_record_offset":4456,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1206,"record_offset":5336,"parent_record_offset":4504,"ancestor_record_offset":4456,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1206,"record_offset":4968,"parent_record_offset":4504,"ancestor_record_offset":4456,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":1206,"record_offset":4992,"parent_record_offset":4968,"ancestor_record_offset":4456,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":1206,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":4456,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":1206,"record_offset":5296,"parent_record_offset":5272,"ancestor_record_offset":4456,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1206,"record_offset":5068,"parent_record_offset":5044,"ancestor_record_offset":4456,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1206,"record_offset":5044,"parent_record_offset":5020,"ancestor_record_offset":4456,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1206,"record_offset":5116,"parent_record_offset":5092,"ancestor_record_offset":4456,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":1206,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":4456,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1206,"record_offset":5272,"parent_record_offset":4992,"ancestor_record_offset":4456,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1206,"record_offset":5020,"parent_record_offset":4992,"ancestor_record_offset":4456,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":1206,"record_offset":4940,"parent_record_offset":4504,"ancestor_record_offset":4456,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":1206,"record_offset":5360,"parent_record_offset":5336,"ancestor_record_offset":4456,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":1206,"record_offset":4756,"parent_record_offset":4724,"ancestor_record_offset":4456,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1206,"record_offset":4660,"parent_record_offset":4636,"ancestor_record_offset":4456,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1206,"record_offset":4636,"parent_record_offset":4548,"ancestor_record_offset":4456,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1206,"record_offset":5140,"parent_record_offset":5116,"ancestor_record_offset":4456,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1206,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":4456,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1206,"record_offset":4864,"parent_record_offset":4832,"ancestor_record_offset":4456,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1206,"record_offset":4912,"parent_record_offset":4504,"ancestor_record_offset":4456,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1206,"record_offset":4580,"parent_record_offset":4548,"ancestor_record_offset":4456,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1206,"record_offset":4604,"parent_record_offset":4580,"ancestor_record_offset":4456,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1206,"record_offset":4724,"parent_record_offset":4504,"ancestor_record_offset":4456,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1206,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":4456,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1206,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":4456,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1206,"record_offset":4784,"parent_record_offset":4756,"ancestor_record_offset":4456,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1206,"record_offset":5228,"parent_record_offset":5204,"ancestor_record_offset":4456,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/01206.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8E5
// 首条位置: {"module":1226,"inlinee":59621}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1226,"record_offset":42128,"parent_record_offset":42104,"ancestor_record_offset":42104,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1226,"record_offset":42152,"parent_record_offset":42128,"ancestor_record_offset":42104,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8F3
// 首条位置: {"module":1227,"inlinee":59635}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1227,"record_offset":13064,"parent_record_offset":12980,"ancestor_record_offset":12980,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01227.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=13
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1227,"record_offset":13008,"parent_record_offset":12980,"ancestor_record_offset":12980,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01227.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 311
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE831
// 首条位置: {"module":1206,"inlinee":59441}
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
// 完整结构及原始类型记录: find_overlap_segments.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment>", "constructor": "core::ptr::drop_in_place", "arguments": ["system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: find_overlap_segments.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: find_overlap_segments.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::WallId>,ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>", "ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache::new
// 信息 ID: 827c8cbda6a7636cbe7ed9eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x4d44ed -> 0xc925d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments
// 信息 ID: 2b66dd268fcb9655639d69c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x4d49d6 -> 0xc928c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x4d4ad7 -> 0xc928c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$0
// 信息 ID: 0d5cdac16d3a1a6708aeb181
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$0> >; module=1226, inline_record=42104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$1
// 信息 ID: 15b7c4cf245e1d643214d998
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::merge_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>,alloc::slice::stable_sort::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::merge_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>,alloc::slice::stable_sort::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::merge_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>,alloc::slice::stable_sort::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::merge_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>,alloc::slice::stable_sort::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::merge_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>,alloc::slice::stable_sort::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1 <- core::slice::sort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>,alloc::slice::stable_sort::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> > >; module=1227, inline_record=12980; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>,alloc::slice::stable_sort::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> > >; module=1227, inline_record=13164; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>,alloc::slice::stable_sort::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> > >; module=1227, inline_record=13348; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>,alloc::slice::stable_sort::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> > >; module=1227, inline_record=14824; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1>,alloc::slice::stable_sort::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> > >; module=1227, inline_record=15212; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1227, inline_record=30880; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,alloc::slice::impl$0::sort_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$1> >; module=1227, inline_record=31292; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure$3
// 信息 ID: bd81a8074e09a6892bfc8d1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3 <- itertools::tuple_impl::circular_tuple_windows<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> >,tuple$<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1206, inline_record=19388; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments; module=1206, inline_record=19584; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > > >; module=1262, inline_record=4680; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > >,core::iter::sources::once::Once<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >,ref_mut$<core::iter::adapters::cycle::Cycle<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> > > > > >; module=1262, inline_record=6004; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::circular_tuple_windows<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments::closure_env$3> >,tuple$<tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> >,tuple$<usize,utils::curve::curve_u_circular::CurveUCircular<utils::curve::CurveCoordSemantic2D> > > >; module=1263, inline_record=16792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_elevation_supports::find_overlap_segments::impl$0::new::closure$0
// 信息 ID: 03ce475bd3cb2cf43261ebbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_elevation_supports::find_overlap_segments::impl$0::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache::new; module=1206, inline_record=4456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
