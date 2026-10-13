// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/collision/visualize_colliders.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 1f35aed095eee900695d577f936fb737d18a0c9b2c99aca9db563cb775b1cdd2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: visualize_colliders.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 450, 1308, 1448
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
// use nalgebra; // namespace candidate
//   package identities: [{"name":"nalgebra","source":"PDB_registry_path_only","version":"0.33.0"}]
// use parry3d; // namespace candidate
//   package identities: [{"name":"parry3d","source":"PDB_registry_path_only","version":"0.17.0"}]
// use rapier3d; // namespace candidate
//   package identities: []
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"PDB_registry_path_only","version":"1.0.6"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_colliders
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1448,"record_offset":7892,"rva":17875840,"va":5386584960}
// 编译实例 RVAs: 0x110c380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140f5dd20
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1763002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386584960,"raw_hex":"e84619e5ff","size":5,"soft_fail":false,"target_va":5384822048,"va":5386585045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420e1e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16603469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386584960,"raw_hex":"e84d59fd00","size":5,"soft_fail":false,"target_va":5403188848,"va":5386585374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::visualize_colliders::visualize_shape
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386584960,"raw_hex":"e809010000","size":5,"soft_fail":false,"target_va":5386585392,"va":5386585122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] slotmap::KeyData::from_ffi
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1638868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386584960,"raw_hex":"e82cfee6ff","size":5,"soft_fail":false,"target_va":5384946288,"va":5386585151,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1448,"record_offset":8068,"parent_record_offset":7892,"ancestor_record_offset":7892,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastMesh::source
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1448,"record_offset":8708,"parent_record_offset":7892,"ancestor_record_offset":7892,"inlinee":22782,"source_line":1178,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::RaycastMeshKey::from_rapier_user_data
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1448,"record_offset":8484,"parent_record_offset":7892,"ancestor_record_offset":7892,"inlinee":22775,"source_line":210,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_direct] rapier3d::geometry::collider::Collider::shape
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-e2bc0f97bb8baf69/7b5ece7/crates/rapier3d/../../src/geometry/collider.rs
//     evidence={"module":1448,"record_offset":8248,"parent_record_offset":7892,"ancestor_record_offset":7892,"inlinee":23347,"source_line":431,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_direct] slotmap::basic::impl$7::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":1448,"record_offset":8512,"parent_record_offset":7892,"ancestor_record_offset":7892,"inlinee":22777,"source_line":863,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1448,"record_offset":8092,"parent_record_offset":8068,"ancestor_record_offset":7892,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1448,"record_offset":8640,"parent_record_offset":8544,"ancestor_record_offset":7892,"inlinee":12684,"source_line":1444,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1448,"record_offset":8204,"parent_record_offset":8180,"ancestor_record_offset":7892,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1448,"record_offset":8152,"parent_record_offset":8116,"ancestor_record_offset":7892,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1448,"record_offset":8576,"parent_record_offset":8544,"ancestor_record_offset":7892,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1448,"record_offset":8604,"parent_record_offset":8576,"ancestor_record_offset":7892,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1448,"record_offset":8180,"parent_record_offset":8116,"ancestor_record_offset":7892,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1448,"record_offset":8116,"parent_record_offset":8092,"ancestor_record_offset":7892,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] rapier3d::data::arena::impl$11::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/rapier-e2bc0f97bb8baf69/7b5ece7/crates/rapier3d/../../src/data/arena.rs
//     evidence={"module":1448,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":7892,"inlinee":59398,"source_line":937,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::SlotMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":1448,"record_offset":8544,"parent_record_offset":8512,"ancestor_record_offset":7892,"inlinee":22778,"source_line":538,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$5::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":1448,"record_offset":8668,"parent_record_offset":8640,"ancestor_record_offset":7892,"inlinee":22780,"source_line":542,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_shape
// 记录形式: compiled_procedure; instances=1
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1448,"record_offset":8768,"rva":17876272,"va":5386585392}
// 编译实例 RVAs: 0x110c530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$dyn$u20$parry3d..shape..shape..Shape$GT$::as_compound
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7731005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386585392,"raw_hex":"e83df77500","size":5,"soft_fail":false,"target_va":5394316528,"va":5386585518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$dyn$u20$parry3d..shape..shape..Shape$GT$::as_convex_polyhedron
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7730394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386585392,"raw_hex":"e8daf47500","size":5,"soft_fail":false,"target_va":5394316656,"va":5386586257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$dyn$u20$parry3d..shape..shape..Shape$GT$::as_cuboid
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7729921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386585392,"raw_hex":"e801f37500","size":5,"soft_fail":false,"target_va":5394316464,"va":5386586538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$dyn$u20$parry3d..shape..shape..Shape$GT$::as_trimesh
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7730579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386585392,"raw_hex":"e893f57500","size":5,"soft_fail":false,"target_va":5394316592,"va":5386586008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13837079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386585392,"raw_hex":"e8e9dc2cff","size":5,"soft_fail":false,"target_va":5372749696,"va":5386586770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13836779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386585392,"raw_hex":"e815de2cff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386586486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7b700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3740177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386585392,"raw_hex":"e8efedc6ff","size":5,"soft_fail":false,"target_va":5382846208,"va":5386586380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d80220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: visualize_colliders.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3720670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386585392,"raw_hex":"e8223ac7ff","size":5,"soft_fail":false,"target_va":5382865440,"va":5386586105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16391176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386585392,"raw_hex":"e8081cfa00","size":5,"soft_fail":false,"target_va":5402978144,"va":5386586963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::visualize_colliders::visualize_shape
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386585392,"raw_hex":"e8b0fdffff","size":5,"soft_fail":false,"target_va":5386585392,"va":5386585979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12300567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386585392,"raw_hex":"e8e94e44ff","size":5,"soft_fail":false,"target_va":5374286080,"va":5386586642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-2385499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386585392,"raw_hex":"e8a599dbff","size":5,"soft_fail":false,"target_va":5384200656,"va":5386586150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parry3d::shape::trimesh::TriMesh::indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1643615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386585392,"raw_hex":"e8a1ebe6ff","size":5,"soft_fail":false,"target_va":5384942432,"va":5386586042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parry3d::shape::trimesh::TriMesh::vertices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1643617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386585392,"raw_hex":"e89febe6ff","size":5,"soft_fail":false,"target_va":5384942416,"va":5386586028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parry3d::transformation::to_trimesh::convex_polyhedron_to_trimesh::_$LT$impl$u20$parry3d..shape..convex_polyhedron..ConvexPolyhedron$GT$::to_trimesh
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1689371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386585392,"raw_hex":"e8e538e6ff","size":5,"soft_fail":false,"target_va":5384896912,"va":5386586278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1448,"record_offset":11620,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1448,"record_offset":11460,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1448,"record_offset":11548,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1448,"record_offset":10612,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1448,"record_offset":10948,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1448,"record_offset":10528,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1448,"record_offset":8968,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$29::to_glam
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1448,"record_offset":11376,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":25278,"source_line":1838,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::color
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1448,"record_offset":10920,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":23779,"source_line":384,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::debug_viz::DebugLine::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/debug_viz.rs
//     evidence={"module":1448,"record_offset":10696,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":23780,"source_line":366,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_direct] nalgebra::geometry::isometry_ops::impl$2::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/isometry_ops.rs
//     evidence={"module":1448,"record_offset":9052,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":66934,"source_line":78,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_direct] parry3d::shape::shared_shape::impl$0::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parry3d","version":"0.17.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parry3d-0.17.0/src/shape/shared_shape.rs
//     evidence={"module":1448,"record_offset":10496,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":62498,"source_line":26,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1448,"record_offset":11676,"parent_record_offset":11652,"ancestor_record_offset":8768,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1448,"record_offset":11700,"parent_record_offset":11676,"ancestor_record_offset":8768,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1448,"record_offset":11112,"parent_record_offset":11088,"ancestor_record_offset":8768,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1448,"record_offset":11652,"parent_record_offset":11620,"ancestor_record_offset":8768,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1448,"record_offset":11088,"parent_record_offset":10996,"ancestor_record_offset":8768,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1448,"record_offset":11024,"parent_record_offset":10996,"ancestor_record_offset":8768,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1448,"record_offset":10996,"parent_record_offset":10972,"ancestor_record_offset":8768,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1448,"record_offset":10636,"parent_record_offset":10612,"ancestor_record_offset":8768,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1448,"record_offset":11824,"parent_record_offset":11800,"ancestor_record_offset":8768,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1448,"record_offset":11872,"parent_record_offset":11848,"ancestor_record_offset":8768,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$17::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":1448,"record_offset":9652,"parent_record_offset":9580,"ancestor_record_offset":8768,"inlinee":6622,"source_line":219,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1448,"record_offset":10660,"parent_record_offset":10636,"ancestor_record_offset":8768,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":1448,"record_offset":11488,"parent_record_offset":11460,"ancestor_record_offset":8768,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1448,"record_offset":11052,"parent_record_offset":11024,"ancestor_record_offset":8768,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1448,"record_offset":9216,"parent_record_offset":9188,"ancestor_record_offset":8768,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1448,"record_offset":9280,"parent_record_offset":9188,"ancestor_record_offset":8768,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1448,"record_offset":10192,"parent_record_offset":10164,"ancestor_record_offset":8768,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1448,"record_offset":9392,"parent_record_offset":9188,"ancestor_record_offset":8768,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1448,"record_offset":10972,"parent_record_offset":10948,"ancestor_record_offset":8768,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1448,"record_offset":10576,"parent_record_offset":10552,"ancestor_record_offset":8768,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1448,"record_offset":9016,"parent_record_offset":8968,"ancestor_record_offset":8768,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1448,"record_offset":11728,"parent_record_offset":11700,"ancestor_record_offset":8768,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1448,"record_offset":10552,"parent_record_offset":10528,"ancestor_record_offset":8768,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1448,"record_offset":11400,"parent_record_offset":11376,"ancestor_record_offset":8768,"inlinee":25259,"source_line":84,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1448,"record_offset":11424,"parent_record_offset":11400,"ancestor_record_offset":8768,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::add_assign_statically_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1448,"record_offset":10164,"parent_record_offset":10136,"ancestor_record_offset":8768,"inlinee":23871,"source_line":183,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::add_assign_statically_unchecked_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1448,"record_offset":10420,"parent_record_offset":10392,"ancestor_record_offset":8768,"inlinee":63747,"source_line":212,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/matrix.rs
//     evidence={"module":1448,"record_offset":9580,"parent_record_offset":9464,"ancestor_record_offset":8768,"inlinee":23864,"source_line":2054,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$21::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1448,"record_offset":10136,"parent_record_offset":10108,"ancestor_record_offset":8768,"inlinee":23870,"source_line":272,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$22::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1448,"record_offset":10392,"parent_record_offset":9084,"ancestor_record_offset":8768,"inlinee":63746,"source_line":290,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$23::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1448,"record_offset":10108,"parent_record_offset":9464,"ancestor_record_offset":8768,"inlinee":23868,"source_line":308,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$34::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1448,"record_offset":9508,"parent_record_offset":9464,"ancestor_record_offset":8768,"inlinee":23866,"source_line":467,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::unit::impl$11::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/unit.rs
//     evidence={"module":1448,"record_offset":10328,"parent_record_offset":9084,"ancestor_record_offset":8768,"inlinee":63755,"source_line":26,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::abstract_rotation::impl$1::transform_vector
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/abstract_rotation.rs
//     evidence={"module":1448,"record_offset":9432,"parent_record_offset":9084,"ancestor_record_offset":8768,"inlinee":23934,"source_line":93,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::isometry::Isometry::from_parts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/isometry.rs
//     evidence={"module":1448,"record_offset":10356,"parent_record_offset":9084,"ancestor_record_offset":8768,"inlinee":23915,"source_line":119,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::isometry_ops::impl$3::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/isometry_ops.rs
//     evidence={"module":1448,"record_offset":9084,"parent_record_offset":9052,"ancestor_record_offset":8768,"inlinee":63738,"source_line":78,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$12::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1448,"record_offset":9188,"parent_record_offset":9160,"ancestor_record_offset":8768,"inlinee":63744,"source_line":93,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$16::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1448,"record_offset":9160,"parent_record_offset":9132,"ancestor_record_offset":8768,"inlinee":63742,"source_line":93,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$19::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1448,"record_offset":9132,"parent_record_offset":9084,"ancestor_record_offset":8768,"inlinee":63740,"source_line":93,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$40::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1448,"record_offset":9464,"parent_record_offset":9432,"ancestor_record_offset":8768,"inlinee":23863,"source_line":93,"archive":"_pdb/A/modules/01448.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_shape::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x64DA
// 首条位置: {"module":450,"inlinee":25818}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x64E9
// 首条位置: {"module":450,"inlinee":25833}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] nalgebra::geometry::isometry::Isometry::transform_point
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/isometry.rs
//     evidence={"module":1308,"record_offset":17256,"parent_record_offset":17228,"ancestor_record_offset":17228,"inlinee":25821,"source_line":306,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$17::clone
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":1308,"record_offset":17476,"parent_record_offset":17440,"ancestor_record_offset":17228,"inlinee":6622,"source_line":219,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1308,"record_offset":17560,"parent_record_offset":17440,"ancestor_record_offset":17228,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1308,"record_offset":18244,"parent_record_offset":18216,"ancestor_record_offset":17228,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1308,"record_offset":17648,"parent_record_offset":17440,"ancestor_record_offset":17228,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::add_assign_statically_unchecked
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1308,"record_offset":18216,"parent_record_offset":18188,"ancestor_record_offset":17228,"inlinee":23871,"source_line":183,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::cross
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/matrix.rs
//     evidence={"module":1308,"record_offset":17440,"parent_record_offset":17388,"ancestor_record_offset":17228,"inlinee":23864,"source_line":2054,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$21::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1308,"record_offset":18188,"parent_record_offset":18160,"ancestor_record_offset":17228,"inlinee":23870,"source_line":272,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$23::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1308,"record_offset":18160,"parent_record_offset":17388,"ancestor_record_offset":17228,"inlinee":23868,"source_line":308,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$34::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":1308,"record_offset":17820,"parent_record_offset":17388,"ancestor_record_offset":17228,"inlinee":23866,"source_line":467,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::abstract_rotation::impl$1::transform_point
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/abstract_rotation.rs
//     evidence={"module":1308,"record_offset":17316,"parent_record_offset":17284,"ancestor_record_offset":17228,"inlinee":25824,"source_line":98,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::isometry_ops::impl$29::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/isometry_ops.rs
//     evidence={"module":1308,"record_offset":17284,"parent_record_offset":17256,"ancestor_record_offset":17228,"inlinee":25823,"source_line":78,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::point_ops::impl$14::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/op_macros.rs
//     evidence={"module":1308,"record_offset":18436,"parent_record_offset":18396,"ancestor_record_offset":17228,"inlinee":25830,"source_line":187,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion::Quaternion::scalar
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/quaternion.rs
//     evidence={"module":1308,"record_offset":17764,"parent_record_offset":17388,"ancestor_record_offset":17228,"inlinee":23935,"source_line":214,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$40::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1308,"record_offset":17388,"parent_record_offset":17352,"ancestor_record_offset":17228,"inlinee":23863,"source_line":93,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$44::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":1308,"record_offset":17352,"parent_record_offset":17316,"ancestor_record_offset":17228,"inlinee":25826,"source_line":93,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::translation_ops::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/op_macros.rs
//     evidence={"module":1308,"record_offset":18396,"parent_record_offset":17284,"ancestor_record_offset":17228,"inlinee":25828,"source_line":187,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_shape::closure$1
// 记录形式: inlinee_declaration; instances=1
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x64E7
// 首条位置: {"module":450,"inlinee":25831}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x64DC
// 首条位置: {"module":450,"inlinee":25820}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":450,"record_offset":42600,"parent_record_offset":42560,"ancestor_record_offset":42560,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":450,"record_offset":43952,"parent_record_offset":42560,"ancestor_record_offset":42560,"inlinee":22717,"source_line":1357,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_direct] nalgebra::geometry::isometry::Isometry::transform_point
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/isometry.rs
//     evidence={"module":450,"record_offset":42708,"parent_record_offset":42560,"ancestor_record_offset":42560,"inlinee":25821,"source_line":306,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$17::clone
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":450,"record_offset":42884,"parent_record_offset":42848,"ancestor_record_offset":42560,"inlinee":6622,"source_line":219,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":450,"record_offset":42996,"parent_record_offset":42848,"ancestor_record_offset":42560,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":450,"record_offset":43628,"parent_record_offset":43600,"ancestor_record_offset":42560,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":450,"record_offset":43044,"parent_record_offset":42848,"ancestor_record_offset":42560,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":450,"record_offset":42632,"parent_record_offset":42600,"ancestor_record_offset":42560,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":450,"record_offset":42664,"parent_record_offset":42632,"ancestor_record_offset":42560,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":450,"record_offset":43976,"parent_record_offset":43952,"ancestor_record_offset":42560,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::add_assign_statically_unchecked
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":450,"record_offset":43600,"parent_record_offset":43572,"ancestor_record_offset":42560,"inlinee":23871,"source_line":183,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::matrix::Matrix::cross
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/matrix.rs
//     evidence={"module":450,"record_offset":42848,"parent_record_offset":42804,"ancestor_record_offset":42560,"inlinee":23864,"source_line":2054,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$21::add
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":450,"record_offset":43572,"parent_record_offset":43544,"ancestor_record_offset":42560,"inlinee":23870,"source_line":272,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$23::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":450,"record_offset":43544,"parent_record_offset":42804,"ancestor_record_offset":42560,"inlinee":23868,"source_line":308,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::base::ops::impl$34::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/base/ops.rs
//     evidence={"module":450,"record_offset":43208,"parent_record_offset":42804,"ancestor_record_offset":42560,"inlinee":23866,"source_line":467,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::abstract_rotation::impl$1::transform_point
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/abstract_rotation.rs
//     evidence={"module":450,"record_offset":42756,"parent_record_offset":42732,"ancestor_record_offset":42560,"inlinee":25824,"source_line":98,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::isometry_ops::impl$29::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/isometry_ops.rs
//     evidence={"module":450,"record_offset":42732,"parent_record_offset":42708,"ancestor_record_offset":42560,"inlinee":25823,"source_line":78,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::point_ops::impl$14::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/op_macros.rs
//     evidence={"module":450,"record_offset":43796,"parent_record_offset":43768,"ancestor_record_offset":42560,"inlinee":25830,"source_line":187,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion::Quaternion::scalar
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/quaternion.rs
//     evidence={"module":450,"record_offset":43152,"parent_record_offset":42804,"ancestor_record_offset":42560,"inlinee":23935,"source_line":214,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$40::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":450,"record_offset":42804,"parent_record_offset":42780,"ancestor_record_offset":42560,"inlinee":23863,"source_line":93,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::quaternion_ops::impl$44::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/quaternion_ops.rs
//     evidence={"module":450,"record_offset":42780,"parent_record_offset":42756,"ancestor_record_offset":42560,"inlinee":25826,"source_line":93,"archive":"_pdb/A/modules/00450.jsonl.gz"}
//   [pdb_inline_transitive] nalgebra::geometry::translation_ops::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nalgebra","version":"0.33.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nalgebra-0.33.0/src/geometry/op_macros.rs
//     evidence={"module":450,"record_offset":43768,"parent_record_offset":42732,"ancestor_record_offset":42560,"inlinee":25828,"source_line":187,"archive":"_pdb/A/modules/00450.jsonl.gz"}
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
// 完整结构及原始类型记录: visualize_colliders.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: visualize_colliders.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: visualize_colliders.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 11; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_colliders
// 信息 ID: f34cbdea137756f03d0a1c60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 56bc6ac7b43a89d4520b70e3: void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x60dd76 -> 0x110c380 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0xe39a41 -> 0x110c380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_shape
// 信息 ID: bf627aa4f875fe1e9ee51248
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::visualize_colliders::visualize_colliders @ 0x110c422 -> 0x110c530 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::visualize_colliders::visualize_shape @ 0x110c77b -> 0x110c530 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_shape::closure$0
// 信息 ID: a8cc7696d295fb662f4fe98c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::visualize_colliders::visualize_shape", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0>; module=1308, inline_record=18988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure$0
// 信息 ID: ae8ab47c3f86de008b24ce72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::collision::visualize_colliders::visualize_shape", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0>; module=1308, inline_record=17228; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0>; module=1308, inline_record=19504; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$0::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$0>; module=1308, inline_record=20972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_shape::closure$1
// 信息 ID: c8c8e4fc21aa810a860c8364
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::collision::visualize_colliders::visualize_shape", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 1 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure$0
// 信息 ID: 7ae3919b7920ae607e376f3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::collision::visualize_colliders::visualize_shape", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<u32>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<u32>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0>; module=450, inline_record=42560; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$1> >; module=1308, inline_record=209616; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$1> >; module=1308, inline_record=211788; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<array$<u32,3> > >,core::iter::adapters::map::Map<core::array::iter::IntoIter<u32,4>,country_core::systems::collision::visualize_colliders::visualize_shape::closure$1::closure_env$0>,country_core::systems::collision::visualize_colliders::visualize_shape::closure_env$1> >; module=1308, inline_record=213368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
