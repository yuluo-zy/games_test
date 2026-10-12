// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/ivy/ivy_spawner.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d4442c6d8e24af294ad4c851e357b5a812510255469a6046915062f82ac8b3ba
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ivy_spawner.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 580
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bracket_noise; // namespace candidate
//   package identities: [{"name":"bracket-noise","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.2"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.20.5"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use perchance; // namespace candidate
//   package identities: [{"name":"perchance","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_spawner::ivy_spawner
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":580,"record_offset":32000,"rva":12323968,"va":5381033088}
// 编译实例 RVAs: 0xbc0c80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5357209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e86741aeff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381035908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7555515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e845b68cff","size":5,"soft_fail":false,"target_va":5373479040,"va":5381034550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2290557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e87df32200","size":5,"soft_fail":false,"target_va":5383325024,"va":5381034462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141600600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10743750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e8c6efa300","size":5,"soft_fail":false,"target_va":5391779328,"va":5381035573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196f620
//     owner=unresolved_namespace; count=4
//     target_aliases=7 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14345367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e897e4da00","size":5,"soft_fail":false,"target_va":5395379744,"va":5381034372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419707a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14349238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e8b6f3da00","size":5,"soft_fail":false,"target_va":5395384224,"va":5381034981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141970d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14350888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e828fada00","size":5,"soft_fail":false,"target_va":5395385648,"va":5381034755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: ivy_spawner.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36024262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e8c6af2502","size":5,"soft_fail":false,"target_va":5417057792,"va":5381033525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bracket_noise::fastnoise::FastNoise::get_noise
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bracket-noise","version":"0.8.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9387811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e8233f8f00","size":5,"soft_fail":false,"target_va":5390422304,"va":5381034488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35586585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e819021f02","size":5,"soft_fail":false,"target_va":5416621600,"va":5381035010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35584833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e841fb1e02","size":5,"soft_fail":false,"target_va":5416622208,"va":5381037370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t35584278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e816f91e02","size":5,"soft_fail":false,"target_va":5416621762,"va":5381037479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35584792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e818fb1e02","size":5,"soft_fail":false,"target_va":5416622240,"va":5381037443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::ivy::ivy_storage::IvyStorage::spawn_ivy
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1751145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e89747e5ff","size":5,"soft_fail":false,"target_va":5379285168,"va":5381036308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ivy::ivy_grower::closest_segment_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e821f6ffff","size":5,"soft_fail":false,"target_va":5381031184,"va":5381033706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::quat::Quat::from_rotation_arc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3573166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e8ae853600","size":5,"soft_fail":false,"target_va":5384610144,"va":5381036973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] perchance::PerchanceContext::usize_less_than
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16301903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e84fbff800","size":5,"soft_fail":false,"target_va":5397335840,"va":5381033932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::is_inside_bbx
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2263689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381033088,"raw_hex":"e8898a2200","size":5,"soft_fail":false,"target_va":5383300912,"va":5381037218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":580,"record_offset":34648,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":580,"record_offset":42236,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":580,"record_offset":42084,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":38760,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":580,"record_offset":41424,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":41028,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":43360,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::input::action_set::DigitalActionState::pressed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":580,"record_offset":33228,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":29412,"source_line":238,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":580,"record_offset":38704,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":28649,"source_line":35,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::IvyStorage::iter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":580,"record_offset":32260,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":29630,"source_line":180,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::ivy::ivy_storage::IvyStorage::roots
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ivy/ivy_storage.rs
//     evidence={"module":580,"record_offset":40516,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":32053,"source_line":172,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":580,"record_offset":38788,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":26091,"source_line":128,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":580,"record_offset":37048,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":28986,"source_line":38,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_spawner::is_point_inside_wall_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_spawner.rs
//     evidence={"module":580,"record_offset":43448,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":36983,"source_line":292,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_spawner::try_ivy_root_next_to_cursor
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_spawner.rs
//     evidence={"module":580,"record_offset":33272,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":36978,"source_line":243,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ivy::ivy_spawner::try_random_ivy_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_spawner.rs
//     evidence={"module":580,"record_offset":34704,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":36979,"source_line":210,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":580,"record_offset":42720,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":33015,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":38676,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":39504,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":39680,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":38612,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":39592,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":9802,"source_line":1361,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::get_bool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":580,"record_offset":38872,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":36980,"source_line":149,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::uniform_range_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":580,"record_offset":42320,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":31957,"source_line":270,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::uniform_range_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":580,"record_offset":42056,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":31956,"source_line":243,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$52::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":580,"record_offset":32624,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":10754,"source_line":2236,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":39020,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":38144,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":26416,"source_line":792,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":38116,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":9818,"source_line":169,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":580,"record_offset":38056,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":26089,"source_line":207,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::is_within_01_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":580,"record_offset":42692,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":36976,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":580,"record_offset":42596,"parent_record_offset":32000,"ancestor_record_offset":32000,"inlinee":26087,"source_line":191,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":41608,"parent_record_offset":41584,"ancestor_record_offset":32000,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":41632,"parent_record_offset":41608,"ancestor_record_offset":32000,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":41656,"parent_record_offset":41632,"ancestor_record_offset":32000,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":580,"record_offset":41584,"parent_record_offset":41496,"ancestor_record_offset":32000,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":41524,"parent_record_offset":41496,"ancestor_record_offset":32000,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":41496,"parent_record_offset":41472,"ancestor_record_offset":32000,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":580,"record_offset":41472,"parent_record_offset":41448,"ancestor_record_offset":32000,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":36608,"parent_record_offset":36556,"ancestor_record_offset":32000,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":40992,"parent_record_offset":40968,"ancestor_record_offset":32000,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":580,"record_offset":34672,"parent_record_offset":34648,"ancestor_record_offset":32000,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":580,"record_offset":36464,"parent_record_offset":36440,"ancestor_record_offset":32000,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":580,"record_offset":36488,"parent_record_offset":36464,"ancestor_record_offset":32000,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":580,"record_offset":42140,"parent_record_offset":42112,"ancestor_record_offset":32000,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":580,"record_offset":37784,"parent_record_offset":37760,"ancestor_record_offset":32000,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":580,"record_offset":43964,"parent_record_offset":43940,"ancestor_record_offset":32000,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":580,"record_offset":32536,"parent_record_offset":32512,"ancestor_record_offset":32000,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":580,"record_offset":37352,"parent_record_offset":37328,"ancestor_record_offset":32000,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":580,"record_offset":37436,"parent_record_offset":37412,"ancestor_record_offset":32000,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":580,"record_offset":44468,"parent_record_offset":44048,"ancestor_record_offset":32000,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":580,"record_offset":44376,"parent_record_offset":44320,"ancestor_record_offset":32000,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":580,"record_offset":44144,"parent_record_offset":44120,"ancestor_record_offset":32000,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":580,"record_offset":44176,"parent_record_offset":44092,"ancestor_record_offset":32000,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":580,"record_offset":44240,"parent_record_offset":44048,"ancestor_record_offset":32000,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":34412,"parent_record_offset":34088,"ancestor_record_offset":32000,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_infinite
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":42660,"parent_record_offset":42596,"ancestor_record_offset":32000,"inlinee":26088,"source_line":626,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":42632,"parent_record_offset":42596,"ancestor_record_offset":32000,"inlinee":12762,"source_line":603,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":34380,"parent_record_offset":34116,"ancestor_record_offset":32000,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":33804,"parent_record_offset":33780,"ancestor_record_offset":32000,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":38556,"parent_record_offset":38532,"ancestor_record_offset":32000,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":37852,"parent_record_offset":37216,"ancestor_record_offset":32000,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_right
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":36336,"parent_record_offset":36312,"ancestor_record_offset":32000,"inlinee":12398,"source_line":2047,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":580,"record_offset":42112,"parent_record_offset":42084,"ancestor_record_offset":32000,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":580,"record_offset":35120,"parent_record_offset":35088,"ancestor_record_offset":32000,"inlinee":6891,"source_line":306,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by::impl$1::spec_advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":580,"record_offset":35144,"parent_record_offset":35120,"ancestor_record_offset":32000,"inlinee":6889,"source_line":325,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":580,"record_offset":40968,"parent_record_offset":40516,"ancestor_record_offset":32000,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":580,"record_offset":35088,"parent_record_offset":34704,"ancestor_record_offset":32000,"inlinee":6943,"source_line":384,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":580,"record_offset":35172,"parent_record_offset":35144,"ancestor_record_offset":32000,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::wrapping_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":36284,"parent_record_offset":36252,"ancestor_record_offset":32000,"inlinee":6064,"source_line":2532,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":41552,"parent_record_offset":41524,"ancestor_record_offset":32000,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_right
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":36312,"parent_record_offset":36252,"ancestor_record_offset":32000,"inlinee":12397,"source_line":414,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":580,"record_offset":32940,"parent_record_offset":32912,"ancestor_record_offset":32000,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":580,"record_offset":32912,"parent_record_offset":32884,"ancestor_record_offset":32000,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":33592,"parent_record_offset":33560,"ancestor_record_offset":32000,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":34516,"parent_record_offset":34492,"ancestor_record_offset":32000,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":33500,"parent_record_offset":33472,"ancestor_record_offset":32000,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":38644,"parent_record_offset":38612,"ancestor_record_offset":32000,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":34172,"parent_record_offset":34148,"ancestor_record_offset":32000,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":580,"record_offset":36440,"parent_record_offset":36416,"ancestor_record_offset":32000,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":580,"record_offset":36416,"parent_record_offset":36384,"ancestor_record_offset":32000,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":33992,"parent_record_offset":33384,"ancestor_record_offset":32000,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":37016,"parent_record_offset":34704,"ancestor_record_offset":32000,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":33352,"parent_record_offset":33328,"ancestor_record_offset":32000,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":34056,"parent_record_offset":33272,"ancestor_record_offset":32000,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":580,"record_offset":32568,"parent_record_offset":32484,"ancestor_record_offset":32000,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":580,"record_offset":37376,"parent_record_offset":37352,"ancestor_record_offset":32000,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":580,"record_offset":41448,"parent_record_offset":41424,"ancestor_record_offset":32000,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":580,"record_offset":33036,"parent_record_offset":33012,"ancestor_record_offset":32000,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":43416,"parent_record_offset":43360,"ancestor_record_offset":32000,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":580,"record_offset":41096,"parent_record_offset":41064,"ancestor_record_offset":32000,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":580,"record_offset":33928,"parent_record_offset":33384,"ancestor_record_offset":32000,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":580,"record_offset":34028,"parent_record_offset":33272,"ancestor_record_offset":32000,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":36632,"parent_record_offset":36608,"ancestor_record_offset":32000,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":33956,"parent_record_offset":33928,"ancestor_record_offset":32000,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":36656,"parent_record_offset":36632,"ancestor_record_offset":32000,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":580,"record_offset":41064,"parent_record_offset":41028,"ancestor_record_offset":32000,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":580,"record_offset":33328,"parent_record_offset":33272,"ancestor_record_offset":32000,"inlinee":28649,"source_line":35,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":580,"record_offset":37808,"parent_record_offset":37784,"ancestor_record_offset":32000,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ivy::ivy_spawner::ivy_spawner::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ivy/ivy_spawner.rs
//     evidence={"module":580,"record_offset":41136,"parent_record_offset":41028,"ancestor_record_offset":32000,"inlinee":36982,"source_line":109,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":580,"record_offset":33652,"parent_record_offset":33424,"ancestor_record_offset":32000,"inlinee":33017,"source_line":538,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::grid_coord_to_cell_index
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":580,"record_offset":33896,"parent_record_offset":33384,"ancestor_record_offset":32000,"inlinee":33018,"source_line":521,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":580,"record_offset":33384,"parent_record_offset":33272,"ancestor_record_offset":32000,"inlinee":33015,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::world_space_to_grid_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":580,"record_offset":33424,"parent_record_offset":33384,"ancestor_record_offset":32000,"inlinee":33016,"source_line":505,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":580,"record_offset":43940,"parent_record_offset":43448,"ancestor_record_offset":32000,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":580,"record_offset":44048,"parent_record_offset":43940,"ancestor_record_offset":32000,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":580,"record_offset":44496,"parent_record_offset":44048,"ancestor_record_offset":32000,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":580,"record_offset":44012,"parent_record_offset":43988,"ancestor_record_offset":32000,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":580,"record_offset":43988,"parent_record_offset":43964,"ancestor_record_offset":32000,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":34288,"parent_record_offset":34260,"ancestor_record_offset":32000,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":38468,"parent_record_offset":38444,"ancestor_record_offset":32000,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":38444,"parent_record_offset":38420,"ancestor_record_offset":32000,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":34468,"parent_record_offset":34444,"ancestor_record_offset":32000,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":38420,"parent_record_offset":38396,"ancestor_record_offset":32000,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":34492,"parent_record_offset":34468,"ancestor_record_offset":32000,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":33692,"parent_record_offset":33424,"ancestor_record_offset":32000,"inlinee":8895,"source_line":1124,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":33560,"parent_record_offset":33424,"ancestor_record_offset":32000,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":34148,"parent_record_offset":34116,"ancestor_record_offset":32000,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":33472,"parent_record_offset":33424,"ancestor_record_offset":32000,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":41216,"parent_record_offset":41136,"ancestor_record_offset":32000,"inlinee":26483,"source_line":585,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":41184,"parent_record_offset":41136,"ancestor_record_offset":32000,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":41244,"parent_record_offset":41216,"ancestor_record_offset":32000,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":580,"record_offset":44120,"parent_record_offset":44092,"ancestor_record_offset":32000,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":580,"record_offset":44092,"parent_record_offset":44048,"ancestor_record_offset":32000,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":580,"record_offset":32844,"parent_record_offset":32804,"ancestor_record_offset":32000,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":580,"record_offset":32884,"parent_record_offset":32844,"ancestor_record_offset":32000,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":580,"record_offset":32980,"parent_record_offset":32804,"ancestor_record_offset":32000,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":580,"record_offset":32804,"parent_record_offset":32760,"ancestor_record_offset":32000,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":580,"record_offset":37328,"parent_record_offset":37216,"ancestor_record_offset":32000,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":580,"record_offset":32512,"parent_record_offset":32484,"ancestor_record_offset":32000,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":580,"record_offset":37880,"parent_record_offset":37216,"ancestor_record_offset":32000,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":580,"record_offset":40856,"parent_record_offset":40800,"ancestor_record_offset":32000,"inlinee":7442,"source_line":101,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":580,"record_offset":37412,"parent_record_offset":37216,"ancestor_record_offset":32000,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":580,"record_offset":37272,"parent_record_offset":37216,"ancestor_record_offset":32000,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":580,"record_offset":37760,"parent_record_offset":37736,"ancestor_record_offset":32000,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":580,"record_offset":37108,"parent_record_offset":37084,"ancestor_record_offset":32000,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":580,"record_offset":34796,"parent_record_offset":34772,"ancestor_record_offset":32000,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":580,"record_offset":32356,"parent_record_offset":32324,"ancestor_record_offset":32000,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":580,"record_offset":37736,"parent_record_offset":37628,"ancestor_record_offset":32000,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":580,"record_offset":32688,"parent_record_offset":32656,"ancestor_record_offset":32000,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":580,"record_offset":37140,"parent_record_offset":37108,"ancestor_record_offset":32000,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":37676,"parent_record_offset":37652,"ancestor_record_offset":32000,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":33012,"parent_record_offset":32760,"ancestor_record_offset":32000,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":37972,"parent_record_offset":37216,"ancestor_record_offset":32000,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":32484,"parent_record_offset":32420,"ancestor_record_offset":32000,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":32760,"parent_record_offset":32720,"ancestor_record_offset":32000,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":37652,"parent_record_offset":37628,"ancestor_record_offset":32000,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":37192,"parent_record_offset":37168,"ancestor_record_offset":32000,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":37168,"parent_record_offset":37108,"ancestor_record_offset":32000,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":32388,"parent_record_offset":32356,"ancestor_record_offset":32000,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":40712,"parent_record_offset":40648,"ancestor_record_offset":32000,"inlinee":8402,"source_line":2675,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":32456,"parent_record_offset":32420,"ancestor_record_offset":32000,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":37216,"parent_record_offset":37192,"ancestor_record_offset":32000,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":32420,"parent_record_offset":32388,"ancestor_record_offset":32000,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":37300,"parent_record_offset":37216,"ancestor_record_offset":32000,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":32720,"parent_record_offset":32688,"ancestor_record_offset":32000,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":37628,"parent_record_offset":37216,"ancestor_record_offset":32000,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::get_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":580,"record_offset":36192,"parent_record_offset":36168,"ancestor_record_offset":32000,"inlinee":31960,"source_line":154,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":580,"record_offset":36168,"parent_record_offset":34704,"ancestor_record_offset":32000,"inlinee":31959,"source_line":250,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::uniform_range_f32_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":580,"record_offset":42344,"parent_record_offset":42320,"ancestor_record_offset":32000,"inlinee":31958,"source_line":275,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] perchance::Rng::next_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":580,"record_offset":36228,"parent_record_offset":36192,"ancestor_record_offset":32000,"inlinee":31962,"source_line":72,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] perchance::Rng::next_u64
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":580,"record_offset":36252,"parent_record_offset":36228,"ancestor_record_offset":32000,"inlinee":31963,"source_line":77,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":580,"record_offset":37084,"parent_record_offset":37048,"ancestor_record_offset":32000,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":580,"record_offset":34772,"parent_record_offset":34704,"ancestor_record_offset":32000,"inlinee":12122,"source_line":708,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":580,"record_offset":32324,"parent_record_offset":32292,"ancestor_record_offset":32000,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":580,"record_offset":32292,"parent_record_offset":32260,"ancestor_record_offset":32000,"inlinee":8400,"source_line":543,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":580,"record_offset":32656,"parent_record_offset":32624,"ancestor_record_offset":32000,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":580,"record_offset":33780,"parent_record_offset":33424,"ancestor_record_offset":32000,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":580,"record_offset":38532,"parent_record_offset":38500,"ancestor_record_offset":32000,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":38184,"parent_record_offset":38144,"ancestor_record_offset":32000,"inlinee":26417,"source_line":681,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":36556,"parent_record_offset":36532,"ancestor_record_offset":32000,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":36532,"parent_record_offset":34704,"ancestor_record_offset":32000,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":38024,"parent_record_offset":37048,"ancestor_record_offset":32000,"inlinee":28987,"source_line":165,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":36580,"parent_record_offset":36556,"ancestor_record_offset":32000,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":36692,"parent_record_offset":36556,"ancestor_record_offset":32000,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":580,"record_offset":36384,"parent_record_offset":34704,"ancestor_record_offset":32000,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":34260,"parent_record_offset":34116,"ancestor_record_offset":32000,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":34444,"parent_record_offset":34088,"ancestor_record_offset":32000,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":38396,"parent_record_offset":38144,"ancestor_record_offset":32000,"inlinee":27588,"source_line":68,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":34088,"parent_record_offset":33272,"ancestor_record_offset":32000,"inlinee":26199,"source_line":184,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":34116,"parent_record_offset":34088,"ancestor_record_offset":32000,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_spawner::ivy_spawner::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9076
// 首条位置: {"module":580,"inlinee":36982}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":41216,"parent_record_offset":41136,"ancestor_record_offset":41136,"inlinee":26483,"source_line":585,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":41184,"parent_record_offset":41136,"ancestor_record_offset":41136,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":41304,"parent_record_offset":41244,"ancestor_record_offset":41136,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":41336,"parent_record_offset":41244,"ancestor_record_offset":41136,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":580,"record_offset":41244,"parent_record_offset":41216,"ancestor_record_offset":41136,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_spawner::try_random_ivy_root
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9073
// 首条位置: {"module":580,"inlinee":36979}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::nth
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":580,"record_offset":35088,"parent_record_offset":34704,"ancestor_record_offset":34704,"inlinee":6943,"source_line":384,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":37016,"parent_record_offset":34704,"ancestor_record_offset":34704,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] perchance::PerchanceContext::uniform_f32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":580,"record_offset":36168,"parent_record_offset":34704,"ancestor_record_offset":34704,"inlinee":31959,"source_line":250,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":580,"record_offset":34772,"parent_record_offset":34704,"ancestor_record_offset":34704,"inlinee":12122,"source_line":708,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":580,"record_offset":34828,"parent_record_offset":34704,"ancestor_record_offset":34704,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":36532,"parent_record_offset":34704,"ancestor_record_offset":34704,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":580,"record_offset":36384,"parent_record_offset":34704,"ancestor_record_offset":34704,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":580,"record_offset":36608,"parent_record_offset":36556,"ancestor_record_offset":34704,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":580,"record_offset":36464,"parent_record_offset":36440,"ancestor_record_offset":34704,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":580,"record_offset":36488,"parent_record_offset":36464,"ancestor_record_offset":34704,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":580,"record_offset":35008,"parent_record_offset":34984,"ancestor_record_offset":34704,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::rotate_right
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":580,"record_offset":36336,"parent_record_offset":36312,"ancestor_record_offset":34704,"inlinee":12398,"source_line":2047,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":580,"record_offset":35120,"parent_record_offset":35088,"ancestor_record_offset":34704,"inlinee":6891,"source_line":306,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::advance_by::impl$1::spec_advance_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":580,"record_offset":35144,"parent_record_offset":35120,"ancestor_record_offset":34704,"inlinee":6889,"source_line":325,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":580,"record_offset":35172,"parent_record_offset":35144,"ancestor_record_offset":34704,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$10::wrapping_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":36284,"parent_record_offset":36252,"ancestor_record_offset":34704,"inlinee":6064,"source_line":2532,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_right
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":580,"record_offset":36312,"parent_record_offset":36252,"ancestor_record_offset":34704,"inlinee":12397,"source_line":414,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":580,"record_offset":35528,"parent_record_offset":35496,"ancestor_record_offset":34704,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":580,"record_offset":35496,"parent_record_offset":35464,"ancestor_record_offset":34704,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":36968,"parent_record_offset":36944,"ancestor_record_offset":34704,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":36884,"parent_record_offset":36860,"ancestor_record_offset":34704,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":580,"record_offset":36440,"parent_record_offset":36416,"ancestor_record_offset":34704,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":580,"record_offset":36416,"parent_record_offset":36384,"ancestor_record_offset":34704,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":580,"record_offset":35040,"parent_record_offset":34956,"ancestor_record_offset":34704,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":580,"record_offset":35716,"parent_record_offset":35684,"ancestor_record_offset":34704,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":36632,"parent_record_offset":36608,"ancestor_record_offset":34704,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":36656,"parent_record_offset":36632,"ancestor_record_offset":34704,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":36832,"parent_record_offset":36804,"ancestor_record_offset":34704,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":36860,"parent_record_offset":36832,"ancestor_record_offset":34704,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":36944,"parent_record_offset":36832,"ancestor_record_offset":34704,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":580,"record_offset":35408,"parent_record_offset":35356,"ancestor_record_offset":34704,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":580,"record_offset":35464,"parent_record_offset":35408,"ancestor_record_offset":34704,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":580,"record_offset":35576,"parent_record_offset":35356,"ancestor_record_offset":34704,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":580,"record_offset":35356,"parent_record_offset":35292,"ancestor_record_offset":34704,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":580,"record_offset":34984,"parent_record_offset":34956,"ancestor_record_offset":34704,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":580,"record_offset":34796,"parent_record_offset":34772,"ancestor_record_offset":34704,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":580,"record_offset":34852,"parent_record_offset":34828,"ancestor_record_offset":34704,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":580,"record_offset":35232,"parent_record_offset":35204,"ancestor_record_offset":34704,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":35684,"parent_record_offset":35292,"ancestor_record_offset":34704,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":34956,"parent_record_offset":34900,"ancestor_record_offset":34704,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":35292,"parent_record_offset":35260,"ancestor_record_offset":34704,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":34876,"parent_record_offset":34852,"ancestor_record_offset":34704,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":34928,"parent_record_offset":34900,"ancestor_record_offset":34704,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":34900,"parent_record_offset":34876,"ancestor_record_offset":34704,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":580,"record_offset":35260,"parent_record_offset":35232,"ancestor_record_offset":34704,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] perchance::PerchanceContext::get_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":580,"record_offset":36192,"parent_record_offset":36168,"ancestor_record_offset":34704,"inlinee":31960,"source_line":154,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] perchance::Rng::next_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":580,"record_offset":36228,"parent_record_offset":36192,"ancestor_record_offset":34704,"inlinee":31962,"source_line":72,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] perchance::Rng::next_u64
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"perchance","version":"0.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/perchance-0.4.0/src/lib.rs
//     evidence={"module":580,"record_offset":36252,"parent_record_offset":36228,"ancestor_record_offset":34704,"inlinee":31963,"source_line":77,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":580,"record_offset":35204,"parent_record_offset":35172,"ancestor_record_offset":34704,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":36556,"parent_record_offset":36532,"ancestor_record_offset":34704,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":36580,"parent_record_offset":36556,"ancestor_record_offset":34704,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":580,"record_offset":36692,"parent_record_offset":36556,"ancestor_record_offset":34704,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":36804,"parent_record_offset":36556,"ancestor_record_offset":34704,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_spawner::try_ivy_root_next_to_cursor
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 243
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9072
// 首条位置: {"module":580,"inlinee":36978}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":34056,"parent_record_offset":33272,"ancestor_record_offset":33272,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":580,"record_offset":34028,"parent_record_offset":33272,"ancestor_record_offset":33272,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":580,"record_offset":33328,"parent_record_offset":33272,"ancestor_record_offset":33272,"inlinee":28649,"source_line":35,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":580,"record_offset":33384,"parent_record_offset":33272,"ancestor_record_offset":33272,"inlinee":33015,"source_line":130,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":34088,"parent_record_offset":33272,"ancestor_record_offset":33272,"inlinee":26199,"source_line":184,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":34412,"parent_record_offset":34088,"ancestor_record_offset":33272,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":34380,"parent_record_offset":34116,"ancestor_record_offset":33272,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":33804,"parent_record_offset":33780,"ancestor_record_offset":33272,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":33592,"parent_record_offset":33560,"ancestor_record_offset":33272,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":34516,"parent_record_offset":34492,"ancestor_record_offset":33272,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":33500,"parent_record_offset":33472,"ancestor_record_offset":33272,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":34172,"parent_record_offset":34148,"ancestor_record_offset":33272,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":33992,"parent_record_offset":33384,"ancestor_record_offset":33272,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":580,"record_offset":33352,"parent_record_offset":33328,"ancestor_record_offset":33272,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":580,"record_offset":33928,"parent_record_offset":33384,"ancestor_record_offset":33272,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":580,"record_offset":33956,"parent_record_offset":33928,"ancestor_record_offset":33272,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":580,"record_offset":33652,"parent_record_offset":33424,"ancestor_record_offset":33272,"inlinee":33017,"source_line":538,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::grid_coord_to_cell_index
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":580,"record_offset":33896,"parent_record_offset":33384,"ancestor_record_offset":33272,"inlinee":33018,"source_line":521,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::world_space_to_grid_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs
//     evidence={"module":580,"record_offset":33424,"parent_record_offset":33384,"ancestor_record_offset":33272,"inlinee":33016,"source_line":505,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":34288,"parent_record_offset":34260,"ancestor_record_offset":33272,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":34468,"parent_record_offset":34444,"ancestor_record_offset":33272,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":34492,"parent_record_offset":34468,"ancestor_record_offset":33272,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":33692,"parent_record_offset":33424,"ancestor_record_offset":33272,"inlinee":8895,"source_line":1124,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":33560,"parent_record_offset":33424,"ancestor_record_offset":33272,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":34148,"parent_record_offset":34116,"ancestor_record_offset":33272,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":33472,"parent_record_offset":33424,"ancestor_record_offset":33272,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":580,"record_offset":33780,"parent_record_offset":33424,"ancestor_record_offset":33272,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":34260,"parent_record_offset":34116,"ancestor_record_offset":33272,"inlinee":34225,"source_line":83,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":34444,"parent_record_offset":34088,"ancestor_record_offset":33272,"inlinee":14719,"source_line":78,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":580,"record_offset":34116,"parent_record_offset":34088,"ancestor_record_offset":33272,"inlinee":26200,"source_line":152,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ivy::ivy_spawner::is_point_inside_wall_segment
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 292
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9077
// 首条位置: {"module":580,"inlinee":36983}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":580,"record_offset":43940,"parent_record_offset":43448,"ancestor_record_offset":43448,"inlinee":11719,"source_line":788,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":44808,"parent_record_offset":43448,"ancestor_record_offset":43448,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":43556,"parent_record_offset":43448,"ancestor_record_offset":43448,"inlinee":11326,"source_line":531,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":43772,"parent_record_offset":43448,"ancestor_record_offset":43448,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":43500,"parent_record_offset":43448,"ancestor_record_offset":43448,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":43828,"parent_record_offset":43448,"ancestor_record_offset":43448,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":580,"record_offset":43964,"parent_record_offset":43940,"ancestor_record_offset":43448,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":580,"record_offset":44468,"parent_record_offset":44048,"ancestor_record_offset":43448,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":580,"record_offset":44376,"parent_record_offset":44320,"ancestor_record_offset":43448,"inlinee":11729,"source_line":22,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":580,"record_offset":44144,"parent_record_offset":44120,"ancestor_record_offset":43448,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":580,"record_offset":44176,"parent_record_offset":44092,"ancestor_record_offset":43448,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":580,"record_offset":44240,"parent_record_offset":44048,"ancestor_record_offset":43448,"inlinee":11730,"source_line":61,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":580,"record_offset":43696,"parent_record_offset":43672,"ancestor_record_offset":43448,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":43796,"parent_record_offset":43772,"ancestor_record_offset":43448,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":43736,"parent_record_offset":43580,"ancestor_record_offset":43448,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":43852,"parent_record_offset":43828,"ancestor_record_offset":43448,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":580,"record_offset":43524,"parent_record_offset":43500,"ancestor_record_offset":43448,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":580,"record_offset":44048,"parent_record_offset":43940,"ancestor_record_offset":43448,"inlinee":11724,"source_line":865,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":580,"record_offset":44496,"parent_record_offset":44048,"ancestor_record_offset":43448,"inlinee":11725,"source_line":274,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":580,"record_offset":44012,"parent_record_offset":43988,"ancestor_record_offset":43448,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":580,"record_offset":43988,"parent_record_offset":43964,"ancestor_record_offset":43448,"inlinee":11721,"source_line":2114,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":43640,"parent_record_offset":43608,"ancestor_record_offset":43448,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":43608,"parent_record_offset":43580,"ancestor_record_offset":43448,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":580,"record_offset":43580,"parent_record_offset":43556,"ancestor_record_offset":43448,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":580,"record_offset":44120,"parent_record_offset":44092,"ancestor_record_offset":43448,"inlinee":11728,"source_line":42,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":580,"record_offset":44092,"parent_record_offset":44048,"ancestor_record_offset":43448,"inlinee":11727,"source_line":66,"archive":"_pdb/C/modules/00580.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":580,"record_offset":43672,"parent_record_offset":43640,"ancestor_record_offset":43448,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00580.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_spawner::is_point_inside_wall_segment
// 信息 ID: e20233111f898f521d99dc44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=580, inline_record=43448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_spawner::ivy_spawner
// 信息 ID: c30d432f9a97d9194659b6d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 53a1173d2944984b40adcdca: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>)> > @ 0x19165e4 -> 0xbc0c80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>),tuple$<>,void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>)> @ 0x19bcd37 -> 0xbc0c80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_spawner::ivy_spawner::closure$0
// 信息 ID: 3545c82f17f97cc51554ff92
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ivy::ivy_spawner::ivy_spawner", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=580, inline_record=41136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_spawner::try_ivy_root_next_to_cursor
// 信息 ID: 3361d0c47afa53c7a6b8f805
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=580, inline_record=33272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ivy::ivy_spawner::try_random_ivy_root
// 信息 ID: 9f60d45b669a5a0e7f066f81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=580, inline_record=34704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
