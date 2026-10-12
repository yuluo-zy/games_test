// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/ivy/ivy_spawner.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 815641b2ef959630b847f1a19f521bcb65aeb130e3009be3cca99cf25b1b62eb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ivy_spawner.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 392
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
// use perchance; // namespace candidate
//   package identities: [{"name":"perchance","source":"PDB_registry_path_only","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_spawner::ivy_spawner
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":392,"record_offset":27416,"rva":5363360,"va":5374072480}
// 编译实例 RVAs: 0x51d6a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4459543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e8e9f3bbff","size":5,"soft_fail":false,"target_va":5369613520,"va":5374073058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1323792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e8f0ccebff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374073499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2870550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e816cd2b00","size":5,"soft_fail":false,"target_va":5376943968,"va":5374073413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7ce50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8777200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e8f0ed8500","size":5,"soft_fail":false,"target_va":5382852176,"va":5374074971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7e580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8785150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e8fe0c8600","size":5,"soft_fail":false,"target_va":5382858112,"va":5374072957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e88710
//     owner=unresolved_namespace; count=4
//     target_aliases=5 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9874712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e818ad9600","size":5,"soft_fail":false,"target_va":5383948048,"va":5374073331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e88fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9875559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e867b09600","size":5,"soft_fail":false,"target_va":5383950272,"va":5374074708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8a0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9880373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e835c39600","size":5,"soft_fail":false,"target_va":5383954608,"va":5374074230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29520350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e8de71c201","size":5,"soft_fail":false,"target_va":5403594096,"va":5374073741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bracket_noise::fastnoise::FastNoise::get_noise
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10324269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e82d899d00","size":5,"soft_fail":false,"target_va":5384397712,"va":5374073438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28902146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e80203b901","size":5,"soft_fail":false,"target_va":5402978880,"va":5374076729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t28902454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e83604b901","size":5,"soft_fail":false,"target_va":5402979152,"va":5374076693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t28902766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e86e05b901","size":5,"soft_fail":false,"target_va":5402979344,"va":5374076573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28902080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e8c002b901","size":5,"soft_fail":false,"target_va":5402978944,"va":5374076859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::ivy_storage::IvyStorage::spawn_ivy
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t494025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e8c9890700","size":5,"soft_fail":false,"target_va":5374569648,"va":5374075618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ivy::ivy_grower::closest_segment_index
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1060331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e8eb2d1000","size":5,"soft_fail":false,"target_va":5375134304,"va":5374073968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::quat::Quat::from_rotation_arc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3272158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e8deed3100","size":5,"soft_fail":false,"target_va":5377348352,"va":5374076189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] perchance::PerchanceContext::get_bool
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10313187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e8e35d9d00","size":5,"soft_fail":false,"target_va":5384387712,"va":5374074520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] perchance::PerchanceContext::uniform_f32
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10314654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e89e639d00","size":5,"soft_fail":false,"target_va":5384387936,"va":5374073277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] perchance::PerchanceContext::usize_less_than
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t10314781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e81d649d00","size":5,"soft_fail":false,"target_va":5384387792,"va":5374073006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::is_inside_bbx
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10343510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374072480,"raw_hex":"e856d49d00","size":5,"soft_fail":false,"target_va":5384419952,"va":5374076437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":392,"record_offset":29956,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":392,"record_offset":37912,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":392,"record_offset":34308,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":392,"record_offset":37508,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":392,"record_offset":37076,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":392,"record_offset":38748,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::action_set::DigitalActionState::pressed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":392,"record_offset":28788,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":21908,"source_line":140,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":392,"record_offset":34972,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":22044,"source_line":28,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::IvyStorage::iter
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":392,"record_offset":27668,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":22370,"source_line":163,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::IvyStorage::roots
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":392,"record_offset":36556,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":22392,"source_line":155,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":392,"record_offset":35032,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":392,"record_offset":33236,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_spawner::is_point_inside_wall_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_spawner.rs
//     evidence={"module":392,"record_offset":38872,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":22399,"source_line":293,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_spawner::try_ivy_root_next_to_cursor
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_spawner.rs
//     evidence={"module":392,"record_offset":28832,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":22375,"source_line":244,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_spawner::try_random_ivy_root
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_spawner.rs
//     evidence={"module":392,"record_offset":30012,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":22376,"source_line":213,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":392,"record_offset":38008,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":22384,"source_line":115,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":34872,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":35748,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$33::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":34908,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":14666,"source_line":1299,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":35660,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":35572,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::uniform_range_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/perchance-0.4.0/src/lib.rs
//     evidence={"module":392,"record_offset":38568,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":22397,"source_line":270,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::uniform_range_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/perchance-0.4.0/src/lib.rs
//     evidence={"module":392,"record_offset":37884,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":22396,"source_line":243,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$53::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":392,"record_offset":28076,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":22373,"source_line":2392,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":35116,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":34432,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":22389,"source_line":615,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":34404,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":392,"record_offset":34344,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::is_within_01_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":392,"record_offset":38720,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":22073,"source_line":216,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":392,"record_offset":38624,"parent_record_offset":27416,"ancestor_record_offset":27416,"inlinee":21345,"source_line":189,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":392,"record_offset":32660,"parent_record_offset":32636,"ancestor_record_offset":27416,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":392,"record_offset":32636,"parent_record_offset":32544,"ancestor_record_offset":27416,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":32572,"parent_record_offset":32544,"ancestor_record_offset":27416,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":32544,"parent_record_offset":32520,"ancestor_record_offset":27416,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":30636,"parent_record_offset":30012,"ancestor_record_offset":27416,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":32092,"parent_record_offset":32036,"ancestor_record_offset":27416,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":30600,"parent_record_offset":30576,"ancestor_record_offset":27416,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":392,"record_offset":29980,"parent_record_offset":29956,"ancestor_record_offset":27416,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":31884,"parent_record_offset":31860,"ancestor_record_offset":27416,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":31908,"parent_record_offset":31884,"ancestor_record_offset":27416,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":37968,"parent_record_offset":37940,"ancestor_record_offset":27416,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":31512,"parent_record_offset":31488,"ancestor_record_offset":27416,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":392,"record_offset":39360,"parent_record_offset":39336,"ancestor_record_offset":27416,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":392,"record_offset":27952,"parent_record_offset":27924,"ancestor_record_offset":27416,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":392,"record_offset":30980,"parent_record_offset":30956,"ancestor_record_offset":27416,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":392,"record_offset":28012,"parent_record_offset":27980,"ancestor_record_offset":27416,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":39860,"parent_record_offset":39444,"ancestor_record_offset":27416,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":39768,"parent_record_offset":39712,"ancestor_record_offset":27416,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":39540,"parent_record_offset":39516,"ancestor_record_offset":27416,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":39572,"parent_record_offset":39488,"ancestor_record_offset":27416,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":39632,"parent_record_offset":39444,"ancestor_record_offset":27416,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":392,"record_offset":29772,"parent_record_offset":29440,"ancestor_record_offset":27416,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":392,"record_offset":38688,"parent_record_offset":38624,"ancestor_record_offset":27416,"inlinee":21346,"source_line":535,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":392,"record_offset":31948,"parent_record_offset":31796,"ancestor_record_offset":27416,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":392,"record_offset":29740,"parent_record_offset":29468,"ancestor_record_offset":27416,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":392,"record_offset":31976,"parent_record_offset":31796,"ancestor_record_offset":27416,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":392,"record_offset":31004,"parent_record_offset":30980,"ancestor_record_offset":27416,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":392,"record_offset":37940,"parent_record_offset":37912,"ancestor_record_offset":27416,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":392,"record_offset":30576,"parent_record_offset":30116,"ancestor_record_offset":27416,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":392,"record_offset":32600,"parent_record_offset":32572,"ancestor_record_offset":27416,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":392,"record_offset":28372,"parent_record_offset":28348,"ancestor_record_offset":27416,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":29880,"parent_record_offset":29852,"ancestor_record_offset":27416,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":29096,"parent_record_offset":29072,"ancestor_record_offset":27416,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":29040,"parent_record_offset":29016,"ancestor_record_offset":27416,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$286::neg
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":34940,"parent_record_offset":34908,"ancestor_record_offset":27416,"inlinee":14668,"source_line":694,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":29524,"parent_record_offset":29500,"ancestor_record_offset":27416,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":392,"record_offset":31860,"parent_record_offset":31836,"ancestor_record_offset":27416,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":392,"record_offset":31836,"parent_record_offset":31796,"ancestor_record_offset":27416,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":392,"record_offset":29344,"parent_record_offset":28948,"ancestor_record_offset":27416,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":392,"record_offset":31760,"parent_record_offset":30012,"ancestor_record_offset":27416,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":392,"record_offset":28916,"parent_record_offset":28892,"ancestor_record_offset":27416,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":392,"record_offset":29408,"parent_record_offset":28832,"ancestor_record_offset":27416,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":392,"record_offset":27896,"parent_record_offset":27868,"ancestor_record_offset":27416,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":392,"record_offset":32496,"parent_record_offset":30012,"ancestor_record_offset":27416,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":392,"record_offset":28620,"parent_record_offset":28596,"ancestor_record_offset":27416,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":392,"record_offset":28596,"parent_record_offset":28572,"ancestor_record_offset":27416,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":392,"record_offset":38836,"parent_record_offset":38812,"ancestor_record_offset":27416,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":392,"record_offset":37172,"parent_record_offset":37116,"ancestor_record_offset":27416,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":392,"record_offset":29280,"parent_record_offset":28948,"ancestor_record_offset":27416,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":392,"record_offset":29380,"parent_record_offset":28832,"ancestor_record_offset":27416,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":392,"record_offset":32120,"parent_record_offset":32092,"ancestor_record_offset":27416,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":392,"record_offset":29308,"parent_record_offset":29280,"ancestor_record_offset":27416,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":392,"record_offset":32148,"parent_record_offset":32120,"ancestor_record_offset":27416,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":392,"record_offset":38812,"parent_record_offset":38748,"ancestor_record_offset":27416,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":392,"record_offset":37116,"parent_record_offset":37076,"ancestor_record_offset":27416,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":392,"record_offset":28892,"parent_record_offset":28832,"ancestor_record_offset":27416,"inlinee":22044,"source_line":28,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":392,"record_offset":31536,"parent_record_offset":31512,"ancestor_record_offset":27416,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":392,"record_offset":30696,"parent_record_offset":30012,"ancestor_record_offset":27416,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_walls::PublicWalls::wall_ids
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":392,"record_offset":30116,"parent_record_offset":30012,"ancestor_record_offset":27416,"inlinee":22378,"source_line":54,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ivy::ivy_spawner::ivy_spawner::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_spawner.rs
//     evidence={"module":392,"record_offset":37212,"parent_record_offset":37076,"ancestor_record_offset":27416,"inlinee":22394,"source_line":112,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":392,"record_offset":29128,"parent_record_offset":28980,"ancestor_record_offset":27416,"inlinee":22386,"source_line":525,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::grid_coord_to_cell_index
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":392,"record_offset":29248,"parent_record_offset":28948,"ancestor_record_offset":27416,"inlinee":22387,"source_line":508,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":392,"record_offset":28948,"parent_record_offset":28832,"ancestor_record_offset":27416,"inlinee":22384,"source_line":115,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::world_space_to_grid_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":392,"record_offset":28980,"parent_record_offset":28948,"ancestor_record_offset":27416,"inlinee":22385,"source_line":492,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":392,"record_offset":39336,"parent_record_offset":38872,"ancestor_record_offset":27416,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":392,"record_offset":39444,"parent_record_offset":39336,"ancestor_record_offset":27416,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":39888,"parent_record_offset":39444,"ancestor_record_offset":27416,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":39408,"parent_record_offset":39384,"ancestor_record_offset":27416,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":39384,"parent_record_offset":39360,"ancestor_record_offset":27416,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29644,"parent_record_offset":29612,"ancestor_record_offset":27416,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":34756,"parent_record_offset":34732,"ancestor_record_offset":27416,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":34732,"parent_record_offset":34708,"ancestor_record_offset":27416,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29828,"parent_record_offset":29804,"ancestor_record_offset":27416,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":34708,"parent_record_offset":34684,"ancestor_record_offset":27416,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29072,"parent_record_offset":28980,"ancestor_record_offset":27416,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29500,"parent_record_offset":29468,"ancestor_record_offset":27416,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29160,"parent_record_offset":28980,"ancestor_record_offset":27416,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29016,"parent_record_offset":28980,"ancestor_record_offset":27416,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29852,"parent_record_offset":29828,"ancestor_record_offset":27416,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":37300,"parent_record_offset":37212,"ancestor_record_offset":27416,"inlinee":22395,"source_line":469,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":37268,"parent_record_offset":37212,"ancestor_record_offset":27416,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":37328,"parent_record_offset":37300,"ancestor_record_offset":27416,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":392,"record_offset":39516,"parent_record_offset":39488,"ancestor_record_offset":27416,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":392,"record_offset":39488,"parent_record_offset":39444,"ancestor_record_offset":27416,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":392,"record_offset":31488,"parent_record_offset":31464,"ancestor_record_offset":27416,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":392,"record_offset":30756,"parent_record_offset":30732,"ancestor_record_offset":27416,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":392,"record_offset":30780,"parent_record_offset":30756,"ancestor_record_offset":27416,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":392,"record_offset":27752,"parent_record_offset":27724,"ancestor_record_offset":27416,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":392,"record_offset":31464,"parent_record_offset":31244,"ancestor_record_offset":27416,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":392,"record_offset":28148,"parent_record_offset":28112,"ancestor_record_offset":27416,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":392,"record_offset":30808,"parent_record_offset":30780,"ancestor_record_offset":27416,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":31376,"parent_record_offset":31352,"ancestor_record_offset":27416,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":31352,"parent_record_offset":31244,"ancestor_record_offset":27416,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":31292,"parent_record_offset":31268,"ancestor_record_offset":27416,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":28572,"parent_record_offset":28224,"ancestor_record_offset":27416,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":31672,"parent_record_offset":30884,"ancestor_record_offset":27416,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":27868,"parent_record_offset":27808,"ancestor_record_offset":27416,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":28224,"parent_record_offset":28184,"ancestor_record_offset":27416,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":31268,"parent_record_offset":31244,"ancestor_record_offset":27416,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":30860,"parent_record_offset":30836,"ancestor_record_offset":27416,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":30836,"parent_record_offset":30780,"ancestor_record_offset":27416,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":27780,"parent_record_offset":27752,"ancestor_record_offset":27416,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":30312,"parent_record_offset":30248,"ancestor_record_offset":27416,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":27840,"parent_record_offset":27808,"ancestor_record_offset":27416,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":30884,"parent_record_offset":30860,"ancestor_record_offset":27416,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":27808,"parent_record_offset":27780,"ancestor_record_offset":27416,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":392,"record_offset":28316,"parent_record_offset":28276,"ancestor_record_offset":27416,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":392,"record_offset":28348,"parent_record_offset":28316,"ancestor_record_offset":27416,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":392,"record_offset":28408,"parent_record_offset":28276,"ancestor_record_offset":27416,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":392,"record_offset":28276,"parent_record_offset":28224,"ancestor_record_offset":27416,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":30928,"parent_record_offset":30884,"ancestor_record_offset":27416,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":28184,"parent_record_offset":28148,"ancestor_record_offset":27416,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":31244,"parent_record_offset":30884,"ancestor_record_offset":27416,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":392,"record_offset":30956,"parent_record_offset":30884,"ancestor_record_offset":27416,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":392,"record_offset":27924,"parent_record_offset":27868,"ancestor_record_offset":27416,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":392,"record_offset":31040,"parent_record_offset":30884,"ancestor_record_offset":27416,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":392,"record_offset":31580,"parent_record_offset":30884,"ancestor_record_offset":27416,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":392,"record_offset":27980,"parent_record_offset":27924,"ancestor_record_offset":27416,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::choose
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/perchance-0.4.0/src/lib.rs
//     evidence={"module":392,"record_offset":30664,"parent_record_offset":30012,"ancestor_record_offset":27416,"inlinee":22380,"source_line":203,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/perchance-0.4.0/src/lib.rs
//     evidence={"module":392,"record_offset":38592,"parent_record_offset":38568,"ancestor_record_offset":27416,"inlinee":22398,"source_line":275,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":392,"record_offset":30732,"parent_record_offset":30696,"ancestor_record_offset":27416,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":392,"record_offset":27724,"parent_record_offset":27696,"ancestor_record_offset":27416,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":392,"record_offset":30140,"parent_record_offset":30116,"ancestor_record_offset":27416,"inlinee":17284,"source_line":361,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":392,"record_offset":27696,"parent_record_offset":27668,"ancestor_record_offset":27416,"inlinee":22371,"source_line":423,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":392,"record_offset":28112,"parent_record_offset":28076,"ancestor_record_offset":27416,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":392,"record_offset":29216,"parent_record_offset":28980,"ancestor_record_offset":27416,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":392,"record_offset":34820,"parent_record_offset":34788,"ancestor_record_offset":27416,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":34472,"parent_record_offset":34432,"ancestor_record_offset":27416,"inlinee":22390,"source_line":519,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":32036,"parent_record_offset":32008,"ancestor_record_offset":27416,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":32008,"parent_record_offset":30012,"ancestor_record_offset":27416,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":31728,"parent_record_offset":30696,"ancestor_record_offset":27416,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":32064,"parent_record_offset":32036,"ancestor_record_offset":27416,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":32188,"parent_record_offset":32036,"ancestor_record_offset":27416,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":392,"record_offset":31796,"parent_record_offset":30012,"ancestor_record_offset":27416,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":392,"record_offset":29612,"parent_record_offset":29468,"ancestor_record_offset":27416,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":392,"record_offset":29804,"parent_record_offset":29440,"ancestor_record_offset":27416,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":392,"record_offset":34684,"parent_record_offset":34432,"ancestor_record_offset":27416,"inlinee":22391,"source_line":65,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":392,"record_offset":29440,"parent_record_offset":28832,"ancestor_record_offset":27416,"inlinee":21612,"source_line":181,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":392,"record_offset":29468,"parent_record_offset":29440,"ancestor_record_offset":27416,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_spawner::ivy_spawner::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x577A
// 首条位置: {"module":392,"inlinee":22394}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":37300,"parent_record_offset":37212,"ancestor_record_offset":37212,"inlinee":22395,"source_line":469,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":37268,"parent_record_offset":37212,"ancestor_record_offset":37212,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":37388,"parent_record_offset":37328,"ancestor_record_offset":37212,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":37420,"parent_record_offset":37328,"ancestor_record_offset":37212,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":392,"record_offset":37328,"parent_record_offset":37300,"ancestor_record_offset":37212,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_spawner::try_random_ivy_root
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 213
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5768
// 首条位置: {"module":392,"inlinee":22376}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":30636,"parent_record_offset":30012,"ancestor_record_offset":30012,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":392,"record_offset":31760,"parent_record_offset":30012,"ancestor_record_offset":30012,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":392,"record_offset":32496,"parent_record_offset":30012,"ancestor_record_offset":30012,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":392,"record_offset":30696,"parent_record_offset":30012,"ancestor_record_offset":30012,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::wall_ids
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":392,"record_offset":30116,"parent_record_offset":30012,"ancestor_record_offset":30012,"inlinee":22378,"source_line":54,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::choose
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/perchance-0.4.0/src/lib.rs
//     evidence={"module":392,"record_offset":30664,"parent_record_offset":30012,"ancestor_record_offset":30012,"inlinee":22380,"source_line":203,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":32008,"parent_record_offset":30012,"ancestor_record_offset":30012,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":392,"record_offset":31796,"parent_record_offset":30012,"ancestor_record_offset":30012,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":392,"record_offset":32660,"parent_record_offset":32636,"ancestor_record_offset":30012,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":392,"record_offset":32636,"parent_record_offset":32544,"ancestor_record_offset":30012,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":32572,"parent_record_offset":32544,"ancestor_record_offset":30012,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":392,"record_offset":32544,"parent_record_offset":32520,"ancestor_record_offset":30012,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":32092,"parent_record_offset":32036,"ancestor_record_offset":30012,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":392,"record_offset":30600,"parent_record_offset":30576,"ancestor_record_offset":30012,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":31884,"parent_record_offset":31860,"ancestor_record_offset":30012,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":31908,"parent_record_offset":31884,"ancestor_record_offset":30012,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":392,"record_offset":31512,"parent_record_offset":31488,"ancestor_record_offset":30012,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":392,"record_offset":30428,"parent_record_offset":30400,"ancestor_record_offset":30012,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":392,"record_offset":30980,"parent_record_offset":30956,"ancestor_record_offset":30012,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":392,"record_offset":30488,"parent_record_offset":30456,"ancestor_record_offset":30012,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":392,"record_offset":31948,"parent_record_offset":31796,"ancestor_record_offset":30012,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":392,"record_offset":31976,"parent_record_offset":31796,"ancestor_record_offset":30012,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":392,"record_offset":31004,"parent_record_offset":30980,"ancestor_record_offset":30012,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":392,"record_offset":30576,"parent_record_offset":30116,"ancestor_record_offset":30012,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":392,"record_offset":32600,"parent_record_offset":32572,"ancestor_record_offset":30012,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":392,"record_offset":31176,"parent_record_offset":31152,"ancestor_record_offset":30012,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":32420,"parent_record_offset":32360,"ancestor_record_offset":30012,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":32448,"parent_record_offset":32360,"ancestor_record_offset":30012,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":32392,"parent_record_offset":32360,"ancestor_record_offset":30012,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":392,"record_offset":31860,"parent_record_offset":31836,"ancestor_record_offset":30012,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":392,"record_offset":31836,"parent_record_offset":31796,"ancestor_record_offset":30012,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":392,"record_offset":30372,"parent_record_offset":30340,"ancestor_record_offset":30012,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":392,"record_offset":32520,"parent_record_offset":32496,"ancestor_record_offset":30012,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":392,"record_offset":31424,"parent_record_offset":31400,"ancestor_record_offset":30012,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":392,"record_offset":31316,"parent_record_offset":31292,"ancestor_record_offset":30012,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":392,"record_offset":32120,"parent_record_offset":32092,"ancestor_record_offset":30012,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":392,"record_offset":32148,"parent_record_offset":32120,"ancestor_record_offset":30012,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":392,"record_offset":31536,"parent_record_offset":31512,"ancestor_record_offset":30012,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":32336,"parent_record_offset":32312,"ancestor_record_offset":30012,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":32360,"parent_record_offset":32336,"ancestor_record_offset":30012,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":392,"record_offset":31488,"parent_record_offset":31464,"ancestor_record_offset":30012,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":392,"record_offset":30756,"parent_record_offset":30732,"ancestor_record_offset":30012,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":392,"record_offset":30780,"parent_record_offset":30756,"ancestor_record_offset":30012,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":392,"record_offset":30192,"parent_record_offset":30164,"ancestor_record_offset":30012,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":392,"record_offset":31464,"parent_record_offset":31244,"ancestor_record_offset":30012,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":392,"record_offset":30808,"parent_record_offset":30780,"ancestor_record_offset":30012,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":31376,"parent_record_offset":31352,"ancestor_record_offset":30012,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":31352,"parent_record_offset":31244,"ancestor_record_offset":30012,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":31292,"parent_record_offset":31268,"ancestor_record_offset":30012,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":31672,"parent_record_offset":30884,"ancestor_record_offset":30012,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":30340,"parent_record_offset":30248,"ancestor_record_offset":30012,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":31268,"parent_record_offset":31244,"ancestor_record_offset":30012,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":30860,"parent_record_offset":30836,"ancestor_record_offset":30012,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":30836,"parent_record_offset":30780,"ancestor_record_offset":30012,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":30220,"parent_record_offset":30192,"ancestor_record_offset":30012,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":30312,"parent_record_offset":30248,"ancestor_record_offset":30012,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":30284,"parent_record_offset":30248,"ancestor_record_offset":30012,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":30884,"parent_record_offset":30860,"ancestor_record_offset":30012,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":30248,"parent_record_offset":30220,"ancestor_record_offset":30012,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":392,"record_offset":31124,"parent_record_offset":31096,"ancestor_record_offset":30012,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":392,"record_offset":31152,"parent_record_offset":31124,"ancestor_record_offset":30012,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":392,"record_offset":31212,"parent_record_offset":31096,"ancestor_record_offset":30012,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":392,"record_offset":31096,"parent_record_offset":30884,"ancestor_record_offset":30012,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":30928,"parent_record_offset":30884,"ancestor_record_offset":30012,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":392,"record_offset":31244,"parent_record_offset":30884,"ancestor_record_offset":30012,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":392,"record_offset":30956,"parent_record_offset":30884,"ancestor_record_offset":30012,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":392,"record_offset":30400,"parent_record_offset":30340,"ancestor_record_offset":30012,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":392,"record_offset":31040,"parent_record_offset":30884,"ancestor_record_offset":30012,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":392,"record_offset":31580,"parent_record_offset":30884,"ancestor_record_offset":30012,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":392,"record_offset":30456,"parent_record_offset":30400,"ancestor_record_offset":30012,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":392,"record_offset":30732,"parent_record_offset":30696,"ancestor_record_offset":30012,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":392,"record_offset":30164,"parent_record_offset":30140,"ancestor_record_offset":30012,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":392,"record_offset":30140,"parent_record_offset":30116,"ancestor_record_offset":30012,"inlinee":17284,"source_line":361,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":32036,"parent_record_offset":32008,"ancestor_record_offset":30012,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":31728,"parent_record_offset":30696,"ancestor_record_offset":30012,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":32064,"parent_record_offset":32036,"ancestor_record_offset":30012,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":392,"record_offset":32188,"parent_record_offset":32036,"ancestor_record_offset":30012,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":392,"record_offset":32312,"parent_record_offset":32036,"ancestor_record_offset":30012,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_spawner::try_ivy_root_next_to_cursor
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 244
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5767
// 首条位置: {"module":392,"inlinee":22375}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":392,"record_offset":29408,"parent_record_offset":28832,"ancestor_record_offset":28832,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":392,"record_offset":29380,"parent_record_offset":28832,"ancestor_record_offset":28832,"inlinee":7501,"source_line":157,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":392,"record_offset":28892,"parent_record_offset":28832,"ancestor_record_offset":28832,"inlinee":22044,"source_line":28,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":392,"record_offset":28948,"parent_record_offset":28832,"ancestor_record_offset":28832,"inlinee":22384,"source_line":115,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":392,"record_offset":29440,"parent_record_offset":28832,"ancestor_record_offset":28832,"inlinee":21612,"source_line":181,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":392,"record_offset":29772,"parent_record_offset":29440,"ancestor_record_offset":28832,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":392,"record_offset":29740,"parent_record_offset":29468,"ancestor_record_offset":28832,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":29880,"parent_record_offset":29852,"ancestor_record_offset":28832,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":29096,"parent_record_offset":29072,"ancestor_record_offset":28832,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":29040,"parent_record_offset":29016,"ancestor_record_offset":28832,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":29524,"parent_record_offset":29500,"ancestor_record_offset":28832,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":392,"record_offset":29344,"parent_record_offset":28948,"ancestor_record_offset":28832,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":392,"record_offset":28916,"parent_record_offset":28892,"ancestor_record_offset":28832,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":392,"record_offset":29280,"parent_record_offset":28948,"ancestor_record_offset":28832,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":392,"record_offset":29308,"parent_record_offset":29280,"ancestor_record_offset":28832,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":392,"record_offset":29128,"parent_record_offset":28980,"ancestor_record_offset":28832,"inlinee":22386,"source_line":525,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::grid_coord_to_cell_index
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":392,"record_offset":29248,"parent_record_offset":28948,"ancestor_record_offset":28832,"inlinee":22387,"source_line":508,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::world_space_to_grid_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":392,"record_offset":28980,"parent_record_offset":28948,"ancestor_record_offset":28832,"inlinee":22385,"source_line":492,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29644,"parent_record_offset":29612,"ancestor_record_offset":28832,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29828,"parent_record_offset":29804,"ancestor_record_offset":28832,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29072,"parent_record_offset":28980,"ancestor_record_offset":28832,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29500,"parent_record_offset":29468,"ancestor_record_offset":28832,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29160,"parent_record_offset":28980,"ancestor_record_offset":28832,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29016,"parent_record_offset":28980,"ancestor_record_offset":28832,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29852,"parent_record_offset":29828,"ancestor_record_offset":28832,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":392,"record_offset":29216,"parent_record_offset":28980,"ancestor_record_offset":28832,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":392,"record_offset":29612,"parent_record_offset":29468,"ancestor_record_offset":28832,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":392,"record_offset":29804,"parent_record_offset":29440,"ancestor_record_offset":28832,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":392,"record_offset":29468,"parent_record_offset":29440,"ancestor_record_offset":28832,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_spawner::is_point_inside_wall_segment
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 293
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x577F
// 首条位置: {"module":392,"inlinee":22399}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":392,"record_offset":39336,"parent_record_offset":38872,"ancestor_record_offset":38872,"inlinee":14676,"source_line":659,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":40188,"parent_record_offset":38872,"ancestor_record_offset":38872,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":38980,"parent_record_offset":38872,"ancestor_record_offset":38872,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":39168,"parent_record_offset":38872,"ancestor_record_offset":38872,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":38924,"parent_record_offset":38872,"ancestor_record_offset":38872,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":39224,"parent_record_offset":38872,"ancestor_record_offset":38872,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":392,"record_offset":39360,"parent_record_offset":39336,"ancestor_record_offset":38872,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":39860,"parent_record_offset":39444,"ancestor_record_offset":38872,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":39768,"parent_record_offset":39712,"ancestor_record_offset":38872,"inlinee":21747,"source_line":20,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":39540,"parent_record_offset":39516,"ancestor_record_offset":38872,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":39572,"parent_record_offset":39488,"ancestor_record_offset":38872,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":392,"record_offset":39632,"parent_record_offset":39444,"ancestor_record_offset":38872,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":39132,"parent_record_offset":39004,"ancestor_record_offset":38872,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":39192,"parent_record_offset":39168,"ancestor_record_offset":38872,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":39248,"parent_record_offset":39224,"ancestor_record_offset":38872,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":392,"record_offset":38948,"parent_record_offset":38924,"ancestor_record_offset":38872,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":392,"record_offset":39444,"parent_record_offset":39336,"ancestor_record_offset":38872,"inlinee":14681,"source_line":731,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":39888,"parent_record_offset":39444,"ancestor_record_offset":38872,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":39408,"parent_record_offset":39384,"ancestor_record_offset":38872,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":392,"record_offset":39384,"parent_record_offset":39360,"ancestor_record_offset":38872,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":39064,"parent_record_offset":39032,"ancestor_record_offset":38872,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":39032,"parent_record_offset":39004,"ancestor_record_offset":38872,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":39004,"parent_record_offset":38980,"ancestor_record_offset":38872,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":392,"record_offset":39516,"parent_record_offset":39488,"ancestor_record_offset":38872,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":392,"record_offset":39488,"parent_record_offset":39444,"ancestor_record_offset":38872,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":392,"record_offset":39096,"parent_record_offset":39064,"ancestor_record_offset":38872,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00392.jsonl.gz"}
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
// 完整结构及原始类型记录: ivy_spawner.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ivy_spawner.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ivy_spawner.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_spawner::is_point_inside_wall_segment
// 信息 ID: d0258a5bd9fdc8dd7c63b056
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=38872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_spawner::ivy_spawner
// 信息 ID: dbce7586db2c11de2af6e419
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5f92d1132be17fe3cb4091b9: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>)> > @ 0xe555c0 -> 0x51d6a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>)> @ 0xf0954d -> 0x51d6a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_spawner::ivy_spawner::closure$0
// 信息 ID: 6d847b954c5decc89518f08c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ivy::ivy_spawner::ivy_spawner", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=37212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_spawner::try_ivy_root_next_to_cursor
// 信息 ID: 688d5491e42472fbee629c06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=28832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_spawner::try_random_ivy_root
// 信息 ID: 432cc800e63d7ac7681bb8dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=30012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
