// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/ui/hover_intersection.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 9464aa985b3ccc2ba56bd7891b56aeff4a99aa9c3cc1229b77d8ea45820a91d8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: hover_intersection.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 481, 1308, 1321, 1336, 1426, 1467
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::hover_intersection::hover_intersection
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1467,"record_offset":3060,"rva":18068096,"va":5386777216}
// 编译实例 RVAs: 0x113b280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: hover_intersection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14031683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e8bde429ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386781390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d702c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hover_intersection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3980705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e85f42c3ff","size":5,"soft_fail":false,"target_va":5382800064,"va":5386780764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dec370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hover_intersection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3469596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e8e40ecbff","size":5,"soft_fail":false,"target_va":5383308144,"va":5386777735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ea6310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hover_intersection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2710890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e896a2d6ff","size":5,"soft_fail":false,"target_va":5384069904,"va":5386780789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140eac280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: hover_intersection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2686719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e80101d7ff","size":5,"soft_fail":false,"target_va":5384094336,"va":5386781050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16137243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e81b3cf600","size":5,"soft_fail":false,"target_va":5402918336,"va":5386781088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-15469786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e826f313ff","size":5,"soft_fail":false,"target_va":5371308800,"va":5386778581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16197130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e80a26f700","size":5,"soft_fail":false,"target_va":5402978688,"va":5386781553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16197350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e8e626f700","size":5,"soft_fail":false,"target_va":5402978880,"va":5386781525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16197828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e8c428f700","size":5,"soft_fail":false,"target_va":5402979344,"va":5386781511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e8a2eaffff","size":5,"soft_fail":false,"target_va":5386775936,"va":5386781401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16202339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e8633af700","size":5,"soft_fail":false,"target_va":5402980128,"va":5386777784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e827fbffff","size":5,"soft_fail":false,"target_va":5386776128,"va":5386777364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t16203210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e8ca3df700","size":5,"soft_fail":false,"target_va":5402980656,"va":5386777441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10421043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e8cdfc60ff","size":5,"soft_fail":false,"target_va":5376356912,"va":5386777950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10417160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e8f80b61ff","size":5,"soft_fail":false,"target_va":5376360560,"va":5386777715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10421205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e82bfc60ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5386777344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::is_inside_bbx
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2359060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e8ec00dcff","size":5,"soft_fail":false,"target_va":5384419952,"va":5386779007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::intersect::SphereIntersection::closest_intersection_distance
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-4510408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e8382dbbff","size":5,"soft_fail":false,"target_va":5382268112,"va":5386778515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::intersect::Triangle::normal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4513460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e84c21bbff","size":5,"soft_fail":false,"target_va":5382266976,"va":5386780431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::intersect::ray_sphere_intersection
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4510331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e8852dbbff","size":5,"soft_fail":false,"target_va":5382268160,"va":5386778486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::intersect::ray_triangle_intersection
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4511424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e84029bbff","size":5,"soft_fail":false,"target_va":5382267488,"va":5386778907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::plane::Plane::intersect_ray
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8127047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e847027c00","size":5,"soft_fail":false,"target_va":5394906640,"va":5386779588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::ray::Ray::from_mouse_ss
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3617240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e828cec8ff","size":5,"soft_fail":false,"target_va":5383160832,"va":5386778067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8820562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e8ae6879ff","size":5,"soft_fail":false,"target_va":5377959584,"va":5386780141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8825662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386777216,"raw_hex":"e8c25479ff","size":5,"soft_fail":false,"target_va":5377953168,"va":5386778825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1467,"record_offset":11188,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":6488,"source_line":263,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":6020,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":6048,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":7256,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":11244,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":6475,"source_line":1653,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":10696,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":11748,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1467,"record_offset":5932,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1467,"record_offset":7536,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":8709,"source_line":3310,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1467,"record_offset":12556,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1467,"record_offset":7452,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":1467,"record_offset":7392,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":22044,"source_line":28,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::window::WindowSize::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/window.rs
//     evidence={"module":1467,"record_offset":5992,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21913,"source_line":35,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::hover_intersection::hover_intersection::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/hover_intersection.rs
//     evidence={"module":1467,"record_offset":7116,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":67229,"source_line":51,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":10072,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":9984,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1467,"record_offset":4884,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1467,"record_offset":3296,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1467,"record_offset":3872,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1467,"record_offset":6076,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1467,"record_offset":6408,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1467,"record_offset":10136,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1467,"record_offset":12732,"parent_record_offset":12708,"ancestor_record_offset":3060,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1467,"record_offset":12708,"parent_record_offset":12608,"ancestor_record_offset":3060,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":11920,"parent_record_offset":11892,"ancestor_record_offset":3060,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":12640,"parent_record_offset":12608,"ancestor_record_offset":3060,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":11892,"parent_record_offset":11860,"ancestor_record_offset":3060,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":7328,"parent_record_offset":7296,"ancestor_record_offset":3060,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":11860,"parent_record_offset":11836,"ancestor_record_offset":3060,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":12608,"parent_record_offset":12580,"ancestor_record_offset":3060,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1467,"record_offset":11212,"parent_record_offset":11188,"ancestor_record_offset":3060,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":7296,"parent_record_offset":7256,"ancestor_record_offset":3060,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":11796,"parent_record_offset":11772,"ancestor_record_offset":3060,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":11308,"parent_record_offset":11272,"ancestor_record_offset":3060,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":11836,"parent_record_offset":11796,"ancestor_record_offset":3060,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":11272,"parent_record_offset":11244,"ancestor_record_offset":3060,"inlinee":6476,"source_line":1679,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":11644,"parent_record_offset":11620,"ancestor_record_offset":3060,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":12192,"parent_record_offset":12164,"ancestor_record_offset":3060,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":11372,"parent_record_offset":11336,"ancestor_record_offset":3060,"inlinee":6480,"source_line":1657,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":11620,"parent_record_offset":11596,"ancestor_record_offset":3060,"inlinee":6487,"source_line":1707,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::process_loop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1467,"record_offset":11336,"parent_record_offset":11272,"ancestor_record_offset":3060,"inlinee":6478,"source_line":1727,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1467,"record_offset":12268,"parent_record_offset":12192,"ancestor_record_offset":3060,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1467,"record_offset":11972,"parent_record_offset":11796,"ancestor_record_offset":3060,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1467,"record_offset":12432,"parent_record_offset":12408,"ancestor_record_offset":3060,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1467,"record_offset":11772,"parent_record_offset":11748,"ancestor_record_offset":3060,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1467,"record_offset":5960,"parent_record_offset":5932,"ancestor_record_offset":3060,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1467,"record_offset":5284,"parent_record_offset":5256,"ancestor_record_offset":3060,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1467,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":3060,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1467,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":3060,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1467,"record_offset":5224,"parent_record_offset":5196,"ancestor_record_offset":3060,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1467,"record_offset":5256,"parent_record_offset":5224,"ancestor_record_offset":3060,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1467,"record_offset":5428,"parent_record_offset":5256,"ancestor_record_offset":3060,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1467,"record_offset":5540,"parent_record_offset":5516,"ancestor_record_offset":3060,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1467,"record_offset":8288,"parent_record_offset":8256,"ancestor_record_offset":3060,"inlinee":6144,"source_line":1275,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1467,"record_offset":6264,"parent_record_offset":6236,"ancestor_record_offset":3060,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1467,"record_offset":6324,"parent_record_offset":6292,"ancestor_record_offset":3060,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1467,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":3060,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1467,"record_offset":9092,"parent_record_offset":9032,"ancestor_record_offset":3060,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1467,"record_offset":11484,"parent_record_offset":11440,"ancestor_record_offset":3060,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1467,"record_offset":8016,"parent_record_offset":7988,"ancestor_record_offset":3060,"inlinee":11890,"source_line":96,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1467,"record_offset":8184,"parent_record_offset":8044,"ancestor_record_offset":3060,"inlinee":6128,"source_line":106,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1467,"record_offset":7664,"parent_record_offset":7636,"ancestor_record_offset":3060,"inlinee":8626,"source_line":73,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1467,"record_offset":7800,"parent_record_offset":7692,"ancestor_record_offset":3060,"inlinee":8628,"source_line":85,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::filter_map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1467,"record_offset":8216,"parent_record_offset":8184,"ancestor_record_offset":3060,"inlinee":25841,"source_line":39,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1467,"record_offset":7988,"parent_record_offset":7572,"ancestor_record_offset":3060,"inlinee":25235,"source_line":144,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":1467,"record_offset":7608,"parent_record_offset":7572,"ancestor_record_offset":3060,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1467,"record_offset":12028,"parent_record_offset":12000,"ancestor_record_offset":3060,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1467,"record_offset":12136,"parent_record_offset":12056,"ancestor_record_offset":3060,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1467,"record_offset":7636,"parent_record_offset":7608,"ancestor_record_offset":3060,"inlinee":8594,"source_line":2898,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1467,"record_offset":7836,"parent_record_offset":7800,"ancestor_record_offset":3060,"inlinee":8630,"source_line":2905,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1467,"record_offset":12000,"parent_record_offset":11796,"ancestor_record_offset":3060,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1467,"record_offset":12164,"parent_record_offset":12136,"ancestor_record_offset":3060,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1467,"record_offset":8256,"parent_record_offset":8216,"ancestor_record_offset":3060,"inlinee":6142,"source_line":3317,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1467,"record_offset":7572,"parent_record_offset":7536,"ancestor_record_offset":3060,"inlinee":8710,"source_line":2618,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1467,"record_offset":7692,"parent_record_offset":7664,"ancestor_record_offset":3060,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1467,"record_offset":11572,"parent_record_offset":11272,"ancestor_record_offset":3060,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1467,"record_offset":5360,"parent_record_offset":5336,"ancestor_record_offset":3060,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1467,"record_offset":8496,"parent_record_offset":8044,"ancestor_record_offset":3060,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1467,"record_offset":12672,"parent_record_offset":12640,"ancestor_record_offset":3060,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1467,"record_offset":3508,"parent_record_offset":3412,"ancestor_record_offset":3060,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1467,"record_offset":6684,"parent_record_offset":6656,"ancestor_record_offset":3060,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1467,"record_offset":7164,"parent_record_offset":7140,"ancestor_record_offset":3060,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1467,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":3060,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1467,"record_offset":10012,"parent_record_offset":9984,"ancestor_record_offset":3060,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1467,"record_offset":7864,"parent_record_offset":7836,"ancestor_record_offset":3060,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1467,"record_offset":8320,"parent_record_offset":8288,"ancestor_record_offset":3060,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":5120,"parent_record_offset":5092,"ancestor_record_offset":3060,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":4468,"parent_record_offset":4008,"ancestor_record_offset":3060,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":5168,"parent_record_offset":4992,"ancestor_record_offset":3060,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":8376,"parent_record_offset":8348,"ancestor_record_offset":3060,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":7416,"parent_record_offset":7392,"ancestor_record_offset":3060,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":7920,"parent_record_offset":7892,"ancestor_record_offset":3060,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1467,"record_offset":3820,"parent_record_offset":3792,"ancestor_record_offset":3060,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1467,"record_offset":8152,"parent_record_offset":8128,"ancestor_record_offset":3060,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1467,"record_offset":5492,"parent_record_offset":5196,"ancestor_record_offset":3060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1467,"record_offset":7500,"parent_record_offset":7476,"ancestor_record_offset":3060,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1467,"record_offset":6816,"parent_record_offset":6788,"ancestor_record_offset":3060,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1467,"record_offset":6788,"parent_record_offset":6760,"ancestor_record_offset":3060,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1467,"record_offset":8528,"parent_record_offset":8044,"ancestor_record_offset":3060,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1467,"record_offset":4244,"parent_record_offset":4220,"ancestor_record_offset":3060,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1467,"record_offset":4220,"parent_record_offset":4196,"ancestor_record_offset":3060,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1467,"record_offset":8128,"parent_record_offset":8044,"ancestor_record_offset":3060,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1467,"record_offset":4428,"parent_record_offset":4396,"ancestor_record_offset":3060,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1467,"record_offset":5384,"parent_record_offset":5360,"ancestor_record_offset":3060,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1467,"record_offset":5892,"parent_record_offset":4956,"ancestor_record_offset":3060,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1467,"record_offset":3444,"parent_record_offset":3412,"ancestor_record_offset":3060,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1467,"record_offset":10724,"parent_record_offset":10696,"ancestor_record_offset":3060,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1467,"record_offset":3472,"parent_record_offset":3444,"ancestor_record_offset":3060,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1467,"record_offset":10752,"parent_record_offset":10724,"ancestor_record_offset":3060,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1467,"record_offset":7476,"parent_record_offset":7452,"ancestor_record_offset":3060,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1467,"record_offset":4196,"parent_record_offset":4172,"ancestor_record_offset":3060,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1467,"record_offset":4524,"parent_record_offset":4500,"ancestor_record_offset":3060,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1467,"record_offset":8044,"parent_record_offset":8016,"ancestor_record_offset":3060,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1467,"record_offset":7728,"parent_record_offset":7692,"ancestor_record_offset":3060,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1467,"record_offset":4172,"parent_record_offset":4064,"ancestor_record_offset":3060,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1467,"record_offset":3412,"parent_record_offset":3384,"ancestor_record_offset":3060,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1467,"record_offset":3904,"parent_record_offset":3872,"ancestor_record_offset":3060,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1467,"record_offset":4036,"parent_record_offset":4008,"ancestor_record_offset":3060,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1467,"record_offset":4008,"parent_record_offset":3976,"ancestor_record_offset":3060,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1467,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":3060,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1467,"record_offset":3976,"parent_record_offset":3952,"ancestor_record_offset":3060,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1467,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":3060,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1467,"record_offset":4500,"parent_record_offset":3976,"ancestor_record_offset":3060,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1467,"record_offset":3608,"parent_record_offset":3576,"ancestor_record_offset":3060,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1467,"record_offset":3768,"parent_record_offset":3608,"ancestor_record_offset":3060,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1467,"record_offset":3576,"parent_record_offset":3552,"ancestor_record_offset":3060,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1467,"record_offset":3328,"parent_record_offset":3296,"ancestor_record_offset":3060,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1467,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":3060,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1467,"record_offset":3384,"parent_record_offset":3352,"ancestor_record_offset":3060,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1467,"record_offset":3352,"parent_record_offset":3328,"ancestor_record_offset":3060,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1467,"record_offset":4064,"parent_record_offset":4036,"ancestor_record_offset":3060,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1467,"record_offset":4312,"parent_record_offset":4064,"ancestor_record_offset":3060,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1467,"record_offset":4284,"parent_record_offset":4064,"ancestor_record_offset":3060,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1467,"record_offset":4116,"parent_record_offset":4064,"ancestor_record_offset":3060,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::hover_intersection::hover_intersection::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/hover_intersection.rs
//     evidence={"module":1467,"record_offset":9032,"parent_record_offset":9008,"ancestor_record_offset":3060,"inlinee":66234,"source_line":88,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::hover_intersection::hover_intersection::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/hover_intersection.rs
//     evidence={"module":1467,"record_offset":9624,"parent_record_offset":9596,"ancestor_record_offset":3060,"inlinee":66236,"source_line":99,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::hover_intersection::hover_intersection::closure$3
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/hover_intersection.rs
//     evidence={"module":1467,"record_offset":7892,"parent_record_offset":7864,"ancestor_record_offset":3060,"inlinee":66235,"source_line":115,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::hover_intersection::hover_intersection::closure$4
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/hover_intersection.rs
//     evidence={"module":1467,"record_offset":8348,"parent_record_offset":8320,"ancestor_record_offset":3060,"inlinee":66237,"source_line":121,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::hover_intersection::hover_intersection::closure$7
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/hover_intersection.rs
//     evidence={"module":1467,"record_offset":11400,"parent_record_offset":11372,"ancestor_record_offset":3060,"inlinee":61445,"source_line":224,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::hover_intersection::hover_intersection::closure$9
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/hover_intersection.rs
//     evidence={"module":1467,"record_offset":12304,"parent_record_offset":12136,"ancestor_record_offset":3060,"inlinee":61485,"source_line":235,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":10100,"parent_record_offset":10072,"ancestor_record_offset":3060,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":7196,"parent_record_offset":7116,"ancestor_record_offset":3060,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":7140,"parent_record_offset":7116,"ancestor_record_offset":3060,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1467,"record_offset":6100,"parent_record_offset":6076,"ancestor_record_offset":3060,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1467,"record_offset":6436,"parent_record_offset":6408,"ancestor_record_offset":3060,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1467,"record_offset":6760,"parent_record_offset":6512,"ancestor_record_offset":3060,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1467,"record_offset":6208,"parent_record_offset":6148,"ancestor_record_offset":3060,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1467,"record_offset":6512,"parent_record_offset":6464,"ancestor_record_offset":3060,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1467,"record_offset":6124,"parent_record_offset":6100,"ancestor_record_offset":3060,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1467,"record_offset":6180,"parent_record_offset":6148,"ancestor_record_offset":3060,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1467,"record_offset":6148,"parent_record_offset":6124,"ancestor_record_offset":3060,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1467,"record_offset":6616,"parent_record_offset":6572,"ancestor_record_offset":3060,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1467,"record_offset":6656,"parent_record_offset":6616,"ancestor_record_offset":3060,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1467,"record_offset":6724,"parent_record_offset":6572,"ancestor_record_offset":3060,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1467,"record_offset":6572,"parent_record_offset":6512,"ancestor_record_offset":3060,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1467,"record_offset":6464,"parent_record_offset":6436,"ancestor_record_offset":3060,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1467,"record_offset":6236,"parent_record_offset":6208,"ancestor_record_offset":3060,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1467,"record_offset":6292,"parent_record_offset":6236,"ancestor_record_offset":3060,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1467,"record_offset":4920,"parent_record_offset":4884,"ancestor_record_offset":3060,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1467,"record_offset":5032,"parent_record_offset":4992,"ancestor_record_offset":3060,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1467,"record_offset":5196,"parent_record_offset":4992,"ancestor_record_offset":3060,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1467,"record_offset":5464,"parent_record_offset":5196,"ancestor_record_offset":3060,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1467,"record_offset":5060,"parent_record_offset":5032,"ancestor_record_offset":3060,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1467,"record_offset":5092,"parent_record_offset":5060,"ancestor_record_offset":3060,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1467,"record_offset":4992,"parent_record_offset":4956,"ancestor_record_offset":3060,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1467,"record_offset":4956,"parent_record_offset":4920,"ancestor_record_offset":3060,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::hover_intersection::hover_intersection::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1069D
// 首条位置: {"module":1467,"inlinee":67229}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":7196,"parent_record_offset":7116,"ancestor_record_offset":7116,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1467,"record_offset":7140,"parent_record_offset":7116,"ancestor_record_offset":7116,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1467,"record_offset":7164,"parent_record_offset":7140,"ancestor_record_offset":7116,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1467,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":7116,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::hover_intersection::hover_intersection::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x102BA
// 首条位置: {"module":1426,"inlinee":66234}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1467,"record_offset":9092,"parent_record_offset":9032,"ancestor_record_offset":9032,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$40::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":9060,"parent_record_offset":9032,"ancestor_record_offset":9032,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1467,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":9032,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::hover_intersection::hover_intersection::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x102BC
// 首条位置: {"module":1426,"inlinee":66236}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":9652,"parent_record_offset":9624,"ancestor_record_offset":9624,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::hover_intersection::hover_intersection::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x102BB
// 首条位置: {"module":1426,"inlinee":66235}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$40::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":7920,"parent_record_offset":7892,"ancestor_record_offset":7892,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::hover_intersection::hover_intersection::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x102BD
// 首条位置: {"module":1426,"inlinee":66237}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1467,"record_offset":8376,"parent_record_offset":8348,"ancestor_record_offset":8348,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::hover_intersection::hover_intersection::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEFFF
// 首条位置: {"module":1308,"inlinee":61439}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":1308,"record_offset":55208,"parent_record_offset":54992,"ancestor_record_offset":54992,"inlinee":23594,"source_line":1098,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1308,"record_offset":55120,"parent_record_offset":54992,"ancestor_record_offset":54992,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1308,"record_offset":55032,"parent_record_offset":54992,"ancestor_record_offset":54992,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1308,"record_offset":55060,"parent_record_offset":55032,"ancestor_record_offset":54992,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1308,"record_offset":55148,"parent_record_offset":55120,"ancestor_record_offset":54992,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1308,"record_offset":55236,"parent_record_offset":55208,"ancestor_record_offset":54992,"inlinee":21868,"source_line":223,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::hover_intersection::hover_intersection::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 221
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF327
// 首条位置: {"module":1336,"inlinee":62247}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::impl$53::cmp
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":1336,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":8612,"inlinee":62249,"source_line":255,"archive":"_pdb/A/modules/01336.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$67::cmp
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1336,"record_offset":8660,"parent_record_offset":8636,"ancestor_record_offset":8612,"inlinee":7055,"source_line":1565,"archive":"_pdb/A/modules/01336.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::hover_intersection::hover_intersection::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 224
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF005
// 首条位置: {"module":1308,"inlinee":61445}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::hover_intersection::hover_intersection::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 229
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF335
// 首条位置: {"module":1336,"inlinee":62261}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1336,"record_offset":65892,"parent_record_offset":65864,"ancestor_record_offset":65864,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01336.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui::hover_intersection::hover_intersection::closure$9
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 235
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF02D
// 首条位置: {"module":1308,"inlinee":61485}
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
// 完整结构及原始类型记录: hover_intersection.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::ui::hover_intersection::WidgetIntersection
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::alloc::Global,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$5>", "constructor": "alloc::vec::Vec::retain", "arguments": ["country_core::systems::ui::hover_intersection::WidgetIntersection", "alloc::alloc::Global", "country_core::systems::ui::hover_intersection::hover_intersection::closure_env$5"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>", "constructor": "tuple$", "arguments": ["ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>", "country_core::systems::ui::hover_intersection::WidgetIntersection"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>", "country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >", "constructor": "alloc::slice::stable_sort::closure_env$0", "arguments": ["tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >", "constructor": "alloc::slice::stable_sort::closure_env$1", "arguments": ["tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >", "constructor": "alloc::slice::stable_sort::closure_env$2", "arguments": ["tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >", "constructor": "alloc::slice::stable_sort::closure_env$3", "arguments": ["tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::merge_sort<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> > >", "constructor": "core::slice::sort::merge_sort", "arguments": ["tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>", "alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >", "alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >", "alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >", "alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["country_core::systems::ui::hover_intersection::WidgetIntersection", "country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >", "constructor": "alloc::slice::stable_sort::closure_env$0", "arguments": ["country_core::systems::ui::hover_intersection::WidgetIntersection", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$1<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >", "constructor": "alloc::slice::stable_sort::closure_env$1", "arguments": ["country_core::systems::ui::hover_intersection::WidgetIntersection", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$2<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >", "constructor": "alloc::slice::stable_sort::closure_env$2", "arguments": ["country_core::systems::ui::hover_intersection::WidgetIntersection", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$3<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >", "constructor": "alloc::slice::stable_sort::closure_env$3", "arguments": ["country_core::systems::ui::hover_intersection::WidgetIntersection", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::merge_sort<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>,alloc::slice::stable_sort::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> > >", "constructor": "core::slice::sort::merge_sort", "arguments": ["country_core::systems::ui::hover_intersection::WidgetIntersection", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>", "alloc::slice::stable_sort::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >", "alloc::slice::stable_sort::closure_env$1<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >", "alloc::slice::stable_sort::closure_env$2<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >", "alloc::slice::stable_sort::closure_env$3<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_left<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >", "constructor": "core::slice::sort::insertion_sort_shift_left", "arguments": ["country_core::systems::ui::hover_intersection::WidgetIntersection", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_left<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >", "constructor": "core::slice::sort::insertion_sort_shift_left", "arguments": ["tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>", "alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: hover_intersection.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: hover_intersection.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 29; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,country_core::systems::world_raster::RasterDrawResources,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathEraseEvent>,bevy_ecs::event::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>,bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "country_core::systems::world_raster::RasterDrawResources", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathEraseEvent>", "bevy_ecs::event::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>", "bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathEraseEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::brush_sizes::BrushSizes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::paths::draw_path::DrawState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::history::replay_raster::RasterReplayResources,bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>)
// 候选参数类型: ["country_core::systems::history::replay_raster::RasterReplayResources", "bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventReader<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>", "bevy_ecs::event::EventReader<country_core::resources::session::SessionLoadedCmd>", "bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::session::NewSessionStartedCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::session::SessionLoadedCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentRefDeserializer<serde_json::error::Error> (*)(ref$<enum2$<serde::__private::de::content::Content> >)
// 候选参数类型: ["ref$<enum2$<serde::__private::de::content::Content> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::system::system_param::Local<country_core::systems::ui::debug_visualize_volumes::DebugRoofs>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >", "bevy_ecs::system::commands::Commands", "bevy_ecs::system::system_param::Local<country_core::systems::ui::debug_visualize_volumes::DebugRoofs>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui::debug_visualize_volumes::DebugRoofs>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::hover_intersection::hover_intersection
// 信息 ID: 3e537ccdab5de4fc946eee0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d6d02250425de9fc35de1627: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)> > @ 0xe4b1fe -> 0x113b280 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>)> @ 0x107db0c -> 0x113b280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::hover_intersection::hover_intersection::closure$0
// 信息 ID: a7563ec81fa794419805fd10
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ui::hover_intersection::hover_intersection", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::hover_intersection::hover_intersection; module=1467, inline_record=7116; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::hover_intersection::hover_intersection; module=1467, inline_record=10920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::hover_intersection::hover_intersection::closure$1
// 信息 ID: c4380c426a7eb8cc8948dd88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::ui::hover_intersection::hover_intersection", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::hover_intersection::hover_intersection; module=1467, inline_record=9032; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::hover_intersection::hover_intersection; module=1467, inline_record=9704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::hover_intersection::hover_intersection::closure$2
// 信息 ID: 01af0141dfb63726b976e4ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::ui::hover_intersection::hover_intersection", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::hover_intersection::hover_intersection; module=1467, inline_record=9624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::hover_intersection::hover_intersection::closure$3
// 信息 ID: 264905c455ae45c978a06181
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::systems::ui::hover_intersection::hover_intersection", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::hover_intersection::hover_intersection; module=1467, inline_record=7892; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::hover_intersection::hover_intersection; module=1467, inline_record=8428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::hover_intersection::hover_intersection::closure$4
// 信息 ID: c8132aaa3e1756f0fcf84dab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "country_core::systems::ui::hover_intersection::hover_intersection", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::hover_intersection::hover_intersection; module=1467, inline_record=8348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::hover_intersection::hover_intersection::closure$5
// 信息 ID: e88685da418f8148f96593eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "country_core::systems::ui::hover_intersection::hover_intersection", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ui::hover_intersection::hover_intersection::closure_env$5 <- alloc::vec::Vec::retain<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::alloc::Global,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$5>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::alloc::Global,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$5>; module=1308, inline_record=54992; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::alloc::Global,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$5>; module=1308, inline_record=55384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::hover_intersection::hover_intersection::closure$6
// 信息 ID: d777e921d9ea0a2c38cf3521
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "country_core::systems::ui::hover_intersection::hover_intersection", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6 <- core::slice::sort::merge_sort<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6 <- core::slice::sort::merge_sort<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6 <- core::slice::sort::merge_sort<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6 <- core::slice::sort::merge_sort<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6 <- core::slice::sort::merge_sort<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6 <- core::slice::sort::insertion_sort_shift_left<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> > >; module=1336, inline_record=8612; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> > >; module=1336, inline_record=8728; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> > >; module=1336, inline_record=8848; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> > >; module=1336, inline_record=9880; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6>,alloc::slice::stable_sort::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$1<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$2<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >,alloc::slice::stable_sort::closure_env$3<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> > >; module=1336, inline_record=10212; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >; module=1336, inline_record=113892; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<ref$<country_core::systems::ui::resources::widget_params::WorldCanvasLayer>,country_core::systems::ui::hover_intersection::WidgetIntersection>,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$6> >; module=1336, inline_record=114188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::hover_intersection::hover_intersection::closure$7
// 信息 ID: 32fe6358be4dae23f562c753
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "country_core::systems::ui::hover_intersection::hover_intersection", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::hover_intersection::hover_intersection; module=1467, inline_record=11400; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::hover_intersection::hover_intersection; module=1467, inline_record=11536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::hover_intersection::hover_intersection::closure$8
// 信息 ID: 527ea0bc9e42737565ca4966
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "country_core::systems::ui::hover_intersection::hover_intersection", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8 <- core::slice::sort::merge_sort<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>,alloc::slice::stable_sort::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8 <- core::slice::sort::merge_sort<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>,alloc::slice::stable_sort::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8 <- core::slice::sort::merge_sort<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>,alloc::slice::stable_sort::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8 <- core::slice::sort::merge_sort<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>,alloc::slice::stable_sort::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8 <- core::slice::sort::merge_sort<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>,alloc::slice::stable_sort::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8 <- core::slice::sort::insertion_sort_shift_left<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>,alloc::slice::stable_sort::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> > >; module=1336, inline_record=65864; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>,alloc::slice::stable_sort::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> > >; module=1336, inline_record=65996; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>,alloc::slice::stable_sort::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> > >; module=1336, inline_record=66108; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>,alloc::slice::stable_sort::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> > >; module=1336, inline_record=67144; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8>,alloc::slice::stable_sort::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> > >; module=1336, inline_record=67480; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >; module=1336, inline_record=107000; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<country_core::systems::ui::hover_intersection::WidgetIntersection,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::ui::hover_intersection::WidgetIntersection,country_core::systems::ui::hover_intersection::hover_intersection::closure_env$8> >; module=1336, inline_record=107400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui::hover_intersection::hover_intersection::closure$9
// 信息 ID: d952e64a027f86ac035550ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "country_core::systems::ui::hover_intersection::hover_intersection", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::hover_intersection::hover_intersection; module=1467, inline_record=12304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
