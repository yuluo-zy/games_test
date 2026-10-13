// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/ui_move_wall_group.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 1a2e565c2b5acf83de2d8b29a4d4c3b039f256432510e1b518c8a8a29b6d2add
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ui_move_wall_group.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 536, 561, 1643, 1682, 1718, 1823
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::impl$3::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1459F
// 首条位置: {"module":1718,"inlinee":83359}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::WallGroupTransformCache::cursor_position
// 记录形式: compiled_procedure; instances=1
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":561,"record_offset":46896,"rva":11621120,"va":5380330240}
// 编译实例 RVAs: 0xb15300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":561,"record_offset":47248,"parent_record_offset":46896,"ancestor_record_offset":46896,"inlinee":28649,"source_line":35,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::mouse_terrain_raycast::CameraCursorRay::intersect_with_xz_plane
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/mouse_terrain_raycast.rs
//     evidence={"module":561,"record_offset":47068,"parent_record_offset":46896,"ancestor_record_offset":46896,"inlinee":34363,"source_line":27,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":47192,"parent_record_offset":47168,"ancestor_record_offset":46896,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":47116,"parent_record_offset":47092,"ancestor_record_offset":46896,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":47272,"parent_record_offset":47248,"ancestor_record_offset":46896,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":561,"record_offset":47092,"parent_record_offset":47068,"ancestor_record_offset":46896,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":561,"record_offset":47168,"parent_record_offset":47068,"ancestor_record_offset":46896,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::WallGroupTransformCache::apply
// 记录形式: compiled_procedure; instances=1
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":561,"record_offset":50740,"rva":11622096,"va":5380331216}
// 编译实例 RVAs: 0xb156d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4652936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380331216,"raw_hex":"e87800b9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380331619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-4653301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380331216,"raw_hex":"e80bffb8ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380332000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11609419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380331216,"raw_hex":"e8b5da4eff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380331606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11092292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380331216,"raw_hex":"e8bcbe56ff","size":5,"soft_fail":false,"target_va":5369240528,"va":5380332815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409897d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1622971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380331216,"raw_hex":"e8453ce7ff","size":5,"soft_fail":false,"target_va":5378709456,"va":5380332422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160c100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11495079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380331216,"raw_hex":"e8a766af00","size":5,"soft_fail":false,"target_va":5391827200,"va":5380332116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36018190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380331216,"raw_hex":"e80e982502","size":5,"soft_fail":false,"target_va":5416350384,"va":5380332189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36018836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380331216,"raw_hex":"e8949a2502","size":5,"soft_fail":false,"target_va":5416350608,"va":5380331767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36725526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380331216,"raw_hex":"e816633002","size":5,"soft_fail":false,"target_va":5417057600,"va":5380332069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36288444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380331216,"raw_hex":"e8bcb72902","size":5,"soft_fail":false,"target_va":5416620794,"va":5380332345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::update_wall_state
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t972438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380331216,"raw_hex":"e896d60e00","size":5,"soft_fail":false,"target_va":5381304336,"va":5380331893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":561,"record_offset":52568,"parent_record_offset":50740,"ancestor_record_offset":50740,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":51148,"parent_record_offset":50740,"ancestor_record_offset":50740,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":51008,"parent_record_offset":50740,"ancestor_record_offset":50740,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":53328,"parent_record_offset":50740,"ancestor_record_offset":50740,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":561,"record_offset":50924,"parent_record_offset":50740,"ancestor_record_offset":50740,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_move_wall_group::WallGroupMember::shape_moved
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":51504,"parent_record_offset":50740,"ancestor_record_offset":50740,"inlinee":35248,"source_line":374,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_move_wall_group::WallGroupTransformCache::notify_circles_rotated
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":54632,"parent_record_offset":50740,"ancestor_record_offset":50740,"inlinee":35252,"source_line":110,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":51876,"parent_record_offset":51852,"ancestor_record_offset":50740,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":51900,"parent_record_offset":51876,"ancestor_record_offset":50740,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":53196,"parent_record_offset":53172,"ancestor_record_offset":50740,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":53220,"parent_record_offset":53196,"ancestor_record_offset":50740,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":51924,"parent_record_offset":51900,"ancestor_record_offset":50740,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":53244,"parent_record_offset":53220,"ancestor_record_offset":50740,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":51852,"parent_record_offset":51816,"ancestor_record_offset":50740,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":53172,"parent_record_offset":53084,"ancestor_record_offset":50740,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":52848,"parent_record_offset":52816,"ancestor_record_offset":50740,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":51748,"parent_record_offset":51716,"ancestor_record_offset":50740,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":53112,"parent_record_offset":53084,"ancestor_record_offset":50740,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":53084,"parent_record_offset":53060,"ancestor_record_offset":50740,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":52896,"parent_record_offset":52872,"ancestor_record_offset":50740,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":52872,"parent_record_offset":52848,"ancestor_record_offset":50740,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":51816,"parent_record_offset":51780,"ancestor_record_offset":50740,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":51780,"parent_record_offset":51748,"ancestor_record_offset":50740,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":53060,"parent_record_offset":53036,"ancestor_record_offset":50740,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":52816,"parent_record_offset":52764,"ancestor_record_offset":50740,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":52148,"parent_record_offset":52124,"ancestor_record_offset":50740,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":54820,"parent_record_offset":54632,"ancestor_record_offset":50740,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":52724,"parent_record_offset":52676,"ancestor_record_offset":50740,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":52764,"parent_record_offset":52724,"ancestor_record_offset":50740,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":51684,"parent_record_offset":51644,"ancestor_record_offset":50740,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":51716,"parent_record_offset":51684,"ancestor_record_offset":50740,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":51580,"parent_record_offset":51548,"ancestor_record_offset":50740,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":51032,"parent_record_offset":51008,"ancestor_record_offset":50740,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":52464,"parent_record_offset":52440,"ancestor_record_offset":50740,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":561,"record_offset":54108,"parent_record_offset":53376,"ancestor_record_offset":50740,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":561,"record_offset":53376,"parent_record_offset":53352,"ancestor_record_offset":50740,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":561,"record_offset":54328,"parent_record_offset":54304,"ancestor_record_offset":50740,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":561,"record_offset":51316,"parent_record_offset":51180,"ancestor_record_offset":50740,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":561,"record_offset":54180,"parent_record_offset":54156,"ancestor_record_offset":50740,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":561,"record_offset":52124,"parent_record_offset":51644,"ancestor_record_offset":50740,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":561,"record_offset":51612,"parent_record_offset":51580,"ancestor_record_offset":50740,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":561,"record_offset":51644,"parent_record_offset":51612,"ancestor_record_offset":50740,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":561,"record_offset":52648,"parent_record_offset":52604,"ancestor_record_offset":50740,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":561,"record_offset":52604,"parent_record_offset":52568,"ancestor_record_offset":50740,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":561,"record_offset":54848,"parent_record_offset":54632,"ancestor_record_offset":50740,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":561,"record_offset":52676,"parent_record_offset":52604,"ancestor_record_offset":50740,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":561,"record_offset":52392,"parent_record_offset":52368,"ancestor_record_offset":50740,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":54736,"parent_record_offset":54632,"ancestor_record_offset":50740,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":52196,"parent_record_offset":52172,"ancestor_record_offset":50740,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":51984,"parent_record_offset":51644,"ancestor_record_offset":50740,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":52248,"parent_record_offset":52220,"ancestor_record_offset":50740,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":51088,"parent_record_offset":51008,"ancestor_record_offset":50740,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":51112,"parent_record_offset":51088,"ancestor_record_offset":50740,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":51180,"parent_record_offset":51148,"ancestor_record_offset":50740,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":561,"record_offset":54156,"parent_record_offset":54132,"ancestor_record_offset":50740,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":561,"record_offset":54132,"parent_record_offset":54108,"ancestor_record_offset":50740,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":51548,"parent_record_offset":51504,"ancestor_record_offset":50740,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":52172,"parent_record_offset":52148,"ancestor_record_offset":50740,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":52440,"parent_record_offset":52248,"ancestor_record_offset":50740,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":53140,"parent_record_offset":53112,"ancestor_record_offset":50740,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":51464,"parent_record_offset":51180,"ancestor_record_offset":50740,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":561,"record_offset":52080,"parent_record_offset":52056,"ancestor_record_offset":50740,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":52964,"parent_record_offset":52764,"ancestor_record_offset":50740,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":50972,"parent_record_offset":50948,"ancestor_record_offset":50740,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":52056,"parent_record_offset":52032,"ancestor_record_offset":50740,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":51284,"parent_record_offset":51216,"ancestor_record_offset":50740,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":52032,"parent_record_offset":52008,"ancestor_record_offset":50740,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":51404,"parent_record_offset":51316,"ancestor_record_offset":50740,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":51252,"parent_record_offset":51216,"ancestor_record_offset":50740,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":51428,"parent_record_offset":51404,"ancestor_record_offset":50740,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":52488,"parent_record_offset":52464,"ancestor_record_offset":50740,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":561,"record_offset":50948,"parent_record_offset":50924,"ancestor_record_offset":50740,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":52220,"parent_record_offset":52196,"ancestor_record_offset":50740,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":51216,"parent_record_offset":51180,"ancestor_record_offset":50740,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":52008,"parent_record_offset":51984,"ancestor_record_offset":50740,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_move_wall_group::impl$1::shape_moved::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":52272,"parent_record_offset":52248,"ancestor_record_offset":50740,"inlinee":35250,"source_line":380,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::rotation_angle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":54680,"parent_record_offset":54632,"ancestor_record_offset":50740,"inlinee":35208,"source_line":101,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::transform_vector
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":52296,"parent_record_offset":52272,"ancestor_record_offset":50740,"inlinee":35206,"source_line":141,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine2::Affine2::transform_vector2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine2.rs
//     evidence={"module":561,"record_offset":52320,"parent_record_offset":52296,"ancestor_record_offset":50740,"inlinee":35207,"source_line":279,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":52368,"parent_record_offset":52344,"ancestor_record_offset":50740,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":52344,"parent_record_offset":52320,"ancestor_record_offset":50740,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":561,"record_offset":54704,"parent_record_offset":54680,"ancestor_record_offset":50740,"inlinee":11245,"source_line":880,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::WallGroupTransformCache::notify_shape_moved
// 记录形式: compiled_procedure; instances=1
// 源码行: 99
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":561,"record_offset":47308,"rva":11621200,"va":5380330320}
// 编译实例 RVAs: 0xb15350
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4651923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380330320,"raw_hex":"e86d04b9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380330606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4652241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380330320,"raw_hex":"e82f03b9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380330940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11608406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380330320,"raw_hex":"e8aade4eff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380330593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160c100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11496395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380330320,"raw_hex":"e8cb6baf00","size":5,"soft_fail":false,"target_va":5391827200,"va":5380330800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36019519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380330320,"raw_hex":"e83f9d2502","size":5,"soft_fail":false,"target_va":5416350384,"va":5380330860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36019875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380330320,"raw_hex":"e8a39e2502","size":5,"soft_fail":false,"target_va":5416350608,"va":5380330728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36726835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380330320,"raw_hex":"e833683002","size":5,"soft_fail":false,"target_va":5417057600,"va":5380330760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36289803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380330320,"raw_hex":"e80bbd2902","size":5,"soft_fail":false,"target_va":5416620794,"va":5380330986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":47496,"parent_record_offset":47308,"ancestor_record_offset":47308,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":561,"record_offset":47672,"parent_record_offset":47308,"ancestor_record_offset":47308,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":47608,"parent_record_offset":47308,"ancestor_record_offset":47308,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_move_wall_group::WallGroupMember::shape_moved
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":48440,"parent_record_offset":47308,"ancestor_record_offset":47308,"inlinee":35248,"source_line":374,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_move_wall_group::WallGroupTransformCache::notify_circles_rotated
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":49488,"parent_record_offset":47308,"ancestor_record_offset":47308,"inlinee":35252,"source_line":110,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":48796,"parent_record_offset":48772,"ancestor_record_offset":47308,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":48820,"parent_record_offset":48796,"ancestor_record_offset":47308,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":48280,"parent_record_offset":48256,"ancestor_record_offset":47308,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":48304,"parent_record_offset":48280,"ancestor_record_offset":47308,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":48844,"parent_record_offset":48820,"ancestor_record_offset":47308,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":48328,"parent_record_offset":48304,"ancestor_record_offset":47308,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":48772,"parent_record_offset":48736,"ancestor_record_offset":47308,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":48256,"parent_record_offset":48168,"ancestor_record_offset":47308,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":47932,"parent_record_offset":47900,"ancestor_record_offset":47308,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":48668,"parent_record_offset":48636,"ancestor_record_offset":47308,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":48196,"parent_record_offset":48168,"ancestor_record_offset":47308,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":48168,"parent_record_offset":48144,"ancestor_record_offset":47308,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":47980,"parent_record_offset":47956,"ancestor_record_offset":47308,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":47956,"parent_record_offset":47932,"ancestor_record_offset":47308,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":48736,"parent_record_offset":48700,"ancestor_record_offset":47308,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":48700,"parent_record_offset":48668,"ancestor_record_offset":47308,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":48144,"parent_record_offset":48120,"ancestor_record_offset":47308,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":47900,"parent_record_offset":47844,"ancestor_record_offset":47308,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":49068,"parent_record_offset":49044,"ancestor_record_offset":47308,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":49672,"parent_record_offset":49488,"ancestor_record_offset":47308,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":47804,"parent_record_offset":47756,"ancestor_record_offset":47308,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":47844,"parent_record_offset":47804,"ancestor_record_offset":47308,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":48604,"parent_record_offset":48564,"ancestor_record_offset":47308,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":48636,"parent_record_offset":48604,"ancestor_record_offset":47308,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":48508,"parent_record_offset":48480,"ancestor_record_offset":47308,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":49384,"parent_record_offset":49360,"ancestor_record_offset":47308,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":561,"record_offset":49044,"parent_record_offset":48564,"ancestor_record_offset":47308,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":561,"record_offset":48536,"parent_record_offset":48508,"ancestor_record_offset":47308,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":561,"record_offset":48564,"parent_record_offset":48536,"ancestor_record_offset":47308,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":561,"record_offset":48404,"parent_record_offset":47708,"ancestor_record_offset":47308,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":561,"record_offset":49728,"parent_record_offset":49700,"ancestor_record_offset":47308,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":561,"record_offset":49700,"parent_record_offset":49488,"ancestor_record_offset":47308,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":561,"record_offset":47708,"parent_record_offset":47672,"ancestor_record_offset":47308,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":561,"record_offset":47756,"parent_record_offset":47708,"ancestor_record_offset":47308,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":561,"record_offset":49312,"parent_record_offset":49288,"ancestor_record_offset":47308,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":49588,"parent_record_offset":49488,"ancestor_record_offset":47308,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":49116,"parent_record_offset":49092,"ancestor_record_offset":47308,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":48904,"parent_record_offset":48564,"ancestor_record_offset":47308,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":49168,"parent_record_offset":49140,"ancestor_record_offset":47308,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":48480,"parent_record_offset":48440,"ancestor_record_offset":47308,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":49092,"parent_record_offset":49068,"ancestor_record_offset":47308,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":49360,"parent_record_offset":49168,"ancestor_record_offset":47308,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":48224,"parent_record_offset":48196,"ancestor_record_offset":47308,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":561,"record_offset":49000,"parent_record_offset":48976,"ancestor_record_offset":47308,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":48048,"parent_record_offset":47844,"ancestor_record_offset":47308,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":47568,"parent_record_offset":47544,"ancestor_record_offset":47308,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":48976,"parent_record_offset":48952,"ancestor_record_offset":47308,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":48952,"parent_record_offset":48928,"ancestor_record_offset":47308,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":47636,"parent_record_offset":47608,"ancestor_record_offset":47308,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":48020,"parent_record_offset":47844,"ancestor_record_offset":47308,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":561,"record_offset":47520,"parent_record_offset":47496,"ancestor_record_offset":47308,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":561,"record_offset":47544,"parent_record_offset":47520,"ancestor_record_offset":47308,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":49140,"parent_record_offset":49116,"ancestor_record_offset":47308,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":48928,"parent_record_offset":48904,"ancestor_record_offset":47308,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_move_wall_group::impl$1::shape_moved::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":49192,"parent_record_offset":49168,"ancestor_record_offset":47308,"inlinee":35250,"source_line":380,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::rotation_angle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":49532,"parent_record_offset":49488,"ancestor_record_offset":47308,"inlinee":35208,"source_line":101,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::transform_vector
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":49216,"parent_record_offset":49192,"ancestor_record_offset":47308,"inlinee":35206,"source_line":141,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine2::Affine2::transform_vector2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine2.rs
//     evidence={"module":561,"record_offset":49240,"parent_record_offset":49216,"ancestor_record_offset":47308,"inlinee":35207,"source_line":279,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":49288,"parent_record_offset":49264,"ancestor_record_offset":47308,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":49264,"parent_record_offset":49240,"ancestor_record_offset":47308,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":561,"record_offset":49556,"parent_record_offset":49532,"ancestor_record_offset":47308,"inlinee":11245,"source_line":880,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::WallGroupTransformCache::notify_circles_rotated
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 110
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89B4
// 首条位置: {"module":561,"inlinee":35252}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":49672,"parent_record_offset":49488,"ancestor_record_offset":49488,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":561,"record_offset":49700,"parent_record_offset":49488,"ancestor_record_offset":49488,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":49588,"parent_record_offset":49488,"ancestor_record_offset":49488,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":49616,"parent_record_offset":49488,"ancestor_record_offset":49488,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":50440,"parent_record_offset":49488,"ancestor_record_offset":49488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::rotation_angle
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":49532,"parent_record_offset":49488,"ancestor_record_offset":49488,"inlinee":35208,"source_line":101,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":50308,"parent_record_offset":50284,"ancestor_record_offset":49488,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":50332,"parent_record_offset":50308,"ancestor_record_offset":49488,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":50356,"parent_record_offset":50332,"ancestor_record_offset":49488,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":50284,"parent_record_offset":50196,"ancestor_record_offset":49488,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":49960,"parent_record_offset":49928,"ancestor_record_offset":49488,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":50224,"parent_record_offset":50196,"ancestor_record_offset":49488,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":50196,"parent_record_offset":50172,"ancestor_record_offset":49488,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":50008,"parent_record_offset":49984,"ancestor_record_offset":49488,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":49984,"parent_record_offset":49960,"ancestor_record_offset":49488,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":50172,"parent_record_offset":50148,"ancestor_record_offset":49488,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":49928,"parent_record_offset":49876,"ancestor_record_offset":49488,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":49840,"parent_record_offset":49796,"ancestor_record_offset":49488,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":49876,"parent_record_offset":49840,"ancestor_record_offset":49488,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":49640,"parent_record_offset":49616,"ancestor_record_offset":49488,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":561,"record_offset":49768,"parent_record_offset":49728,"ancestor_record_offset":49488,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":561,"record_offset":49728,"parent_record_offset":49700,"ancestor_record_offset":49488,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":561,"record_offset":49796,"parent_record_offset":49728,"ancestor_record_offset":49488,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":50252,"parent_record_offset":50224,"ancestor_record_offset":49488,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":50076,"parent_record_offset":49876,"ancestor_record_offset":49488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":50048,"parent_record_offset":49876,"ancestor_record_offset":49488,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":561,"record_offset":49556,"parent_record_offset":49532,"ancestor_record_offset":49488,"inlinee":11245,"source_line":880,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::impl$0::notify_circles_rotated::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DC1
// 首条位置: {"module":1643,"inlinee":81345}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::WallGroupTransformCache::target_states
// 记录形式: compiled_procedure; instances=1
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":561,"record_offset":43500,"rva":11619920,"va":5380329040}
// 编译实例 RVAs: 0xb14e50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4650526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380329040,"raw_hex":"e8e209b9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380329209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4651109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380329040,"raw_hex":"e89b07b9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380329808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11607009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380329040,"raw_hex":"e81fe44eff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380329196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d590
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1735880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380329040,"raw_hex":"e83883e5ff","size":5,"soft_fail":false,"target_va":5378594192,"va":5380330067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141601bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11455573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380329040,"raw_hex":"e855ccae00","size":5,"soft_fail":false,"target_va":5391784880,"va":5380329302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141606ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11476898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380329040,"raw_hex":"e8a21faf00","size":5,"soft_fail":false,"target_va":5391806176,"va":5380329273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141810d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13613912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380329040,"raw_hex":"e858bbcf00","size":5,"soft_fail":false,"target_va":5393943824,"va":5380329907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36728437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380329040,"raw_hex":"e8756e3002","size":5,"soft_fail":false,"target_va":5417057600,"va":5380329158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36291558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380329040,"raw_hex":"e8e6c32902","size":5,"soft_fail":false,"target_va":5416620794,"va":5380329231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36292218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380329040,"raw_hex":"e87ac62902","size":5,"soft_fail":false,"target_va":5416622240,"va":5380330017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":45656,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":45200,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":45072,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":44128,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":44932,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$55::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\internal_macros.rs
//     evidence={"module":561,"record_offset":45144,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":9788,"source_line":27,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":45804,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":561,"record_offset":44680,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::is_identity
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":43700,"parent_record_offset":43500,"ancestor_record_offset":43500,"inlinee":35247,"source_line":96,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":44444,"parent_record_offset":44420,"ancestor_record_offset":43500,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":44468,"parent_record_offset":44444,"ancestor_record_offset":43500,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":45988,"parent_record_offset":45964,"ancestor_record_offset":43500,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":46012,"parent_record_offset":45988,"ancestor_record_offset":43500,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":44492,"parent_record_offset":44468,"ancestor_record_offset":43500,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":46036,"parent_record_offset":46012,"ancestor_record_offset":43500,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":44420,"parent_record_offset":44388,"ancestor_record_offset":43500,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":45964,"parent_record_offset":45876,"ancestor_record_offset":43500,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":44336,"parent_record_offset":44308,"ancestor_record_offset":43500,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":45904,"parent_record_offset":45876,"ancestor_record_offset":43500,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":45876,"parent_record_offset":45852,"ancestor_record_offset":43500,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":44388,"parent_record_offset":44360,"ancestor_record_offset":43500,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":44360,"parent_record_offset":44336,"ancestor_record_offset":43500,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":45852,"parent_record_offset":45828,"ancestor_record_offset":43500,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":44576,"parent_record_offset":44552,"ancestor_record_offset":43500,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":44280,"parent_record_offset":44236,"ancestor_record_offset":43500,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":44308,"parent_record_offset":44280,"ancestor_record_offset":43500,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":44164,"parent_record_offset":44128,"ancestor_record_offset":43500,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":561,"record_offset":46140,"parent_record_offset":46116,"ancestor_record_offset":43500,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":561,"record_offset":46188,"parent_record_offset":46164,"ancestor_record_offset":43500,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":561,"record_offset":45588,"parent_record_offset":45484,"ancestor_record_offset":43500,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":561,"record_offset":44552,"parent_record_offset":44236,"ancestor_record_offset":43500,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":561,"record_offset":44200,"parent_record_offset":44164,"ancestor_record_offset":43500,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":561,"record_offset":44236,"parent_record_offset":44200,"ancestor_record_offset":43500,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":45028,"parent_record_offset":45004,"ancestor_record_offset":43500,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":45004,"parent_record_offset":44980,"ancestor_record_offset":43500,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":561,"record_offset":43836,"parent_record_offset":43812,"ancestor_record_offset":43500,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":44096,"parent_record_offset":43700,"ancestor_record_offset":43500,"inlinee":11731,"source_line":1569,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\panic.rs
//     evidence={"module":561,"record_offset":45736,"parent_record_offset":45688,"ancestor_record_offset":43500,"inlinee":11859,"source_line":166,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp::do_panic::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":561,"record_offset":45764,"parent_record_offset":45736,"ancestor_record_offset":43500,"inlinee":11861,"source_line":2434,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":44956,"parent_record_offset":44932,"ancestor_record_offset":43500,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":45524,"parent_record_offset":45484,"ancestor_record_offset":43500,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":45484,"parent_record_offset":45452,"ancestor_record_offset":43500,"inlinee":6658,"source_line":164,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":44980,"parent_record_offset":44956,"ancestor_record_offset":43500,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":45100,"parent_record_offset":45072,"ancestor_record_offset":43500,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":561,"record_offset":44812,"parent_record_offset":44788,"ancestor_record_offset":43500,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":44600,"parent_record_offset":44576,"ancestor_record_offset":43500,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":44624,"parent_record_offset":44600,"ancestor_record_offset":43500,"inlinee":8052,"source_line":841,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":45932,"parent_record_offset":45904,"ancestor_record_offset":43500,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":43896,"parent_record_offset":43872,"ancestor_record_offset":43500,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":43956,"parent_record_offset":43932,"ancestor_record_offset":43500,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":45828,"parent_record_offset":45804,"ancestor_record_offset":43500,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":44728,"parent_record_offset":44704,"ancestor_record_offset":43500,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":45616,"parent_record_offset":45588,"ancestor_record_offset":43500,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":561,"record_offset":44704,"parent_record_offset":44680,"ancestor_record_offset":43500,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::rotation_angle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":44040,"parent_record_offset":43700,"ancestor_record_offset":43500,"inlinee":35208,"source_line":101,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::transform_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":43732,"parent_record_offset":43700,"ancestor_record_offset":43500,"inlinee":35205,"source_line":137,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine2::Affine2::transform_point2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine2.rs
//     evidence={"module":561,"record_offset":43760,"parent_record_offset":43732,"ancestor_record_offset":43500,"inlinee":27806,"source_line":270,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":43812,"parent_record_offset":43788,"ancestor_record_offset":43500,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":43788,"parent_record_offset":43760,"ancestor_record_offset":43500,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":44008,"parent_record_offset":43984,"ancestor_record_offset":43500,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":43984,"parent_record_offset":43700,"ancestor_record_offset":43500,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":43872,"parent_record_offset":43760,"ancestor_record_offset":43500,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":43932,"parent_record_offset":43700,"ancestor_record_offset":43500,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":561,"record_offset":44064,"parent_record_offset":44040,"ancestor_record_offset":43500,"inlinee":11245,"source_line":880,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::impl$0::target_states::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 142
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14382
// 首条位置: {"module":1682,"inlinee":82818}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$45::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1682,"record_offset":88120,"parent_record_offset":88088,"ancestor_record_offset":88088,"inlinee":30580,"source_line":115,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":88668,"parent_record_offset":88644,"ancestor_record_offset":88088,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":88692,"parent_record_offset":88668,"ancestor_record_offset":88088,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":89832,"parent_record_offset":89808,"ancestor_record_offset":88088,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":89856,"parent_record_offset":89832,"ancestor_record_offset":88088,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":88716,"parent_record_offset":88692,"ancestor_record_offset":88088,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":89880,"parent_record_offset":89856,"ancestor_record_offset":88088,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":88644,"parent_record_offset":88464,"ancestor_record_offset":88088,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":89808,"parent_record_offset":89720,"ancestor_record_offset":88088,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":88404,"parent_record_offset":88376,"ancestor_record_offset":88088,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":89748,"parent_record_offset":89720,"ancestor_record_offset":88088,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":89720,"parent_record_offset":89696,"ancestor_record_offset":88088,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":88464,"parent_record_offset":88432,"ancestor_record_offset":88088,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":88432,"parent_record_offset":88404,"ancestor_record_offset":88088,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":89696,"parent_record_offset":89672,"ancestor_record_offset":88088,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":88500,"parent_record_offset":88464,"ancestor_record_offset":88088,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":88948,"parent_record_offset":88876,"ancestor_record_offset":88088,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":88332,"parent_record_offset":88260,"ancestor_record_offset":88088,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1682,"record_offset":88304,"parent_record_offset":88260,"ancestor_record_offset":88088,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1682,"record_offset":88376,"parent_record_offset":88332,"ancestor_record_offset":88088,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1682,"record_offset":88224,"parent_record_offset":88168,"ancestor_record_offset":88088,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1682,"record_offset":88260,"parent_record_offset":88224,"ancestor_record_offset":88088,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1682,"record_offset":88524,"parent_record_offset":88500,"ancestor_record_offset":88088,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1682,"record_offset":88604,"parent_record_offset":88548,"ancestor_record_offset":88088,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1682,"record_offset":89452,"parent_record_offset":89400,"ancestor_record_offset":88088,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":1682,"record_offset":89400,"parent_record_offset":88948,"ancestor_record_offset":88088,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1682,"record_offset":89604,"parent_record_offset":88948,"ancestor_record_offset":88088,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1682,"record_offset":88548,"parent_record_offset":88524,"ancestor_record_offset":88088,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1682,"record_offset":88576,"parent_record_offset":88548,"ancestor_record_offset":88088,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1682,"record_offset":89776,"parent_record_offset":89748,"ancestor_record_offset":88088,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1682,"record_offset":88772,"parent_record_offset":88332,"ancestor_record_offset":88088,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":88796,"parent_record_offset":88772,"ancestor_record_offset":88088,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":89648,"parent_record_offset":88168,"ancestor_record_offset":88088,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1682,"record_offset":89564,"parent_record_offset":89496,"ancestor_record_offset":88088,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1682,"record_offset":89536,"parent_record_offset":89496,"ancestor_record_offset":88088,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1682,"record_offset":89496,"parent_record_offset":89452,"ancestor_record_offset":88088,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1682,"record_offset":89940,"parent_record_offset":88120,"ancestor_record_offset":88088,"inlinee":30582,"source_line":11,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1682,"record_offset":88168,"parent_record_offset":88120,"ancestor_record_offset":88088,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::impl$0::target_states::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DAC
// 首条位置: {"module":1643,"inlinee":81324}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::impl$0::target_states::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D9D
// 首条位置: {"module":1643,"inlinee":81309}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui_move_wall_group::WallGroupMember::max_bottom_y_at
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":1643,"record_offset":424804,"parent_record_offset":424776,"ancestor_record_offset":424776,"inlinee":35304,"source_line":347,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1643,"record_offset":425228,"parent_record_offset":425108,"ancestor_record_offset":424776,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1643,"record_offset":425168,"parent_record_offset":425108,"ancestor_record_offset":424776,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1643,"record_offset":425140,"parent_record_offset":425108,"ancestor_record_offset":424776,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1643,"record_offset":425196,"parent_record_offset":425108,"ancestor_record_offset":424776,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1643,"record_offset":425360,"parent_record_offset":425008,"ancestor_record_offset":424776,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1643,"record_offset":425404,"parent_record_offset":424844,"ancestor_record_offset":424776,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1643,"record_offset":424876,"parent_record_offset":424844,"ancestor_record_offset":424776,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1643,"record_offset":425584,"parent_record_offset":425428,"ancestor_record_offset":424776,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1643,"record_offset":424844,"parent_record_offset":424804,"ancestor_record_offset":424776,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1643,"record_offset":426056,"parent_record_offset":425428,"ancestor_record_offset":424776,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1643,"record_offset":425292,"parent_record_offset":425264,"ancestor_record_offset":424776,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1643,"record_offset":425996,"parent_record_offset":425584,"ancestor_record_offset":424776,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1643,"record_offset":424984,"parent_record_offset":424960,"ancestor_record_offset":424776,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1643,"record_offset":424960,"parent_record_offset":424876,"ancestor_record_offset":424776,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1643,"record_offset":425548,"parent_record_offset":425524,"ancestor_record_offset":424776,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1643,"record_offset":425524,"parent_record_offset":425500,"ancestor_record_offset":424776,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1643,"record_offset":425500,"parent_record_offset":425428,"ancestor_record_offset":424776,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1643,"record_offset":424928,"parent_record_offset":424904,"ancestor_record_offset":424776,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1643,"record_offset":425428,"parent_record_offset":425404,"ancestor_record_offset":424776,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1643,"record_offset":424904,"parent_record_offset":424876,"ancestor_record_offset":424776,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_move_wall_group::impl$1::max_bottom_y_at::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":1643,"record_offset":425008,"parent_record_offset":424984,"ancestor_record_offset":424776,"inlinee":35210,"source_line":359,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::transform_point
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":1643,"record_offset":425036,"parent_record_offset":425008,"ancestor_record_offset":424776,"inlinee":35205,"source_line":137,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine2::Affine2::transform_point2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine2.rs
//     evidence={"module":1643,"record_offset":425060,"parent_record_offset":425036,"ancestor_record_offset":424776,"inlinee":27806,"source_line":270,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1643,"record_offset":425108,"parent_record_offset":425084,"ancestor_record_offset":424776,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1643,"record_offset":425084,"parent_record_offset":425060,"ancestor_record_offset":424776,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1643,"record_offset":425264,"parent_record_offset":425060,"ancestor_record_offset":424776,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::WallGroupTransformCache::tree_areas
// 记录形式: compiled_procedure; instances=1
// 源码行: 215
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":561,"record_offset":42324,"rva":11619648,"va":5380328768}
// 编译实例 RVAs: 0xb14d40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4650157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380328768,"raw_hex":"e8530bb9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380328840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11606640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380328768,"raw_hex":"e890e54eff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380328827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36291760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380328768,"raw_hex":"e8b0c42902","size":5,"soft_fail":false,"target_va":5416620794,"va":5380329029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":42632,"parent_record_offset":42324,"ancestor_record_offset":42324,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":42492,"parent_record_offset":42324,"ancestor_record_offset":42324,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":42928,"parent_record_offset":42904,"ancestor_record_offset":42324,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":42952,"parent_record_offset":42928,"ancestor_record_offset":42324,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":42976,"parent_record_offset":42952,"ancestor_record_offset":42324,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":42904,"parent_record_offset":42872,"ancestor_record_offset":42324,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":42812,"parent_record_offset":42784,"ancestor_record_offset":42324,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":42872,"parent_record_offset":42840,"ancestor_record_offset":42324,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":42840,"parent_record_offset":42812,"ancestor_record_offset":42324,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":43060,"parent_record_offset":43036,"ancestor_record_offset":42324,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":42756,"parent_record_offset":42716,"ancestor_record_offset":42324,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":42784,"parent_record_offset":42756,"ancestor_record_offset":42324,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":42660,"parent_record_offset":42632,"ancestor_record_offset":42324,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":43308,"parent_record_offset":43284,"ancestor_record_offset":42324,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":561,"record_offset":43036,"parent_record_offset":42716,"ancestor_record_offset":42324,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":561,"record_offset":42688,"parent_record_offset":42660,"ancestor_record_offset":42324,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":561,"record_offset":42716,"parent_record_offset":42688,"ancestor_record_offset":42324,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":42588,"parent_record_offset":42564,"ancestor_record_offset":42324,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":43420,"parent_record_offset":43396,"ancestor_record_offset":42324,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":42564,"parent_record_offset":42540,"ancestor_record_offset":42324,"inlinee":6666,"source_line":1574,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":43108,"parent_record_offset":43084,"ancestor_record_offset":42324,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":43188,"parent_record_offset":43156,"ancestor_record_offset":42324,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":42516,"parent_record_offset":42492,"ancestor_record_offset":42324,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":43132,"parent_record_offset":43108,"ancestor_record_offset":42324,"inlinee":9710,"source_line":99,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$4::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":43156,"parent_record_offset":43132,"ancestor_record_offset":42324,"inlinee":9711,"source_line":275,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":42540,"parent_record_offset":42516,"ancestor_record_offset":42324,"inlinee":7230,"source_line":299,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":561,"record_offset":43396,"parent_record_offset":43372,"ancestor_record_offset":42324,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":561,"record_offset":43372,"parent_record_offset":43156,"ancestor_record_offset":42324,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":43084,"parent_record_offset":43060,"ancestor_record_offset":42324,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":43284,"parent_record_offset":43188,"ancestor_record_offset":42324,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":43332,"parent_record_offset":43308,"ancestor_record_offset":42324,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_move_wall_group::WallGroupMember::tree_area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":43236,"parent_record_offset":43212,"ancestor_record_offset":42324,"inlinee":35229,"source_line":417,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_move_wall_group::impl$0::tree_areas::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":43212,"parent_record_offset":43188,"ancestor_record_offset":42324,"inlinee":35246,"source_line":219,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::impl$0::tree_areas::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 219
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89AE
// 首条位置: {"module":561,"inlinee":35246}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui_move_wall_group::WallGroupMember::tree_area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":43236,"parent_record_offset":43212,"ancestor_record_offset":43212,"inlinee":35229,"source_line":417,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::group_wire_rms_displacement
// 记录形式: compiled_procedure; instances=1
// 源码行: 228
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":561,"record_offset":55900,"rva":11623792,"va":5380332912}
// 编译实例 RVAs: 0xb15d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":56848,"parent_record_offset":55900,"ancestor_record_offset":55900,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":56080,"parent_record_offset":55900,"ancestor_record_offset":55900,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::transform_point
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":56144,"parent_record_offset":55900,"ancestor_record_offset":55900,"inlinee":35205,"source_line":137,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":57116,"parent_record_offset":55900,"ancestor_record_offset":55900,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":57872,"parent_record_offset":55900,"ancestor_record_offset":55900,"inlinee":26151,"source_line":471,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":57764,"parent_record_offset":55900,"ancestor_record_offset":55900,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::segments
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":561,"record_offset":56820,"parent_record_offset":55900,"ancestor_record_offset":55900,"inlinee":35244,"source_line":137,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":561,"record_offset":56432,"parent_record_offset":56344,"ancestor_record_offset":55900,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":561,"record_offset":56372,"parent_record_offset":56344,"ancestor_record_offset":55900,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":561,"record_offset":56400,"parent_record_offset":56344,"ancestor_record_offset":55900,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":57308,"parent_record_offset":57280,"ancestor_record_offset":55900,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":561,"record_offset":57024,"parent_record_offset":56884,"ancestor_record_offset":55900,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":561,"record_offset":56924,"parent_record_offset":56884,"ancestor_record_offset":55900,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":56884,"parent_record_offset":56848,"ancestor_record_offset":55900,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":56252,"parent_record_offset":56216,"ancestor_record_offset":55900,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":57200,"parent_record_offset":57152,"ancestor_record_offset":55900,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":56108,"parent_record_offset":56080,"ancestor_record_offset":55900,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":56964,"parent_record_offset":56924,"ancestor_record_offset":55900,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine2::Affine2::transform_point2
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine2.rs
//     evidence={"module":561,"record_offset":56180,"parent_record_offset":56144,"ancestor_record_offset":55900,"inlinee":27806,"source_line":270,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":56344,"parent_record_offset":56320,"ancestor_record_offset":55900,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":56320,"parent_record_offset":56180,"ancestor_record_offset":55900,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":57232,"parent_record_offset":57152,"ancestor_record_offset":55900,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":56216,"parent_record_offset":56180,"ancestor_record_offset":55900,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":57152,"parent_record_offset":57116,"ancestor_record_offset":55900,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":561,"record_offset":57280,"parent_record_offset":57232,"ancestor_record_offset":55900,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::WallGroupMember::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":561,"record_offset":22728,"rva":11607392,"va":5380316512}
// 编译实例 RVAs: 0xb11d60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4638396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380316512,"raw_hex":"e84439b9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380317079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4639049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380316512,"raw_hex":"e8b736b9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380317748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11594879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380316512,"raw_hex":"e881134fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380317066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=2
//     target_aliases=11 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11077193","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380316512,"raw_hex":"e8b7f956ff","size":5,"soft_fail":false,"target_va":5369240528,"va":5380317716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6837652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380316512,"raw_hex":"e86caa97ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5380316687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b17f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t24914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380316512,"raw_hex":"e852610000","size":5,"soft_fail":false,"target_va":5380341536,"va":5380316617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141618340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11560007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380316512,"raw_hex":"e84764b000","size":5,"soft_fail":false,"target_va":5391876928,"va":5380316916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d7a5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36078629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380316512,"raw_hex":"e825842602","size":5,"soft_fail":false,"target_va":5416396272,"va":5380317638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36303135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380316512,"raw_hex":"e81ff12902","size":5,"soft_fail":false,"target_va":5416620794,"va":5380317654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36303901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380316512,"raw_hex":"e81df42902","size":5,"soft_fail":false,"target_va":5416621536,"va":5380317630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":561,"record_offset":24064,"parent_record_offset":22728,"ancestor_record_offset":22728,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":24196,"parent_record_offset":22728,"ancestor_record_offset":22728,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":22924,"parent_record_offset":22728,"ancestor_record_offset":22728,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":25084,"parent_record_offset":22728,"ancestor_record_offset":22728,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::wall_decorator_relative_positions
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":22960,"parent_record_offset":22728,"ancestor_record_offset":22728,"inlinee":35212,"source_line":189,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::ShapeParameters::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":561,"record_offset":22896,"parent_record_offset":22728,"ancestor_record_offset":22728,"inlinee":29289,"source_line":82,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":24516,"parent_record_offset":24492,"ancestor_record_offset":22728,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":24540,"parent_record_offset":24516,"ancestor_record_offset":22728,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":25272,"parent_record_offset":25248,"ancestor_record_offset":22728,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":25296,"parent_record_offset":25272,"ancestor_record_offset":22728,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":24564,"parent_record_offset":24540,"ancestor_record_offset":22728,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":25320,"parent_record_offset":25296,"ancestor_record_offset":22728,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":24492,"parent_record_offset":24460,"ancestor_record_offset":22728,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":25248,"parent_record_offset":25156,"ancestor_record_offset":22728,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":24400,"parent_record_offset":24372,"ancestor_record_offset":22728,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":25184,"parent_record_offset":25156,"ancestor_record_offset":22728,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":25156,"parent_record_offset":25132,"ancestor_record_offset":22728,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":24460,"parent_record_offset":24428,"ancestor_record_offset":22728,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":24428,"parent_record_offset":24400,"ancestor_record_offset":22728,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":25132,"parent_record_offset":25108,"ancestor_record_offset":22728,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":24656,"parent_record_offset":24624,"ancestor_record_offset":22728,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":24344,"parent_record_offset":24304,"ancestor_record_offset":22728,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":24372,"parent_record_offset":24344,"ancestor_record_offset":22728,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":24000,"parent_record_offset":23976,"ancestor_record_offset":22728,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":24924,"parent_record_offset":24888,"ancestor_record_offset":22728,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":561,"record_offset":25008,"parent_record_offset":24924,"ancestor_record_offset":22728,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":561,"record_offset":24624,"parent_record_offset":24304,"ancestor_record_offset":22728,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":561,"record_offset":24268,"parent_record_offset":24232,"ancestor_record_offset":22728,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":561,"record_offset":24304,"parent_record_offset":24268,"ancestor_record_offset":22728,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":23728,"parent_record_offset":23704,"ancestor_record_offset":22728,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":561,"record_offset":23296,"parent_record_offset":23272,"ancestor_record_offset":22728,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":561,"record_offset":23380,"parent_record_offset":23356,"ancestor_record_offset":22728,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":561,"record_offset":23796,"parent_record_offset":23156,"ancestor_record_offset":22728,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":24720,"parent_record_offset":24688,"ancestor_record_offset":22728,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":24852,"parent_record_offset":24752,"ancestor_record_offset":22728,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":561,"record_offset":24024,"parent_record_offset":24000,"ancestor_record_offset":22728,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":23976,"parent_record_offset":22960,"ancestor_record_offset":22728,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":24688,"parent_record_offset":24656,"ancestor_record_offset":22728,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":24888,"parent_record_offset":24852,"ancestor_record_offset":22728,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":25216,"parent_record_offset":25184,"ancestor_record_offset":22728,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":561,"record_offset":23528,"parent_record_offset":23500,"ancestor_record_offset":22728,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":561,"record_offset":23500,"parent_record_offset":23472,"ancestor_record_offset":22728,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":24148,"parent_record_offset":24120,"ancestor_record_offset":22728,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":23320,"parent_record_offset":23296,"ancestor_record_offset":22728,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":25108,"parent_record_offset":25084,"ancestor_record_offset":22728,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":23644,"parent_record_offset":23620,"ancestor_record_offset":22728,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":24812,"parent_record_offset":24752,"ancestor_record_offset":22728,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":24968,"parent_record_offset":24924,"ancestor_record_offset":22728,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":24752,"parent_record_offset":24720,"ancestor_record_offset":22728,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":561,"record_offset":22988,"parent_record_offset":22960,"ancestor_record_offset":22728,"inlinee":32740,"source_line":701,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":561,"record_offset":23752,"parent_record_offset":23728,"ancestor_record_offset":22728,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":561,"record_offset":24120,"parent_record_offset":24092,"ancestor_record_offset":22728,"inlinee":29140,"source_line":190,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_move_wall_group::impl$1::new::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":24092,"parent_record_offset":24064,"ancestor_record_offset":22728,"inlinee":35214,"source_line":302,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":561,"record_offset":23440,"parent_record_offset":23412,"ancestor_record_offset":22728,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":561,"record_offset":23472,"parent_record_offset":23440,"ancestor_record_offset":22728,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":561,"record_offset":23412,"parent_record_offset":23156,"ancestor_record_offset":22728,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":561,"record_offset":23272,"parent_record_offset":23156,"ancestor_record_offset":22728,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":561,"record_offset":23828,"parent_record_offset":23156,"ancestor_record_offset":22728,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":561,"record_offset":23356,"parent_record_offset":23156,"ancestor_record_offset":22728,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":561,"record_offset":23216,"parent_record_offset":23156,"ancestor_record_offset":22728,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":561,"record_offset":23704,"parent_record_offset":23680,"ancestor_record_offset":22728,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":561,"record_offset":23036,"parent_record_offset":23012,"ancestor_record_offset":22728,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":561,"record_offset":23680,"parent_record_offset":23572,"ancestor_record_offset":22728,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":561,"record_offset":23076,"parent_record_offset":23036,"ancestor_record_offset":22728,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":23620,"parent_record_offset":23596,"ancestor_record_offset":22728,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":23920,"parent_record_offset":23156,"ancestor_record_offset":22728,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":23596,"parent_record_offset":23572,"ancestor_record_offset":22728,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":23132,"parent_record_offset":23104,"ancestor_record_offset":22728,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":23104,"parent_record_offset":23036,"ancestor_record_offset":22728,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":23156,"parent_record_offset":23132,"ancestor_record_offset":22728,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":23244,"parent_record_offset":23156,"ancestor_record_offset":22728,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":23572,"parent_record_offset":23156,"ancestor_record_offset":22728,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":561,"record_offset":23012,"parent_record_offset":22988,"ancestor_record_offset":22728,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::impl$1::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 302
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x898E
// 首条位置: {"module":561,"inlinee":35214}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":561,"record_offset":24120,"parent_record_offset":24092,"ancestor_record_offset":24092,"inlinee":29140,"source_line":190,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":24148,"parent_record_offset":24120,"ancestor_record_offset":24092,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::WallGroupMember::target_state
// 记录形式: compiled_procedure; instances=1
// 源码行: 319
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":561,"record_offset":19872,"rva":11606304,"va":5380315424}
// 编译实例 RVAs: 0xb11920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4636878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380315424,"raw_hex":"e8323fb9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380315561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4637709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380315424,"raw_hex":"e8f33bb9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380316408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11593355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380315424,"raw_hex":"e875194fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380315542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140081bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11075962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380315424,"raw_hex":"e886fe56ff","size":5,"soft_fail":false,"target_va":5369240528,"va":5380316485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36741455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380315424,"raw_hex":"e84fa13002","size":5,"soft_fail":false,"target_va":5417057056,"va":5380315596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36741432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380315424,"raw_hex":"e838a13002","size":5,"soft_fail":false,"target_va":5417057600,"va":5380316163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36305272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380315424,"raw_hex":"e878f92902","size":5,"soft_fail":false,"target_va":5416620794,"va":5380315517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::circular_f32::CircularF32::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17756397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380315424,"raw_hex":"e8edf00e01","size":5,"soft_fail":false,"target_va":5398072576,"va":5380316174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":22468,"parent_record_offset":19872,"ancestor_record_offset":19872,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::ClutterVersion::latest
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":561,"record_offset":22440,"parent_record_offset":19872,"ancestor_record_offset":19872,"inlinee":26924,"source_line":205,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$45::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":561,"record_offset":20052,"parent_record_offset":19872,"ancestor_record_offset":19872,"inlinee":30580,"source_line":115,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::rotation_angle
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":22384,"parent_record_offset":19872,"ancestor_record_offset":19872,"inlinee":35208,"source_line":101,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::transform_point
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":21916,"parent_record_offset":19872,"ancestor_record_offset":19872,"inlinee":35205,"source_line":137,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::transform_vector
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":22244,"parent_record_offset":19872,"ancestor_record_offset":19872,"inlinee":35206,"source_line":141,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::ShapeParameters::center
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":561,"record_offset":21888,"parent_record_offset":19872,"ancestor_record_offset":19872,"inlinee":29289,"source_line":82,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":20608,"parent_record_offset":20584,"ancestor_record_offset":19872,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":20632,"parent_record_offset":20608,"ancestor_record_offset":19872,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":21748,"parent_record_offset":21724,"ancestor_record_offset":19872,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":21772,"parent_record_offset":21748,"ancestor_record_offset":19872,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":20656,"parent_record_offset":20632,"ancestor_record_offset":19872,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":21796,"parent_record_offset":21772,"ancestor_record_offset":19872,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":20584,"parent_record_offset":20404,"ancestor_record_offset":19872,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":21724,"parent_record_offset":21636,"ancestor_record_offset":19872,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":20348,"parent_record_offset":20324,"ancestor_record_offset":19872,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":21664,"parent_record_offset":21636,"ancestor_record_offset":19872,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":21636,"parent_record_offset":21612,"ancestor_record_offset":19872,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":20404,"parent_record_offset":20372,"ancestor_record_offset":19872,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":20372,"parent_record_offset":20348,"ancestor_record_offset":19872,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":21612,"parent_record_offset":21588,"ancestor_record_offset":19872,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":20440,"parent_record_offset":20404,"ancestor_record_offset":19872,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":561,"record_offset":20872,"parent_record_offset":20808,"ancestor_record_offset":19872,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":561,"record_offset":20280,"parent_record_offset":20208,"ancestor_record_offset":19872,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":20252,"parent_record_offset":20208,"ancestor_record_offset":19872,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":20324,"parent_record_offset":20280,"ancestor_record_offset":19872,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":20172,"parent_record_offset":20116,"ancestor_record_offset":19872,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":20208,"parent_record_offset":20172,"ancestor_record_offset":19872,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":561,"record_offset":20464,"parent_record_offset":20440,"ancestor_record_offset":19872,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":561,"record_offset":22112,"parent_record_offset":21988,"ancestor_record_offset":19872,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":561,"record_offset":22052,"parent_record_offset":21988,"ancestor_record_offset":19872,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":561,"record_offset":22024,"parent_record_offset":21988,"ancestor_record_offset":19872,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":561,"record_offset":22080,"parent_record_offset":21988,"ancestor_record_offset":19872,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":561,"record_offset":20544,"parent_record_offset":20488,"ancestor_record_offset":19872,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":561,"record_offset":21368,"parent_record_offset":21316,"ancestor_record_offset":19872,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":561,"record_offset":21316,"parent_record_offset":20872,"ancestor_record_offset":19872,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":561,"record_offset":21520,"parent_record_offset":20872,"ancestor_record_offset":19872,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":20488,"parent_record_offset":20464,"ancestor_record_offset":19872,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":20516,"parent_record_offset":20488,"ancestor_record_offset":19872,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":21692,"parent_record_offset":21664,"ancestor_record_offset":19872,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":22176,"parent_record_offset":22148,"ancestor_record_offset":19872,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":561,"record_offset":20712,"parent_record_offset":20280,"ancestor_record_offset":19872,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":20736,"parent_record_offset":20712,"ancestor_record_offset":19872,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":21564,"parent_record_offset":20116,"ancestor_record_offset":19872,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":21480,"parent_record_offset":21412,"ancestor_record_offset":19872,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":21452,"parent_record_offset":21412,"ancestor_record_offset":19872,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":21412,"parent_record_offset":21368,"ancestor_record_offset":19872,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":561,"record_offset":21856,"parent_record_offset":20052,"ancestor_record_offset":19872,"inlinee":30582,"source_line":11,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine2::Affine2::transform_point2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine2.rs
//     evidence={"module":561,"record_offset":21940,"parent_record_offset":21916,"ancestor_record_offset":19872,"inlinee":27806,"source_line":270,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine2::Affine2::transform_vector2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine2.rs
//     evidence={"module":561,"record_offset":22268,"parent_record_offset":22244,"ancestor_record_offset":19872,"inlinee":35207,"source_line":279,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":21988,"parent_record_offset":21964,"ancestor_record_offset":19872,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":21964,"parent_record_offset":21940,"ancestor_record_offset":19872,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":22148,"parent_record_offset":21940,"ancestor_record_offset":19872,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":561,"record_offset":22408,"parent_record_offset":22384,"ancestor_record_offset":19872,"inlinee":11245,"source_line":880,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":561,"record_offset":20116,"parent_record_offset":20052,"ancestor_record_offset":19872,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::WallGroupMember::max_bottom_y_at
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 347
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89E8
// 首条位置: {"module":1643,"inlinee":35304}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1643,"record_offset":424844,"parent_record_offset":424804,"ancestor_record_offset":424804,"inlinee":9020,"source_line":2678,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1643,"record_offset":425228,"parent_record_offset":425108,"ancestor_record_offset":424804,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1643,"record_offset":425168,"parent_record_offset":425108,"ancestor_record_offset":424804,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1643,"record_offset":425140,"parent_record_offset":425108,"ancestor_record_offset":424804,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1643,"record_offset":425196,"parent_record_offset":425108,"ancestor_record_offset":424804,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1643,"record_offset":425360,"parent_record_offset":425008,"ancestor_record_offset":424804,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1643,"record_offset":425404,"parent_record_offset":424844,"ancestor_record_offset":424804,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1643,"record_offset":424876,"parent_record_offset":424844,"ancestor_record_offset":424804,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1643,"record_offset":425584,"parent_record_offset":425428,"ancestor_record_offset":424804,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1643,"record_offset":426056,"parent_record_offset":425428,"ancestor_record_offset":424804,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1643,"record_offset":425292,"parent_record_offset":425264,"ancestor_record_offset":424804,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1643,"record_offset":425996,"parent_record_offset":425584,"ancestor_record_offset":424804,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1643,"record_offset":424984,"parent_record_offset":424960,"ancestor_record_offset":424804,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1643,"record_offset":424960,"parent_record_offset":424876,"ancestor_record_offset":424804,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1643,"record_offset":425548,"parent_record_offset":425524,"ancestor_record_offset":424804,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1643,"record_offset":425524,"parent_record_offset":425500,"ancestor_record_offset":424804,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1643,"record_offset":425500,"parent_record_offset":425428,"ancestor_record_offset":424804,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1643,"record_offset":424928,"parent_record_offset":424904,"ancestor_record_offset":424804,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1643,"record_offset":425428,"parent_record_offset":425404,"ancestor_record_offset":424804,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1643,"record_offset":424904,"parent_record_offset":424876,"ancestor_record_offset":424804,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_move_wall_group::impl$1::max_bottom_y_at::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":1643,"record_offset":425008,"parent_record_offset":424984,"ancestor_record_offset":424804,"inlinee":35210,"source_line":359,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::transform_point
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":1643,"record_offset":425036,"parent_record_offset":425008,"ancestor_record_offset":424804,"inlinee":35205,"source_line":137,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine2::Affine2::transform_point2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine2.rs
//     evidence={"module":1643,"record_offset":425060,"parent_record_offset":425036,"ancestor_record_offset":424804,"inlinee":27806,"source_line":270,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1643,"record_offset":425108,"parent_record_offset":425084,"ancestor_record_offset":424804,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1643,"record_offset":425084,"parent_record_offset":425060,"ancestor_record_offset":424804,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1643,"record_offset":425264,"parent_record_offset":425060,"ancestor_record_offset":424804,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::impl$1::max_bottom_y_at::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 359
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x898A
// 首条位置: {"module":561,"inlinee":35210}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1643,"record_offset":425360,"parent_record_offset":425008,"ancestor_record_offset":425008,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::transform_point
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":1643,"record_offset":425036,"parent_record_offset":425008,"ancestor_record_offset":425008,"inlinee":35205,"source_line":137,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1643,"record_offset":425228,"parent_record_offset":425108,"ancestor_record_offset":425008,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1643,"record_offset":425168,"parent_record_offset":425108,"ancestor_record_offset":425008,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1643,"record_offset":425140,"parent_record_offset":425108,"ancestor_record_offset":425008,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1643,"record_offset":425196,"parent_record_offset":425108,"ancestor_record_offset":425008,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1643,"record_offset":425292,"parent_record_offset":425264,"ancestor_record_offset":425008,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine2::Affine2::transform_point2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine2.rs
//     evidence={"module":1643,"record_offset":425060,"parent_record_offset":425036,"ancestor_record_offset":425008,"inlinee":27806,"source_line":270,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1643,"record_offset":425108,"parent_record_offset":425084,"ancestor_record_offset":425008,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":1643,"record_offset":425084,"parent_record_offset":425060,"ancestor_record_offset":425008,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1643,"record_offset":425264,"parent_record_offset":425060,"ancestor_record_offset":425008,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::WallGroupMember::fits_inside_glade
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 367
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89DE
// 首条位置: {"module":561,"inlinee":35294}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::glade_border::GladeBorder::is_curve2_inside
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/glade_border.rs
//     evidence={"module":561,"record_offset":89764,"parent_record_offset":89732,"ancestor_record_offset":89732,"inlinee":35295,"source_line":54,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":89836,"parent_record_offset":89804,"ancestor_record_offset":89732,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":89804,"parent_record_offset":89764,"ancestor_record_offset":89732,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::glade_border::GladeBorder::is_pos_inside
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/glade_border.rs
//     evidence={"module":561,"record_offset":89876,"parent_record_offset":89764,"ancestor_record_offset":89732,"inlinee":28651,"source_line":40,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":561,"record_offset":89912,"parent_record_offset":89876,"ancestor_record_offset":89732,"inlinee":28653,"source_line":125,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::WallGroupMember::shape_moved
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 374
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89B0
// 首条位置: {"module":561,"inlinee":35248}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":48480,"parent_record_offset":48440,"ancestor_record_offset":48440,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":48796,"parent_record_offset":48772,"ancestor_record_offset":48440,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":48820,"parent_record_offset":48796,"ancestor_record_offset":48440,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":48844,"parent_record_offset":48820,"ancestor_record_offset":48440,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":48772,"parent_record_offset":48736,"ancestor_record_offset":48440,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":48668,"parent_record_offset":48636,"ancestor_record_offset":48440,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":48736,"parent_record_offset":48700,"ancestor_record_offset":48440,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":48700,"parent_record_offset":48668,"ancestor_record_offset":48440,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":49068,"parent_record_offset":49044,"ancestor_record_offset":48440,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":48604,"parent_record_offset":48564,"ancestor_record_offset":48440,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":48636,"parent_record_offset":48604,"ancestor_record_offset":48440,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":48508,"parent_record_offset":48480,"ancestor_record_offset":48440,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":49384,"parent_record_offset":49360,"ancestor_record_offset":48440,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":561,"record_offset":49044,"parent_record_offset":48564,"ancestor_record_offset":48440,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":561,"record_offset":48536,"parent_record_offset":48508,"ancestor_record_offset":48440,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":561,"record_offset":48564,"parent_record_offset":48536,"ancestor_record_offset":48440,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":561,"record_offset":49312,"parent_record_offset":49288,"ancestor_record_offset":48440,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":49116,"parent_record_offset":49092,"ancestor_record_offset":48440,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":48904,"parent_record_offset":48564,"ancestor_record_offset":48440,"inlinee":5082,"source_line":111,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":49168,"parent_record_offset":49140,"ancestor_record_offset":48440,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":49092,"parent_record_offset":49068,"ancestor_record_offset":48440,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":49360,"parent_record_offset":49168,"ancestor_record_offset":48440,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":561,"record_offset":49000,"parent_record_offset":48976,"ancestor_record_offset":48440,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":48976,"parent_record_offset":48952,"ancestor_record_offset":48440,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":48952,"parent_record_offset":48928,"ancestor_record_offset":48440,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":49408,"parent_record_offset":49384,"ancestor_record_offset":48440,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":49140,"parent_record_offset":49116,"ancestor_record_offset":48440,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":48928,"parent_record_offset":48904,"ancestor_record_offset":48440,"inlinee":6665,"source_line":218,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_move_wall_group::impl$1::shape_moved::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":49192,"parent_record_offset":49168,"ancestor_record_offset":48440,"inlinee":35250,"source_line":380,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::transform_vector
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":49216,"parent_record_offset":49192,"ancestor_record_offset":48440,"inlinee":35206,"source_line":141,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine2::Affine2::transform_vector2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine2.rs
//     evidence={"module":561,"record_offset":49240,"parent_record_offset":49216,"ancestor_record_offset":48440,"inlinee":35207,"source_line":279,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":49288,"parent_record_offset":49264,"ancestor_record_offset":48440,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":49264,"parent_record_offset":49240,"ancestor_record_offset":48440,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::impl$1::shape_moved::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 380
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89B2
// 首条位置: {"module":561,"inlinee":35250}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::transform_vector
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":49216,"parent_record_offset":49192,"ancestor_record_offset":49192,"inlinee":35206,"source_line":141,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":561,"record_offset":49312,"parent_record_offset":49288,"ancestor_record_offset":49192,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine2::Affine2::transform_vector2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine2.rs
//     evidence={"module":561,"record_offset":49240,"parent_record_offset":49216,"ancestor_record_offset":49192,"inlinee":35207,"source_line":279,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":49288,"parent_record_offset":49264,"ancestor_record_offset":49192,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":49264,"parent_record_offset":49240,"ancestor_record_offset":49192,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::WallGroupMember::geometry_changed
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 388
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x899E
// 首条位置: {"module":561,"inlinee":35230}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":32372,"parent_record_offset":32020,"ancestor_record_offset":32020,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":32060,"parent_record_offset":32020,"ancestor_record_offset":32020,"inlinee":13036,"source_line":2126,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":561,"record_offset":32428,"parent_record_offset":32396,"ancestor_record_offset":32020,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":32088,"parent_record_offset":32060,"ancestor_record_offset":32020,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":32148,"parent_record_offset":32116,"ancestor_record_offset":32020,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":561,"record_offset":32452,"parent_record_offset":32428,"ancestor_record_offset":32020,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":561,"record_offset":32480,"parent_record_offset":32452,"ancestor_record_offset":32020,"inlinee":6223,"source_line":122,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":561,"record_offset":32116,"parent_record_offset":32088,"ancestor_record_offset":32020,"inlinee":35232,"source_line":62,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$103::eq
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":32208,"parent_record_offset":32172,"ancestor_record_offset":32020,"inlinee":14672,"source_line":20,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":561,"record_offset":32696,"parent_record_offset":32672,"ancestor_record_offset":32020,"inlinee":17826,"source_line":6,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$48::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":561,"record_offset":32396,"parent_record_offset":32372,"ancestor_record_offset":32020,"inlinee":35236,"source_line":61,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":561,"record_offset":32296,"parent_record_offset":32272,"ancestor_record_offset":32020,"inlinee":35238,"source_line":16,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":561,"record_offset":32172,"parent_record_offset":32148,"ancestor_record_offset":32020,"inlinee":35234,"source_line":114,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::WallGroupMember::history_edit
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 398
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89A7
// 首条位置: {"module":561,"inlinee":35239}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":34560,"parent_record_offset":32752,"ancestor_record_offset":32752,"inlinee":35240,"source_line":113,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":561,"record_offset":32820,"parent_record_offset":32752,"ancestor_record_offset":32752,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":33300,"parent_record_offset":33276,"ancestor_record_offset":32752,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":33324,"parent_record_offset":33300,"ancestor_record_offset":32752,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":34452,"parent_record_offset":34428,"ancestor_record_offset":32752,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":34476,"parent_record_offset":34452,"ancestor_record_offset":32752,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":33348,"parent_record_offset":33324,"ancestor_record_offset":32752,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":34500,"parent_record_offset":34476,"ancestor_record_offset":32752,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":33276,"parent_record_offset":33096,"ancestor_record_offset":32752,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":34428,"parent_record_offset":34340,"ancestor_record_offset":32752,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":33036,"parent_record_offset":33008,"ancestor_record_offset":32752,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":34368,"parent_record_offset":34340,"ancestor_record_offset":32752,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":34340,"parent_record_offset":34316,"ancestor_record_offset":32752,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":33096,"parent_record_offset":33064,"ancestor_record_offset":32752,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":33064,"parent_record_offset":33036,"ancestor_record_offset":32752,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":34316,"parent_record_offset":34292,"ancestor_record_offset":32752,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":33132,"parent_record_offset":33096,"ancestor_record_offset":32752,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":561,"record_offset":33572,"parent_record_offset":33504,"ancestor_record_offset":32752,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":561,"record_offset":32972,"parent_record_offset":32904,"ancestor_record_offset":32752,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":32944,"parent_record_offset":32904,"ancestor_record_offset":32752,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":33008,"parent_record_offset":32972,"ancestor_record_offset":32752,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":32872,"parent_record_offset":32820,"ancestor_record_offset":32752,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":32904,"parent_record_offset":32872,"ancestor_record_offset":32752,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":561,"record_offset":33156,"parent_record_offset":33132,"ancestor_record_offset":32752,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":561,"record_offset":33236,"parent_record_offset":33180,"ancestor_record_offset":32752,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":561,"record_offset":34072,"parent_record_offset":34020,"ancestor_record_offset":32752,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":561,"record_offset":34020,"parent_record_offset":33572,"ancestor_record_offset":32752,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":561,"record_offset":34224,"parent_record_offset":33572,"ancestor_record_offset":32752,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":33180,"parent_record_offset":33156,"ancestor_record_offset":32752,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":33208,"parent_record_offset":33180,"ancestor_record_offset":32752,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":34396,"parent_record_offset":34368,"ancestor_record_offset":32752,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":561,"record_offset":33404,"parent_record_offset":32972,"ancestor_record_offset":32752,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":33428,"parent_record_offset":33404,"ancestor_record_offset":32752,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":34268,"parent_record_offset":32820,"ancestor_record_offset":32752,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":34184,"parent_record_offset":34116,"ancestor_record_offset":32752,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":34156,"parent_record_offset":34116,"ancestor_record_offset":32752,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":34116,"parent_record_offset":34072,"ancestor_record_offset":32752,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::WallGroupMember::tree_area
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 417
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x899D
// 首条位置: {"module":561,"inlinee":35229}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::WallGroupGizmoPlacement::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 436
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":561,"record_offset":37552,"rva":11618144,"va":5380327264}
// 编译实例 RVAs: 0xb14760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4648824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380327264,"raw_hex":"e88810b9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380327507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-4648699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380327264,"raw_hex":"e80511b9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380327398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11605307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380327264,"raw_hex":"e8c5ea4eff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380327494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c71040
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1426758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380327264,"raw_hex":"e846c51500","size":5,"soft_fail":false,"target_va":5381754944,"va":5380328181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2996737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380327264,"raw_hex":"e801ba2d00","size":5,"soft_fail":false,"target_va":5383325056,"va":5380328314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141615bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11539499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380327264,"raw_hex":"e82b14b000","size":5,"soft_fail":false,"target_va":5391866864,"va":5380327360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14180d430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13601175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380327264,"raw_hex":"e89789cf00","size":5,"soft_fail":false,"target_va":5393929264,"va":5380328084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36292312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380327264,"raw_hex":"e8d8c62902","size":5,"soft_fail":false,"target_va":5416620794,"va":5380328477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":41144,"parent_record_offset":37552,"ancestor_record_offset":37552,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":37820,"parent_record_offset":37552,"ancestor_record_offset":37552,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":40848,"parent_record_offset":37552,"ancestor_record_offset":37552,"inlinee":8403,"source_line":3653,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":38304,"parent_record_offset":37552,"ancestor_record_offset":37552,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":38332,"parent_record_offset":37552,"ancestor_record_offset":37552,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":41172,"parent_record_offset":37552,"ancestor_record_offset":37552,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":561,"record_offset":37736,"parent_record_offset":37552,"ancestor_record_offset":37552,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::wall_wire_metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":38360,"parent_record_offset":37552,"ancestor_record_offset":37552,"inlinee":35243,"source_line":155,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":38760,"parent_record_offset":38736,"ancestor_record_offset":37552,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":38784,"parent_record_offset":38760,"ancestor_record_offset":37552,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":38188,"parent_record_offset":38164,"ancestor_record_offset":37552,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":38212,"parent_record_offset":38188,"ancestor_record_offset":37552,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":38808,"parent_record_offset":38784,"ancestor_record_offset":37552,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":38236,"parent_record_offset":38212,"ancestor_record_offset":37552,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":38736,"parent_record_offset":38704,"ancestor_record_offset":37552,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":38164,"parent_record_offset":38076,"ancestor_record_offset":37552,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":38644,"parent_record_offset":38616,"ancestor_record_offset":37552,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":38104,"parent_record_offset":38076,"ancestor_record_offset":37552,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":38076,"parent_record_offset":38052,"ancestor_record_offset":37552,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":38704,"parent_record_offset":38672,"ancestor_record_offset":37552,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":38672,"parent_record_offset":38644,"ancestor_record_offset":37552,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":38052,"parent_record_offset":38032,"ancestor_record_offset":37552,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":38892,"parent_record_offset":38868,"ancestor_record_offset":37552,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":38588,"parent_record_offset":38552,"ancestor_record_offset":37552,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":38616,"parent_record_offset":38588,"ancestor_record_offset":37552,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":37948,"parent_record_offset":37924,"ancestor_record_offset":37552,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":39316,"parent_record_offset":38360,"ancestor_record_offset":37552,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":39132,"parent_record_offset":39088,"ancestor_record_offset":37552,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":561,"record_offset":39236,"parent_record_offset":39132,"ancestor_record_offset":37552,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":561,"record_offset":38868,"parent_record_offset":38552,"ancestor_record_offset":37552,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":561,"record_offset":38520,"parent_record_offset":38488,"ancestor_record_offset":37552,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":561,"record_offset":38552,"parent_record_offset":38520,"ancestor_record_offset":37552,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":41036,"parent_record_offset":41008,"ancestor_record_offset":37552,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":39948,"parent_record_offset":39920,"ancestor_record_offset":37552,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":40332,"parent_record_offset":40308,"ancestor_record_offset":37552,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":38940,"parent_record_offset":38916,"ancestor_record_offset":37552,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":561,"record_offset":39044,"parent_record_offset":38964,"ancestor_record_offset":37552,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\skip.rs
//     evidence={"module":561,"record_offset":39680,"parent_record_offset":39544,"ancestor_record_offset":37552,"inlinee":9772,"source_line":40,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":561,"record_offset":39584,"parent_record_offset":39544,"ancestor_record_offset":37552,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::try_process
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\mod.rs
//     evidence={"module":561,"record_offset":37868,"parent_record_offset":37844,"ancestor_record_offset":37552,"inlinee":11759,"source_line":152,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":39512,"parent_record_offset":38360,"ancestor_record_offset":37552,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":561,"record_offset":39544,"parent_record_offset":39512,"ancestor_record_offset":37552,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":561,"record_offset":40872,"parent_record_offset":40848,"ancestor_record_offset":37552,"inlinee":19979,"source_line":160,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$128::sum::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\accum.rs
//     evidence={"module":561,"record_offset":41068,"parent_record_offset":40980,"ancestor_record_offset":37552,"inlinee":19983,"source_line":164,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":561,"record_offset":37972,"parent_record_offset":37948,"ancestor_record_offset":37552,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":37924,"parent_record_offset":37904,"ancestor_record_offset":37552,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":38916,"parent_record_offset":38892,"ancestor_record_offset":37552,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":39088,"parent_record_offset":39044,"ancestor_record_offset":37552,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":40308,"parent_record_offset":38360,"ancestor_record_offset":37552,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":41100,"parent_record_offset":40920,"ancestor_record_offset":37552,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":38132,"parent_record_offset":38104,"ancestor_record_offset":37552,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":40024,"parent_record_offset":39996,"ancestor_record_offset":37552,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":40088,"parent_record_offset":40060,"ancestor_record_offset":37552,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":40276,"parent_record_offset":40252,"ancestor_record_offset":37552,"inlinee":8897,"source_line":517,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":40216,"parent_record_offset":40188,"ancestor_record_offset":37552,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":39840,"parent_record_offset":39800,"ancestor_record_offset":37552,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$46::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":37844,"parent_record_offset":37820,"ancestor_record_offset":37552,"inlinee":11757,"source_line":2745,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$46::from_iter::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":37904,"parent_record_offset":37868,"ancestor_record_offset":37552,"inlinee":11761,"source_line":2749,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":38012,"parent_record_offset":37868,"ancestor_record_offset":37552,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":37784,"parent_record_offset":37760,"ancestor_record_offset":37552,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":39012,"parent_record_offset":38964,"ancestor_record_offset":37552,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":39452,"parent_record_offset":39428,"ancestor_record_offset":37552,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":39188,"parent_record_offset":39132,"ancestor_record_offset":37552,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":561,"record_offset":39340,"parent_record_offset":39316,"ancestor_record_offset":37552,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":561,"record_offset":37760,"parent_record_offset":37736,"ancestor_record_offset":37552,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":38964,"parent_record_offset":38940,"ancestor_record_offset":37552,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":39428,"parent_record_offset":38360,"ancestor_record_offset":37552,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_move_wall_group::impl$2::new::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":41008,"parent_record_offset":40980,"ancestor_record_offset":37552,"inlinee":32751,"source_line":452,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":39772,"parent_record_offset":38360,"ancestor_record_offset":37552,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":39872,"parent_record_offset":39800,"ancestor_record_offset":37552,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":40060,"parent_record_offset":38360,"ancestor_record_offset":37552,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":39996,"parent_record_offset":38360,"ancestor_record_offset":37552,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":40188,"parent_record_offset":38360,"ancestor_record_offset":37552,"inlinee":11679,"source_line":1428,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":39800,"parent_record_offset":39772,"ancestor_record_offset":37552,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":40252,"parent_record_offset":38360,"ancestor_record_offset":37552,"inlinee":16775,"source_line":1174,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":561,"record_offset":39920,"parent_record_offset":39872,"ancestor_record_offset":37552,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::segments
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":561,"record_offset":39484,"parent_record_offset":38360,"ancestor_record_offset":37552,"inlinee":35244,"source_line":137,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::impl$2::new::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 443
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x143A1
// 首条位置: {"module":1682,"inlinee":82849}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$47::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1682,"record_offset":137448,"parent_record_offset":136396,"ancestor_record_offset":136396,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":1682,"record_offset":136436,"parent_record_offset":136396,"ancestor_record_offset":136396,"inlinee":28986,"source_line":38,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1682,"record_offset":137176,"parent_record_offset":137152,"ancestor_record_offset":136396,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1682,"record_offset":136744,"parent_record_offset":136720,"ancestor_record_offset":136396,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1682,"record_offset":136828,"parent_record_offset":136804,"ancestor_record_offset":136396,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1682,"record_offset":137244,"parent_record_offset":136612,"ancestor_record_offset":136396,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1682,"record_offset":136976,"parent_record_offset":136948,"ancestor_record_offset":136396,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1682,"record_offset":136948,"parent_record_offset":136920,"ancestor_record_offset":136396,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":136768,"parent_record_offset":136744,"ancestor_record_offset":136396,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1682,"record_offset":137092,"parent_record_offset":137068,"ancestor_record_offset":136396,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1682,"record_offset":137200,"parent_record_offset":137176,"ancestor_record_offset":136396,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1682,"record_offset":136888,"parent_record_offset":136860,"ancestor_record_offset":136396,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1682,"record_offset":136920,"parent_record_offset":136888,"ancestor_record_offset":136396,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1682,"record_offset":136860,"parent_record_offset":136612,"ancestor_record_offset":136396,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":136720,"parent_record_offset":136612,"ancestor_record_offset":136396,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":137272,"parent_record_offset":136612,"ancestor_record_offset":136396,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":136804,"parent_record_offset":136612,"ancestor_record_offset":136396,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1682,"record_offset":136664,"parent_record_offset":136612,"ancestor_record_offset":136396,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1682,"record_offset":137152,"parent_record_offset":137128,"ancestor_record_offset":136396,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1682,"record_offset":136500,"parent_record_offset":136472,"ancestor_record_offset":136396,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1682,"record_offset":137128,"parent_record_offset":137020,"ancestor_record_offset":136396,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1682,"record_offset":136532,"parent_record_offset":136500,"ancestor_record_offset":136396,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":137068,"parent_record_offset":137044,"ancestor_record_offset":136396,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":137364,"parent_record_offset":136612,"ancestor_record_offset":136396,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":137044,"parent_record_offset":137020,"ancestor_record_offset":136396,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":136588,"parent_record_offset":136560,"ancestor_record_offset":136396,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":136560,"parent_record_offset":136500,"ancestor_record_offset":136396,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":136612,"parent_record_offset":136588,"ancestor_record_offset":136396,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":136692,"parent_record_offset":136612,"ancestor_record_offset":136396,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":137020,"parent_record_offset":136612,"ancestor_record_offset":136396,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1682,"record_offset":136472,"parent_record_offset":136436,"ancestor_record_offset":136396,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1682,"record_offset":137416,"parent_record_offset":136436,"ancestor_record_offset":136396,"inlinee":28987,"source_line":165,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::impl$2::new::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 452
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7FEF
// 首条位置: {"module":536,"inlinee":32751}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":41036,"parent_record_offset":41008,"ancestor_record_offset":41008,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::ui_move_wall_group
// 记录形式: compiled_procedure; instances=1
// 源码行: 471
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":561,"record_offset":25380,"rva":11608800,"va":5380317920}
// 编译实例 RVAs: 0xb122e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-4640008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8f832b9ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380318691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-4645548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8541db9ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380324247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11596485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e83b0d4fff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380318672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048c880
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6842033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e84f9997ff","size":5,"soft_fail":false,"target_va":5373479040,"va":5380321068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140545280
//     owner=unresolved_namespace; count=2
//     target_aliases=8 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6088723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8ed17a3ff","size":5,"soft_fail":false,"target_va":5374235264,"va":5380323982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d1a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1729852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8c49ae5ff","size":5,"soft_fail":false,"target_va":5378593184,"va":5380323031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d590
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1730016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8209ae5ff","size":5,"soft_fail":false,"target_va":5378594192,"va":5380324203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a5530
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1494949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e85b30e9ff","size":5,"soft_fail":false,"target_va":5378823472,"va":5380318416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a7df0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1490717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8e340e9ff","size":5,"soft_fail":false,"target_va":5378833904,"va":5380324616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7ede0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-610308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8fcaff6ff","size":5,"soft_fail":false,"target_va":5379714528,"va":5380324831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a99c00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-496021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e86b6ef8ff","size":5,"soft_fail":false,"target_va":5379824640,"va":5380320656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b0ba30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8a98dffff","size":5,"soft_fail":false,"target_va":5380291120,"va":5380320386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bc85f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t744773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8455d0b00","size":5,"soft_fail":false,"target_va":5381064176,"va":5380319398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414eee00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10336049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e831b79d00","size":5,"soft_fail":false,"target_va":5390659072,"va":5380323018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f7980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11419491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8633fae00","size":5,"soft_fail":false,"target_va":5391743360,"va":5380323864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141605fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11481824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8e032af00","size":5,"soft_fail":false,"target_va":5391802304,"va":5380320475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416118e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11525579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8cbddaf00","size":5,"soft_fail":false,"target_va":5391849696,"va":5380324112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419af860
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15318693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8a5bee900","size":5,"soft_fail":false,"target_va":5395642464,"va":5380323766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419afa80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15319050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e80ac0e900","size":5,"soft_fail":false,"target_va":5395643008,"va":5380323953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36029159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8e7c22502","size":5,"soft_fail":false,"target_va":5416350496,"va":5380321332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36030519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e837c82502","size":5,"soft_fail":false,"target_va":5416351056,"va":5380320532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36738246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8c6943002","size":5,"soft_fail":false,"target_va":5417057056,"va":5380318805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bda0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36734428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8dc853002","size":5,"soft_fail":false,"target_va":5417057696,"va":5380323263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bef0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: ui_move_wall_group.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36734777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e839873002","size":5,"soft_fail":false,"target_va":5417058032,"va":5380323250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t36302407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e847ee2902","size":5,"soft_fail":false,"target_va":5416620794,"va":5380318382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec::clear<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::alloc::Global>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8e7650000","size":5,"soft_fail":false,"target_va":5380346368,"va":5380320276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec::with_capacity<enum2$<country_core::resources::tree_storage::WallGroupTreeArea> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1932096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8c084e2ff","size":5,"soft_fail":false,"target_va":5378388848,"va":5380320939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36298313","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e849de2902","size":5,"soft_fail":false,"target_va":5416621536,"va":5380323218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::iter::impl$171::all<country_core::resources::walls::inner_wall_state::WallState,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$3>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e84d5a0000","size":5,"soft_fail":false,"target_va":5380346640,"va":5380323518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t227712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e880790300","size":5,"soft_fail":false,"target_va":5380552224,"va":5380324507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t506510","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e88eba0700","size":5,"soft_fail":false,"target_va":5380830960,"va":5380324445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t508895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8dfc30700","size":5,"soft_fail":false,"target_va":5380829808,"va":5380320908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::with_concurrent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t507889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8f1bf0700","size":5,"soft_fail":false,"target_va":5380829968,"va":5380322074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-809676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e834a5f3ff","size":5,"soft_fail":false,"target_va":5379511040,"va":5380320711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui_move_shapes::move_gizmo_and_trigger_vol
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t303000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8989f0400","size":5,"soft_fail":false,"target_va":5380623152,"va":5380320147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui_move_wall_group::WallGroupGizmoPlacement::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8201f0000","size":5,"soft_fail":false,"target_va":5380327264,"va":5380319291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui_move_wall_group::WallGroupTransformCache::apply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8651d0000","size":5,"soft_fail":false,"target_va":5380331216,"va":5380323686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui_move_wall_group::WallGroupTransformCache::cursor_position
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8f2240000","size":5,"soft_fail":false,"target_va":5380330240,"va":5380320777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui_move_wall_group::WallGroupTransformCache::notify_shape_moved
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8f4240000","size":5,"soft_fail":false,"target_va":5380330320,"va":5380320855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui_move_wall_group::WallGroupTransformCache::target_states
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8e6150000","size":5,"soft_fail":false,"target_va":5380329040,"va":5380323429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui_move_wall_group::WallGroupTransformCache::tree_areas
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e82a140000","size":5,"soft_fail":false,"target_va":5380328768,"va":5380323601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui_move_wall_group::group_wire_rms_displacement
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e87d230000","size":5,"soft_fail":false,"target_va":5380332912,"va":5380323822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui_wall_transform_gizmos::wall_rotation_gizmo
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t16415705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8d97bfa00","size":5,"soft_fail":false,"target_va":5396735696,"va":5380319986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody_core::camera::CameraMatrices::eye_direction
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3093152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380317920,"raw_hex":"e8a0322f00","size":5,"soft_fail":false,"target_va":5383412624,"va":5380319467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":561,"record_offset":25692,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":26264,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":29480,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":30260,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::partial_eq::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":561,"record_offset":27020,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":13752,"source_line":17,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":561,"record_offset":29712,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":561,"record_offset":31660,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":561,"record_offset":28768,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":561,"record_offset":28192,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":29228,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":28836,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":34944,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":28864,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":35672,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":561,"record_offset":29144,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":561,"record_offset":26320,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":26084,"source_line":3133,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":29908,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":561,"record_offset":29796,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::new_edit_sequence
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":561,"record_offset":30232,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":29291,"source_line":44,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_terrain
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":561,"record_offset":34648,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":28656,"source_line":329,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":561,"record_offset":30428,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":29283,"source_line":128,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::mode_manager::WallGroupSelection::wall_ids
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/mode_manager.rs
//     evidence={"module":561,"record_offset":25664,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":29274,"source_line":73,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::DragState::is_some
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":561,"record_offset":27420,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":28340,"source_line":388,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":561,"record_offset":27360,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":28114,"source_line":165,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::WidgetId::from_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":561,"record_offset":27304,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":26801,"source_line":21,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_move_wall_group::WallGroupMember::geometry_changed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":32020,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":35230,"source_line":388,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_move_wall_group::WallGroupMember::history_edit
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":32752,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":35239,"source_line":398,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_move_wall_group::WallGroupMember::tree_area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":31396,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":35229,"source_line":417,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::rotated
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":34972,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":35241,"source_line":84,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoInteraction::is_rotating
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":28740,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":35221,"source_line":409,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState::reset
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":26292,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":35216,"source_line":397,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::WallTransformGizmoInteraction::drag_state
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":29072,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":35227,"source_line":431,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::WallTransformGizmoInteraction::resolve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":29016,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":35226,"source_line":423,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_wall_transform_gizmos::live_camera_wall_gizmo_axis
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":27448,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":35217,"source_line":379,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":30084,"parent_record_offset":25380,"ancestor_record_offset":25380,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":26092,"parent_record_offset":26068,"ancestor_record_offset":25380,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":26116,"parent_record_offset":26092,"ancestor_record_offset":25380,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":34452,"parent_record_offset":34428,"ancestor_record_offset":25380,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":34476,"parent_record_offset":34452,"ancestor_record_offset":25380,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":26140,"parent_record_offset":26116,"ancestor_record_offset":25380,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":34500,"parent_record_offset":34476,"ancestor_record_offset":25380,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":26068,"parent_record_offset":25884,"ancestor_record_offset":25380,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":561,"record_offset":34428,"parent_record_offset":34340,"ancestor_record_offset":25380,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":29588,"parent_record_offset":29552,"ancestor_record_offset":25380,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":25820,"parent_record_offset":25792,"ancestor_record_offset":25380,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":34368,"parent_record_offset":34340,"ancestor_record_offset":25380,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":34340,"parent_record_offset":34316,"ancestor_record_offset":25380,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":29636,"parent_record_offset":29612,"ancestor_record_offset":25380,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":29612,"parent_record_offset":29588,"ancestor_record_offset":25380,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":25884,"parent_record_offset":25848,"ancestor_record_offset":25380,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":25848,"parent_record_offset":25820,"ancestor_record_offset":25380,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":34316,"parent_record_offset":34292,"ancestor_record_offset":25380,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":561,"record_offset":25924,"parent_record_offset":25884,"ancestor_record_offset":25380,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":561,"record_offset":28320,"parent_record_offset":28276,"ancestor_record_offset":25380,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":561,"record_offset":25724,"parent_record_offset":25692,"ancestor_record_offset":25380,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":561,"record_offset":33572,"parent_record_offset":33504,"ancestor_record_offset":25380,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":561,"record_offset":25756,"parent_record_offset":25724,"ancestor_record_offset":25380,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":561,"record_offset":28276,"parent_record_offset":28248,"ancestor_record_offset":25380,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":561,"record_offset":28248,"parent_record_offset":28220,"ancestor_record_offset":25380,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":561,"record_offset":28220,"parent_record_offset":28192,"ancestor_record_offset":25380,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":31504,"parent_record_offset":31464,"ancestor_record_offset":25380,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":32944,"parent_record_offset":32904,"ancestor_record_offset":25380,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":31780,"parent_record_offset":31744,"ancestor_record_offset":25380,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":29508,"parent_record_offset":29480,"ancestor_record_offset":25380,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":25792,"parent_record_offset":25756,"ancestor_record_offset":25380,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":32872,"parent_record_offset":32820,"ancestor_record_offset":25380,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":29448,"parent_record_offset":29424,"ancestor_record_offset":25380,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":561,"record_offset":32904,"parent_record_offset":32872,"ancestor_record_offset":25380,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":561,"record_offset":32428,"parent_record_offset":32396,"ancestor_record_offset":25380,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":561,"record_offset":29736,"parent_record_offset":29712,"ancestor_record_offset":25380,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":561,"record_offset":31684,"parent_record_offset":31660,"ancestor_record_offset":25380,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":561,"record_offset":31744,"parent_record_offset":31684,"ancestor_record_offset":25380,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":561,"record_offset":25948,"parent_record_offset":25924,"ancestor_record_offset":25380,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":27056,"parent_record_offset":27020,"ancestor_record_offset":25380,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":26636,"parent_record_offset":26600,"ancestor_record_offset":25380,"inlinee":6450,"source_line":1410,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":31152,"parent_record_offset":31128,"ancestor_record_offset":25380,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":561,"record_offset":32060,"parent_record_offset":32020,"ancestor_record_offset":25380,"inlinee":13036,"source_line":2126,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":561,"record_offset":30720,"parent_record_offset":30696,"ancestor_record_offset":25380,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":561,"record_offset":30804,"parent_record_offset":30780,"ancestor_record_offset":25380,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":561,"record_offset":35232,"parent_record_offset":35176,"ancestor_record_offset":25380,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":561,"record_offset":35204,"parent_record_offset":35176,"ancestor_record_offset":25380,"inlinee":27811,"source_line":974,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":561,"record_offset":27656,"parent_record_offset":27632,"ancestor_record_offset":25380,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":561,"record_offset":26404,"parent_record_offset":26360,"ancestor_record_offset":25380,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":561,"record_offset":26028,"parent_record_offset":25972,"ancestor_record_offset":25380,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":561,"record_offset":34072,"parent_record_offset":34020,"ancestor_record_offset":25380,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":561,"record_offset":34020,"parent_record_offset":33572,"ancestor_record_offset":25380,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":561,"record_offset":29276,"parent_record_offset":29252,"ancestor_record_offset":25380,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":561,"record_offset":34844,"parent_record_offset":34820,"ancestor_record_offset":25380,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":561,"record_offset":34224,"parent_record_offset":33572,"ancestor_record_offset":25380,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":25972,"parent_record_offset":25948,"ancestor_record_offset":25380,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":26000,"parent_record_offset":25972,"ancestor_record_offset":25380,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":561,"record_offset":34396,"parent_record_offset":34368,"ancestor_record_offset":25380,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":561,"record_offset":30952,"parent_record_offset":30924,"ancestor_record_offset":25380,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":561,"record_offset":30924,"parent_record_offset":30896,"ancestor_record_offset":25380,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":27728,"parent_record_offset":27700,"ancestor_record_offset":25380,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":27816,"parent_record_offset":27792,"ancestor_record_offset":25380,"inlinee":8939,"source_line":720,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":561,"record_offset":30112,"parent_record_offset":30084,"ancestor_record_offset":25380,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnMut::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":561,"record_offset":26600,"parent_record_offset":26516,"ancestor_record_offset":25380,"inlinee":4216,"source_line":166,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":34732,"parent_record_offset":34704,"ancestor_record_offset":25380,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":26672,"parent_record_offset":26636,"ancestor_record_offset":25380,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":34704,"parent_record_offset":34676,"ancestor_record_offset":25380,"inlinee":11848,"source_line":1438,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":27848,"parent_record_offset":27448,"ancestor_record_offset":25380,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$46::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":561,"record_offset":29252,"parent_record_offset":29228,"ancestor_record_offset":25380,"inlinee":11757,"source_line":2745,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":561,"record_offset":26196,"parent_record_offset":25756,"ancestor_record_offset":25380,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":26220,"parent_record_offset":26196,"ancestor_record_offset":25380,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":26864,"parent_record_offset":26516,"ancestor_record_offset":25380,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":26976,"parent_record_offset":26460,"ancestor_record_offset":25380,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":561,"record_offset":31068,"parent_record_offset":31044,"ancestor_record_offset":25380,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":29968,"parent_record_offset":29908,"ancestor_record_offset":25380,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":29940,"parent_record_offset":29908,"ancestor_record_offset":25380,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":561,"record_offset":29676,"parent_record_offset":29508,"ancestor_record_offset":25380,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":561,"record_offset":27096,"parent_record_offset":27056,"ancestor_record_offset":25380,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":561,"record_offset":27136,"parent_record_offset":27096,"ancestor_record_offset":25380,"inlinee":6223,"source_line":122,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":561,"record_offset":29820,"parent_record_offset":29796,"ancestor_record_offset":25380,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":561,"record_offset":29168,"parent_record_offset":29144,"ancestor_record_offset":25380,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":34116,"parent_record_offset":34072,"ancestor_record_offset":25380,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":561,"record_offset":26896,"parent_record_offset":26864,"ancestor_record_offset":25380,"inlinee":4734,"source_line":302,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insert_tail
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":561,"record_offset":26516,"parent_record_offset":26460,"ancestor_record_offset":25380,"inlinee":9485,"source_line":542,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::shared::smallsort::insertion_sort_shift_left
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\shared\smallsort.rs
//     evidence={"module":561,"record_offset":26460,"parent_record_offset":26360,"ancestor_record_offset":25380,"inlinee":9500,"source_line":580,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::unstable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\sort\unstable\mod.rs
//     evidence={"module":561,"record_offset":26360,"parent_record_offset":26320,"ancestor_record_offset":25380,"inlinee":11895,"source_line":22,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::DecoratorStorage::iter_terrain_decorators
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":561,"record_offset":34676,"parent_record_offset":34648,"ancestor_record_offset":25380,"inlinee":28658,"source_line":709,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":561,"record_offset":27200,"parent_record_offset":27176,"ancestor_record_offset":25380,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::camera_rig::MainCamera::eye_direction
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/camera_rig.rs
//     evidence={"module":561,"record_offset":27484,"parent_record_offset":27448,"ancestor_record_offset":25380,"inlinee":32187,"source_line":727,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::DragState::is_some
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":561,"record_offset":29040,"parent_record_offset":29016,"ancestor_record_offset":25380,"inlinee":28340,"source_line":388,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":561,"record_offset":28940,"parent_record_offset":28916,"ancestor_record_offset":25380,"inlinee":28114,"source_line":165,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":561,"record_offset":28916,"parent_record_offset":28892,"ancestor_record_offset":25380,"inlinee":28143,"source_line":287,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$14::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":561,"record_offset":27388,"parent_record_offset":27360,"ancestor_record_offset":25380,"inlinee":28109,"source_line":14,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$1::to_widget_name_hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":561,"record_offset":27328,"parent_record_offset":27304,"ancestor_record_offset":25380,"inlinee":26803,"source_line":56,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_move_wall_group::ui_move_wall_group::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":28792,"parent_record_offset":28768,"ancestor_record_offset":25380,"inlinee":35223,"source_line":562,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_move_wall_group::ui_move_wall_group::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":28892,"parent_record_offset":28864,"ancestor_record_offset":25380,"inlinee":35224,"source_line":579,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::RigidXZTransform::affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":34560,"parent_record_offset":32752,"ancestor_record_offset":25380,"inlinee":35240,"source_line":113,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui_wall_transform_gizmos::live_camera_wall_gizmo_axis::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_wall_transform_gizmos.rs
//     evidence={"module":561,"record_offset":27876,"parent_record_offset":27848,"ancestor_record_offset":25380,"inlinee":35219,"source_line":382,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":561,"record_offset":32116,"parent_record_offset":32088,"ancestor_record_offset":25380,"inlinee":35232,"source_line":62,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sin_cos
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/math.rs
//     evidence={"module":561,"record_offset":35028,"parent_record_offset":35000,"ancestor_record_offset":25380,"inlinee":11738,"source_line":182,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::from_angle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":35000,"parent_record_offset":34972,"ancestor_record_offset":25380,"inlinee":35242,"source_line":131,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":35176,"parent_record_offset":35152,"ancestor_record_offset":25380,"inlinee":27810,"source_line":326,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":35116,"parent_record_offset":35000,"ancestor_record_offset":25380,"inlinee":31978,"source_line":56,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$22::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat2.rs
//     evidence={"module":561,"record_offset":35152,"parent_record_offset":34972,"ancestor_record_offset":25380,"inlinee":27808,"source_line":581,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":27600,"parent_record_offset":27568,"ancestor_record_offset":25380,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":27568,"parent_record_offset":27544,"ancestor_record_offset":25380,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":27544,"parent_record_offset":27512,"ancestor_record_offset":25380,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::try_normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":27512,"parent_record_offset":27448,"ancestor_record_offset":25380,"inlinee":27634,"source_line":546,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$103::eq
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":32208,"parent_record_offset":32172,"ancestor_record_offset":25380,"inlinee":14672,"source_line":20,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":27700,"parent_record_offset":27512,"ancestor_record_offset":25380,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":35268,"parent_record_offset":34972,"ancestor_record_offset":25380,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":561,"record_offset":27792,"parent_record_offset":27448,"ancestor_record_offset":25380,"inlinee":13898,"source_line":1853,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":561,"record_offset":30864,"parent_record_offset":30836,"ancestor_record_offset":25380,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":561,"record_offset":30896,"parent_record_offset":30864,"ancestor_record_offset":25380,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":561,"record_offset":30836,"parent_record_offset":30584,"ancestor_record_offset":25380,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":561,"record_offset":30696,"parent_record_offset":30584,"ancestor_record_offset":25380,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":561,"record_offset":31248,"parent_record_offset":30584,"ancestor_record_offset":25380,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":561,"record_offset":30780,"parent_record_offset":30584,"ancestor_record_offset":25380,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":561,"record_offset":30640,"parent_record_offset":30584,"ancestor_record_offset":25380,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":561,"record_offset":31128,"parent_record_offset":31104,"ancestor_record_offset":25380,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":561,"record_offset":30476,"parent_record_offset":30452,"ancestor_record_offset":25380,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":561,"record_offset":31104,"parent_record_offset":30996,"ancestor_record_offset":25380,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":561,"record_offset":34820,"parent_record_offset":34796,"ancestor_record_offset":25380,"inlinee":12036,"source_line":4649,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":561,"record_offset":30508,"parent_record_offset":30476,"ancestor_record_offset":25380,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":31044,"parent_record_offset":31020,"ancestor_record_offset":25380,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":31340,"parent_record_offset":30584,"ancestor_record_offset":25380,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":31020,"parent_record_offset":30996,"ancestor_record_offset":25380,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":30560,"parent_record_offset":30536,"ancestor_record_offset":25380,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":30536,"parent_record_offset":30476,"ancestor_record_offset":25380,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":30584,"parent_record_offset":30560,"ancestor_record_offset":25380,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":30668,"parent_record_offset":30584,"ancestor_record_offset":25380,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":561,"record_offset":30996,"parent_record_offset":30584,"ancestor_record_offset":25380,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$10::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":561,"record_offset":34796,"parent_record_offset":34772,"ancestor_record_offset":25380,"inlinee":12034,"source_line":1307,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":561,"record_offset":30452,"parent_record_offset":30428,"ancestor_record_offset":25380,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":561,"record_offset":34772,"parent_record_offset":34648,"ancestor_record_offset":25380,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":561,"record_offset":35076,"parent_record_offset":35052,"ancestor_record_offset":25380,"inlinee":11242,"source_line":699,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sin_cos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":561,"record_offset":35052,"parent_record_offset":35028,"ancestor_record_offset":25380,"inlinee":11739,"source_line":910,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":561,"record_offset":27632,"parent_record_offset":27600,"ancestor_record_offset":25380,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":561,"record_offset":32696,"parent_record_offset":32672,"ancestor_record_offset":25380,"inlinee":17826,"source_line":6,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":561,"record_offset":32820,"parent_record_offset":32752,"ancestor_record_offset":25380,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$48::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":561,"record_offset":32396,"parent_record_offset":32372,"ancestor_record_offset":25380,"inlinee":35236,"source_line":61,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":561,"record_offset":32296,"parent_record_offset":32272,"ancestor_record_offset":25380,"inlinee":35238,"source_line":16,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":561,"record_offset":32172,"parent_record_offset":32148,"ancestor_record_offset":25380,"inlinee":35234,"source_line":114,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::ui_move_wall_group::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 562
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8997
// 首条位置: {"module":561,"inlinee":35223}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::ui_move_wall_group::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 579
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8998
// 首条位置: {"module":561,"inlinee":35224}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":561,"record_offset":28916,"parent_record_offset":28892,"ancestor_record_offset":28892,"inlinee":28143,"source_line":287,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":561,"record_offset":28940,"parent_record_offset":28916,"ancestor_record_offset":28892,"inlinee":28114,"source_line":165,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":561,"record_offset":28972,"parent_record_offset":28940,"ancestor_record_offset":28892,"inlinee":28109,"source_line":14,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::ui_move_wall_group::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 593
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x143B9
// 首条位置: {"module":1682,"inlinee":82873}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":194740,"parent_record_offset":190804,"ancestor_record_offset":190804,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_wall_state::impl$45::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":1682,"record_offset":191856,"parent_record_offset":190804,"ancestor_record_offset":190804,"inlinee":30580,"source_line":115,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":1682,"record_offset":190888,"parent_record_offset":190804,"ancestor_record_offset":190804,"inlinee":29283,"source_line":128,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":1682,"record_offset":193732,"parent_record_offset":190804,"ancestor_record_offset":190804,"inlinee":28986,"source_line":38,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":192432,"parent_record_offset":192408,"ancestor_record_offset":190804,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":192456,"parent_record_offset":192432,"ancestor_record_offset":190804,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":193592,"parent_record_offset":193568,"ancestor_record_offset":190804,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":193616,"parent_record_offset":193592,"ancestor_record_offset":190804,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":192480,"parent_record_offset":192456,"ancestor_record_offset":190804,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":193640,"parent_record_offset":193616,"ancestor_record_offset":190804,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":192408,"parent_record_offset":192228,"ancestor_record_offset":190804,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":193568,"parent_record_offset":193480,"ancestor_record_offset":190804,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":192168,"parent_record_offset":192140,"ancestor_record_offset":190804,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":193508,"parent_record_offset":193480,"ancestor_record_offset":190804,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":193480,"parent_record_offset":193456,"ancestor_record_offset":190804,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":192228,"parent_record_offset":192196,"ancestor_record_offset":190804,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":192196,"parent_record_offset":192168,"ancestor_record_offset":190804,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":193456,"parent_record_offset":193432,"ancestor_record_offset":190804,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":192264,"parent_record_offset":192228,"ancestor_record_offset":190804,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":192712,"parent_record_offset":192640,"ancestor_record_offset":190804,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":192092,"parent_record_offset":192020,"ancestor_record_offset":190804,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1682,"record_offset":192064,"parent_record_offset":192020,"ancestor_record_offset":190804,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1682,"record_offset":192140,"parent_record_offset":192092,"ancestor_record_offset":190804,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1682,"record_offset":191984,"parent_record_offset":191924,"ancestor_record_offset":190804,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1682,"record_offset":192020,"parent_record_offset":191984,"ancestor_record_offset":190804,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1682,"record_offset":192288,"parent_record_offset":192264,"ancestor_record_offset":190804,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1682,"record_offset":191612,"parent_record_offset":191588,"ancestor_record_offset":190804,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1682,"record_offset":191180,"parent_record_offset":191156,"ancestor_record_offset":190804,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1682,"record_offset":191264,"parent_record_offset":191240,"ancestor_record_offset":190804,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1682,"record_offset":191680,"parent_record_offset":191044,"ancestor_record_offset":190804,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1682,"record_offset":192368,"parent_record_offset":192312,"ancestor_record_offset":190804,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1682,"record_offset":193212,"parent_record_offset":193164,"ancestor_record_offset":190804,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":1682,"record_offset":193164,"parent_record_offset":192712,"ancestor_record_offset":190804,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1682,"record_offset":193364,"parent_record_offset":192712,"ancestor_record_offset":190804,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1682,"record_offset":192312,"parent_record_offset":192288,"ancestor_record_offset":190804,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1682,"record_offset":192340,"parent_record_offset":192312,"ancestor_record_offset":190804,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1682,"record_offset":193536,"parent_record_offset":193508,"ancestor_record_offset":190804,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1682,"record_offset":191412,"parent_record_offset":191384,"ancestor_record_offset":190804,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1682,"record_offset":191384,"parent_record_offset":191356,"ancestor_record_offset":190804,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1682,"record_offset":192536,"parent_record_offset":192092,"ancestor_record_offset":190804,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":191204,"parent_record_offset":191180,"ancestor_record_offset":190804,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":193408,"parent_record_offset":191924,"ancestor_record_offset":190804,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1682,"record_offset":191528,"parent_record_offset":191504,"ancestor_record_offset":190804,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1682,"record_offset":193324,"parent_record_offset":193256,"ancestor_record_offset":190804,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1682,"record_offset":193296,"parent_record_offset":193256,"ancestor_record_offset":190804,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1682,"record_offset":193256,"parent_record_offset":193212,"ancestor_record_offset":190804,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1682,"record_offset":191636,"parent_record_offset":191612,"ancestor_record_offset":190804,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::impl$3::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1682,"record_offset":193700,"parent_record_offset":191856,"ancestor_record_offset":190804,"inlinee":30582,"source_line":11,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1682,"record_offset":191324,"parent_record_offset":191296,"ancestor_record_offset":190804,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1682,"record_offset":191356,"parent_record_offset":191324,"ancestor_record_offset":190804,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1682,"record_offset":191296,"parent_record_offset":191044,"ancestor_record_offset":190804,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":191156,"parent_record_offset":191044,"ancestor_record_offset":190804,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":191708,"parent_record_offset":191044,"ancestor_record_offset":190804,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":191240,"parent_record_offset":191044,"ancestor_record_offset":190804,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1682,"record_offset":191100,"parent_record_offset":191044,"ancestor_record_offset":190804,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1682,"record_offset":191588,"parent_record_offset":191564,"ancestor_record_offset":190804,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1682,"record_offset":190936,"parent_record_offset":190912,"ancestor_record_offset":190804,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1682,"record_offset":191564,"parent_record_offset":191456,"ancestor_record_offset":190804,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1682,"record_offset":190968,"parent_record_offset":190936,"ancestor_record_offset":190804,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":191504,"parent_record_offset":191480,"ancestor_record_offset":190804,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":191800,"parent_record_offset":191044,"ancestor_record_offset":190804,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":191480,"parent_record_offset":191456,"ancestor_record_offset":190804,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":191020,"parent_record_offset":190996,"ancestor_record_offset":190804,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":190996,"parent_record_offset":190936,"ancestor_record_offset":190804,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":191044,"parent_record_offset":191020,"ancestor_record_offset":190804,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":191128,"parent_record_offset":191044,"ancestor_record_offset":190804,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":191456,"parent_record_offset":191044,"ancestor_record_offset":190804,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1682,"record_offset":190912,"parent_record_offset":190888,"ancestor_record_offset":190804,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1682,"record_offset":194708,"parent_record_offset":193732,"ancestor_record_offset":190804,"inlinee":28987,"source_line":165,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$45::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1682,"record_offset":191924,"parent_record_offset":191856,"ancestor_record_offset":190804,"inlinee":12078,"source_line":61,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::ui_move_wall_group::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 676
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x89DD
// 首条位置: {"module":561,"inlinee":35293}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::ui_move_wall_group::WallGroupMember::fits_inside_glade
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":561,"record_offset":89732,"parent_record_offset":89704,"ancestor_record_offset":89704,"inlinee":35294,"source_line":367,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":561,"record_offset":89836,"parent_record_offset":89804,"ancestor_record_offset":89704,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":561,"record_offset":89804,"parent_record_offset":89764,"ancestor_record_offset":89704,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::glade_border::GladeBorder::is_curve2_inside
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/glade_border.rs
//     evidence={"module":561,"record_offset":89764,"parent_record_offset":89732,"ancestor_record_offset":89704,"inlinee":35295,"source_line":54,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::glade_border::GladeBorder::is_pos_inside
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/glade_border.rs
//     evidence={"module":561,"record_offset":89876,"parent_record_offset":89764,"ancestor_record_offset":89704,"inlinee":28651,"source_line":40,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::valid_enclosure::ShapeParameters::contains_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":561,"record_offset":89912,"parent_record_offset":89876,"ancestor_record_offset":89704,"inlinee":28653,"source_line":125,"archive":"_pdb/C/modules/00561.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::ui_move_wall_group::ui_move_wall_group::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 698
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1436C
// 首条位置: {"module":1682,"inlinee":82796}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1682,"record_offset":42460,"parent_record_offset":41404,"ancestor_record_offset":41404,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":1682,"record_offset":41444,"parent_record_offset":41404,"ancestor_record_offset":41404,"inlinee":29283,"source_line":128,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui_move_wall_group::WallGroupMember::tree_area
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui_move_wall_group.rs
//     evidence={"module":1682,"record_offset":42420,"parent_record_offset":41404,"ancestor_record_offset":41404,"inlinee":35229,"source_line":417,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1682,"record_offset":42176,"parent_record_offset":42152,"ancestor_record_offset":41404,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1682,"record_offset":41744,"parent_record_offset":41720,"ancestor_record_offset":41404,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1682,"record_offset":41828,"parent_record_offset":41804,"ancestor_record_offset":41404,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1682,"record_offset":42244,"parent_record_offset":41608,"ancestor_record_offset":41404,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1682,"record_offset":41976,"parent_record_offset":41948,"ancestor_record_offset":41404,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1682,"record_offset":41948,"parent_record_offset":41920,"ancestor_record_offset":41404,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":41768,"parent_record_offset":41744,"ancestor_record_offset":41404,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1682,"record_offset":42092,"parent_record_offset":42068,"ancestor_record_offset":41404,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1682,"record_offset":42200,"parent_record_offset":42176,"ancestor_record_offset":41404,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1682,"record_offset":41888,"parent_record_offset":41860,"ancestor_record_offset":41404,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1682,"record_offset":41920,"parent_record_offset":41888,"ancestor_record_offset":41404,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1682,"record_offset":41860,"parent_record_offset":41608,"ancestor_record_offset":41404,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":41720,"parent_record_offset":41608,"ancestor_record_offset":41404,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":42272,"parent_record_offset":41608,"ancestor_record_offset":41404,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1682,"record_offset":41804,"parent_record_offset":41608,"ancestor_record_offset":41404,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1682,"record_offset":41664,"parent_record_offset":41608,"ancestor_record_offset":41404,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1682,"record_offset":42152,"parent_record_offset":42128,"ancestor_record_offset":41404,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1682,"record_offset":41500,"parent_record_offset":41472,"ancestor_record_offset":41404,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1682,"record_offset":42128,"parent_record_offset":42020,"ancestor_record_offset":41404,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1682,"record_offset":41532,"parent_record_offset":41500,"ancestor_record_offset":41404,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":42068,"parent_record_offset":42044,"ancestor_record_offset":41404,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":42364,"parent_record_offset":41608,"ancestor_record_offset":41404,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":42044,"parent_record_offset":42020,"ancestor_record_offset":41404,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":41584,"parent_record_offset":41560,"ancestor_record_offset":41404,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":41560,"parent_record_offset":41500,"ancestor_record_offset":41404,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":41608,"parent_record_offset":41584,"ancestor_record_offset":41404,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":41692,"parent_record_offset":41608,"ancestor_record_offset":41404,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1682,"record_offset":42020,"parent_record_offset":41608,"ancestor_record_offset":41404,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1682,"record_offset":41472,"parent_record_offset":41444,"ancestor_record_offset":41404,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01682.jsonl.gz"}
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
// 完整结构及原始类型记录: ui_move_wall_group.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::ui_move_wall_group::WallGroupGizmoPlacement
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::ui_move_wall_group::WallGroupMember
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 9; 过程引用数: 15
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::systems::ui_move_wall_group::WallGroupMember", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>", "constructor": "core::slice::iter::Iter", "arguments": ["country_core::systems::ui_move_wall_group::WallGroupMember"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::zip::Zip<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,alloc::vec::into_iter::IntoIter<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global> >", "constructor": "core::iter::adapters::zip::Zip", "arguments": ["core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>", "alloc::vec::into_iter::IntoIter<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::zip::Zip<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,alloc::vec::into_iter::IntoIter<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::zip::Zip<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,alloc::vec::into_iter::IntoIter<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::ui_move_wall_group::WallGroupMember>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::ui_move_wall_group::WallGroupMember"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::impl$46::from_iter::closure_env$0<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::vec::Vec<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2> >", "constructor": "core::option::impl$46::from_iter::closure_env$0", "arguments": ["country_core::systems::ui_move_wall_group::WallGroupMember", "alloc::vec::Vec<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::alloc::Global>", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::try_process<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>,country_core::systems::ui_move_wall_group::WallGroupMember,enum2$<core::option::Option<enum2$<core::convert::Infallible> > >,core::option::impl$46::from_iter::closure_env$0<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::vec::Vec<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2> >,alloc::vec::Vec<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::alloc::Global> >", "constructor": "core::iter::adapters::try_process", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>", "country_core::systems::ui_move_wall_group::WallGroupMember", "enum2$<core::option::Option<enum2$<core::convert::Infallible> > >", "core::option::impl$46::from_iter::closure_env$0<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::vec::Vec<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2> >", "alloc::vec::Vec<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::clear<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::alloc::Global>", "constructor": "alloc::vec::Vec::clear", "arguments": ["country_core::systems::ui_move_wall_group::WallGroupMember", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>", "country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<f32> >,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$2> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<core::option::Option<f32> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>", "country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::inner_wall_state::WallState,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::walls::inner_wall_state::WallState", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ui_move_wall_group::WallGroupMember,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::ui_move_wall_group::WallGroupMember", "core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::notify_circles_rotated::closure_env$0>", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>", "country_core::systems::ui_move_wall_group::impl$0::notify_circles_rotated::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::WallId,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::notify_circles_rotated::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::resources::walls::WallId", "core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::notify_circles_rotated::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>", "country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<country_core::resources::tree_storage::WallGroupTreeArea>", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["enum2$<country_core::resources::tree_storage::WallGroupTreeArea>", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["enum2$<country_core::resources::tree_storage::WallGroupTreeArea>", "alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["enum2$<country_core::resources::tree_storage::WallGroupTreeArea>", "core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>", "country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>", "country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["country_core::resources::walls::inner_wall_state::WallState", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["country_core::resources::walls::inner_wall_state::WallState", "alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<country_core::resources::walls::inner_wall_state::WallState,core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["country_core::resources::walls::inner_wall_state::WallState", "core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>", "country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<country_core::systems::ui_move_wall_group::WallGroupMember>", "constructor": "core::option::Option", "arguments": ["country_core::systems::ui_move_wall_group::WallGroupMember"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<country_core::systems::ui_move_wall_group::WallGroupMember> >", "constructor": "enum2$", "arguments": ["core::option::Option<country_core::systems::ui_move_wall_group::WallGroupMember>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> >", "constructor": "core::ops::control_flow::ControlFlow", "arguments": ["country_core::systems::ui_move_wall_group::WallGroupMember", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >", "constructor": "enum2$", "arguments": ["core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::systems::ui_move_wall_group::WallGroupMember,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > (*)(country_core::systems::ui_move_wall_group::WallGroupMember)>", "constructor": "core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0", "arguments": ["country_core::systems::ui_move_wall_group::WallGroupMember", "enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >", "enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > (*)(country_core::systems::ui_move_wall_group::WallGroupMember)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::systems::ui_move_wall_group::WallGroupMember,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > (*)(country_core::systems::ui_move_wall_group::WallGroupMember)>,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > >", "constructor": "core::iter::adapters::impl$0::try_fold::closure_env$0", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>", "enum2$<core::option::Option<enum2$<core::convert::Infallible> > >", "tuple$<>", "core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::systems::ui_move_wall_group::WallGroupMember,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > (*)(country_core::systems::ui_move_wall_group::WallGroupMember)>", "enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,tuple$<> >", "constructor": "core::ops::control_flow::ControlFlow", "arguments": ["enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,tuple$<> > >", "constructor": "enum2$", "arguments": ["core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<enum2$<core::option::Option<country_core::systems::ui_move_wall_group::WallGroupMember> >,core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::systems::ui_move_wall_group::WallGroupMember,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > (*)(country_core::systems::ui_move_wall_group::WallGroupMember)>,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,tuple$<> > > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["enum2$<core::option::Option<country_core::systems::ui_move_wall_group::WallGroupMember> >", "core::slice::iter::Iter<country_core::resources::walls::WallId>", "country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2", "tuple$<>", "core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::systems::ui_move_wall_group::WallGroupMember,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > (*)(country_core::systems::ui_move_wall_group::WallGroupMember)>,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > >", "enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::ui_move_wall_group::WallGroupTransformCache
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 34
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["country_core::systems::ui_move_wall_group::WallGroupTransformCache"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >", "constructor": "tuple$", "arguments": ["bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>", "bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)", "tuple$<>", "void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::ui_move_wall_group::WallGroupTransformCache>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::ui_move_wall_group::WallGroupTransformCache"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)", "tuple$<>", "void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)", "tuple$<>", "void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)", "tuple$<>", "void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)", "tuple$<>", "void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)", "tuple$<>", "void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)", "tuple$<>", "void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)", "tuple$<>", "void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)", "tuple$<>", "void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)", "tuple$<>", "void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::name<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>", "constructor": "bevy_ecs::system::function_system::impl$12::name", "arguments": ["void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)", "tuple$<>", "void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)", "tuple$<>", "void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$130::init_access<tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio> >", "constructor": "bevy_ecs::system::system_param::impl$130::init_access", "arguments": ["tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources", "tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >", "tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ui_move_wall_group.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ui_move_wall_group.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 32; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<alloc::vec::Vec<country_core::resources::clutter_packs::ClutterPackObject,alloc::alloc::Global> > > (*)(ref$<tg_mods::overlay::Mod>)
// 候选参数类型: ["ref$<tg_mods::overlay::Mod>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::sparse_instance_buffer::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] country_core::systems::wall::structure_outline::WallOutline (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// [文件级接口候选，具体函数归属 unknown] tuple$<bool,tuple$<> > (*)(ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>,enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >,ref$<bevy_ecs::component::info::Components>,ref$<slice2$<bevy_ecs::component::info::ComponentId> >)
// 候选参数类型: ["ref_mut$<bevy_ecs::storage::sparse_set::SparseSets>", "enum2$<core::option::Option<ref_mut$<bevy_ecs::storage::table::Table> > >", "ref$<bevy_ecs::component::info::Components>", "ref$<slice2$<bevy_ecs::component::info::ComponentId> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] relative_path::relative_path_buf::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr> (*)(country_core::resources::walls::WallId)
// 候选参数类型: ["country_core::resources::walls::WallId"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::WallGroupGizmoPlacement::new
// 信息 ID: 66a20b2ff4d20fddde2c0430
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_elevate_wall_group::ui_elevate_wall_group @ 0x933d17 -> 0xb14760 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_wall_group::ui_move_wall_group @ 0xb1283b -> 0xb14760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::WallGroupMember::fits_inside_glade
// 信息 ID: 23202ff9914a0debf53f86e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::slice::iter::impl$171::all<country_core::resources::walls::inner_wall_state::WallState,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$3>; module=561, inline_record=89732; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::WallGroupMember::geometry_changed
// 信息 ID: 83db9cb581b38274da012863
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::ui_move_wall_group; module=561, inline_record=32020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::WallGroupMember::history_edit
// 信息 ID: 839cf195f3c6e8e28907a79b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::ui_move_wall_group; module=561, inline_record=32752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::WallGroupMember::max_bottom_y_at
// 信息 ID: 3d411c2bff961d74d14fa9d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<f32> >,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$2> >; module=1643, inline_record=424804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::WallGroupMember::new
// 信息 ID: 53c1cbd1ff7b634095ab8055
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c5c60f15599d089557c43d58: enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > (*)(country_core::systems::ui_move_wall_group::WallGroupMember)
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::try_fold<enum2$<core::option::Option<country_core::systems::ui_move_wall_group::WallGroupMember> >,core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::systems::ui_move_wall_group::WallGroupMember,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > (*)(country_core::systems::ui_move_wall_group::WallGroupMember)>,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,tuple$<> > > > @ 0x1816337 -> 0xb11d60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::WallGroupMember::shape_moved
// 信息 ID: d9b0230ef92fc816eeb79844
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::WallGroupTransformCache::notify_shape_moved; module=561, inline_record=48440; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::WallGroupTransformCache::apply; module=561, inline_record=51504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::WallGroupMember::target_state
// 信息 ID: ac5cf9c739ef708bd2eac770
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::inner_wall_state::WallState,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$1> > @ 0x1606fca -> 0xb11920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::WallGroupMember::tree_area
// 信息 ID: 9de3d227584ad89eead5ed87
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::ui_move_wall_group; module=561, inline_record=31396; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::WallGroupTransformCache::tree_areas; module=561, inline_record=43236; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4> > > >; module=1682, inline_record=42420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::WallGroupTransformCache::apply
// 信息 ID: 70ee265551b36e56e49c7c6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::WallGroupTransformCache::cursor_position
// 信息 ID: 85e8ac28be55084da43b9a3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui_move_wall_group::ui_move_wall_group @ 0xb13758 -> 0xb15300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::WallGroupTransformCache::notify_circles_rotated
// 信息 ID: d0bd60ea8a0c1695192037dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::WallGroupTransformCache::notify_shape_moved; module=561, inline_record=49488; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::WallGroupTransformCache::apply; module=561, inline_record=54632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::WallGroupTransformCache::notify_shape_moved
// 信息 ID: 95655dc43414c1ca0b80191d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::WallGroupTransformCache::target_states
// 信息 ID: 8a15d7d7b791c74556a0fb75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::WallGroupTransformCache::tree_areas
// 信息 ID: e5f78a52eef198d7e82d1ca3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::group_wire_rms_displacement
// 信息 ID: 09e0f9c9f5bea5c9f09e3f01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::impl$0::notify_circles_rotated::closure$0
// 信息 ID: fbe81c573a72b9183d7ffe8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ui_move_wall_group::impl$0::notify_circles_rotated", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::impl$0::notify_circles_rotated::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::WallId,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::notify_circles_rotated::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::WallId,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::notify_circles_rotated::closure_env$0> >; module=1643, inline_record=579820; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::WallId,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::notify_circles_rotated::closure_env$0> >; module=1643, inline_record=580420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::impl$0::target_states::closure$0
// 信息 ID: d6537f19a68ffa635ac046d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ui_move_wall_group::impl$0::target_states", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0 <- core::iter::adapters::map::impl$2::fold<country_core::resources::walls::inner_wall_state::WallState,core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0 <- core::iter::adapters::map::impl$2::fold<country_core::resources::walls::inner_wall_state::WallState,core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<country_core::resources::walls::inner_wall_state::WallState,core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::resources::walls::inner_wall_state::WallState,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$0> > > >; module=1682, inline_record=88088; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::impl$0::target_states::closure$1
// 信息 ID: 461bb0e12c7f0fbcdbc1ccb1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::ui_move_wall_group::impl$0::target_states", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::inner_wall_state::WallState,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::walls::inner_wall_state::WallState,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$1> >; module=1643, inline_record=496556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::impl$0::target_states::closure$2
// 信息 ID: 8177b8399db2d934a5fbd0be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::ui_move_wall_group::impl$0::target_states", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<f32> >,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<f32> >,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$2> >; module=1643, inline_record=424776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::impl$0::tree_areas::closure$0
// 信息 ID: 7026c2efb4e346d4a8b5a037
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ui_move_wall_group::impl$0::tree_areas", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::WallGroupTransformCache::tree_areas; module=561, inline_record=43212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::impl$1::max_bottom_y_at::closure$0
// 信息 ID: 511e4bb4c0e6f15ea7dbf6d2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ui_move_wall_group::impl$1::max_bottom_y_at", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<f32> >,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$2> >; module=1643, inline_record=425008; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<f32> >,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::impl$0::target_states::closure_env$2> >; module=1643, inline_record=425608; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::impl$1::new::closure$0
// 信息 ID: be762463016a1b8e5bcd9b09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ui_move_wall_group::impl$1::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::WallGroupMember::new; module=561, inline_record=24092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::impl$1::shape_moved::closure$0
// 信息 ID: 439cc70194e6808e7d3aa9e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ui_move_wall_group::impl$1::shape_moved", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::WallGroupTransformCache::notify_shape_moved; module=561, inline_record=49192; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::WallGroupTransformCache::apply; module=561, inline_record=52272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::impl$2::new::closure$0
// 信息 ID: eea198a88671755f9a4c250a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 89cabf0a70d8d22da18ef881: enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ui_move_wall_group::impl$2::new", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::impl$2::new::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::impl$2::new::closure_env$0>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::impl$2::new::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::option::Option<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics> > >,core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::impl$2::new::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::impl$2::new::closure_env$0>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)>,enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::impl$2::new::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::option::Option<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics> > >,core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::impl$2::new::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::impl$2::new::closure_env$0>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)>,enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::option::Option<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics> > >,core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::impl$2::new::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::impl$2::new::closure_env$0>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)>,enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > >,tuple$<> > > >; module=1682, inline_record=136396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::impl$2::new::closure$2
// 信息 ID: d3ca662c191b48d95983dd66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::ui_move_wall_group::impl$2::new", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::WallGroupGizmoPlacement::new; module=561, inline_record=41008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::impl$3::default
// 信息 ID: 19a0d09edc7ddffc883da2d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] caf77d0562d5c83942cc5de4: void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$12::initialize<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)>; module=1718, inline_record=10408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::ui_move_wall_group
// 信息 ID: 22974486bd054e80f58b0d90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] caf77d0562d5c83942cc5de4: void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> > @ 0x190f8ee -> 0xb122e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>),tuple$<>,void (*)(tuple$<bevy_ecs::system::system_param::Local<country_core::systems::ui_move_wall_group::WallGroupTransformCache>,bevy_ecs::system::system_param::Local<country_core::systems::ui_wall_transform_gizmos::WallRotationGizmoState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,tuple$<bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream> >,tuple$<bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime> >,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder> >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnShapeMoved>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::events::OnCircleRotated>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::tree_storage::HideTreesForWallGroupCmd> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::wall_group_move_audio::RequestWallGroupMoveAudio>)> @ 0x19d5c45 -> 0xb122e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::ui_move_wall_group::closure$0
// 信息 ID: cd5e8f9e3e4ed82672e4aee6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::ui_move_wall_group::ui_move_wall_group", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::ui_move_wall_group; module=561, inline_record=28792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::ui_move_wall_group::closure$1
// 信息 ID: 9ef0ccb2a7fd30c9a51037da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::ui_move_wall_group::ui_move_wall_group", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui_move_wall_group::ui_move_wall_group; module=561, inline_record=28892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::ui_move_wall_group::closure$2
// 信息 ID: 3f2f2673f524f8d69e56288c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c5c60f15599d089557c43d58: enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > (*)(country_core::systems::ui_move_wall_group::WallGroupMember)
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::ui_move_wall_group::ui_move_wall_group", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2 <- core::iter::adapters::try_process<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>,country_core::systems::ui_move_wall_group::WallGroupMember,enum2$<core::option::Option<enum2$<core::convert::Infallible> > >,core::option::impl$46::from_iter::closure_env$0<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::vec::Vec<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2> >,alloc::vec::Vec<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2 <- core::iter::adapters::try_process<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>,country_core::systems::ui_move_wall_group::WallGroupMember,enum2$<core::option::Option<enum2$<core::convert::Infallible> > >,core::option::impl$46::from_iter::closure_env$0<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::vec::Vec<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2> >,alloc::vec::Vec<country_core::systems::ui_move_wall_group::WallGroupMember,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::ui_move_wall_group::WallGroupMember,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::option::Option<country_core::systems::ui_move_wall_group::WallGroupMember> >,core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::systems::ui_move_wall_group::WallGroupMember,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > (*)(country_core::systems::ui_move_wall_group::WallGroupMember)>,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::option::Option<country_core::systems::ui_move_wall_group::WallGroupMember> >,core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::systems::ui_move_wall_group::WallGroupMember,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > (*)(country_core::systems::ui_move_wall_group::WallGroupMember)>,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::option::Option<country_core::systems::ui_move_wall_group::WallGroupMember> >,core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::walls::WallId>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$2>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::systems::ui_move_wall_group::WallGroupMember,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > (*)(country_core::systems::ui_move_wall_group::WallGroupMember)>,enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::systems::ui_move_wall_group::WallGroupMember,tuple$<> > >,tuple$<> > > >; module=1682, inline_record=190804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::ui_move_wall_group::closure$3
// 信息 ID: 34ffa0126adf1484e46f7f47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::systems::ui_move_wall_group::ui_move_wall_group", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$3 <- core::slice::iter::impl$171::all<country_core::resources::walls::inner_wall_state::WallState,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$3>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::slice::iter::impl$171::all<country_core::resources::walls::inner_wall_state::WallState,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$3>; module=561, inline_record=89704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::ui_move_wall_group::ui_move_wall_group::closure$4
// 信息 ID: b2d84dde808f5b76aa23dbdb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "country_core::systems::ui_move_wall_group::ui_move_wall_group", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4 <- alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4 <- core::iter::adapters::map::impl$2::fold<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4 <- core::iter::adapters::map::impl$2::fold<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::vec::impl$21::extend_trusted::closure_env$0<enum2$<country_core::resources::tree_storage::WallGroupTreeArea>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::systems::ui_move_wall_group::WallGroupMember>,country_core::systems::ui_move_wall_group::ui_move_wall_group::closure_env$4> > > >; module=1682, inline_record=41404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
