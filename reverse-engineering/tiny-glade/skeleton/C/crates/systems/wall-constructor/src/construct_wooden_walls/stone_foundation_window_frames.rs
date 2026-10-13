// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 4e181dcb31b1c2ceed3fe54ae14ccff8e83b698d76f112236ff3ff3aebf33e50
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: stone_foundation_window_frames.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1462, 1482, 1495, 3157
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: [{"name":"system-wall-constructor","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1495,"record_offset":5452,"rva":21367088,"va":5390076208}
// 编译实例 RVAs: 0x1460930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=28
//     evidence={"assembly":"\tcallq\t-14400159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8614524ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390078858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de62a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6794504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8f85298ff","size":5,"soft_fail":false,"target_va":5383283360,"va":5390077859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f7ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-434910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8225df9ff","size":5,"soft_fail":false,"target_va":5389646512,"va":5390081417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f7bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-433384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e81863f9ff","size":5,"soft_fail":false,"target_va":5389646832,"va":5390080211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413f9e10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-424579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e87d85f9ff","size":5,"soft_fail":false,"target_va":5389655568,"va":5390080142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fa430
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-421499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e88591f9ff","size":5,"soft_fail":false,"target_va":5389657136,"va":5390078630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413fb040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-421118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e80293f9ff","size":5,"soft_fail":false,"target_va":5389660224,"va":5390081337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14142b2c0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-219742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8a2a5fcff","size":5,"soft_fail":false,"target_va":5389857472,"va":5390077209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1424e32b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17311549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e83d270801","size":5,"soft_fail":false,"target_va":5407388336,"va":5390076782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26981125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e805b39b01","size":5,"soft_fail":false,"target_va":5417057792,"va":5390076662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26539519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8fff59401","size":5,"soft_fail":false,"target_va":5416621600,"va":5390082076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26540100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e844f89401","size":5,"soft_fail":false,"target_va":5416622208,"va":5390082103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26539585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e841f69401","size":5,"soft_fail":false,"target_va":5416621762,"va":5390082172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_door_outline_profile_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11213828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8fce354ff","size":5,"soft_fail":false,"target_va":5378864192,"va":5390078015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_window_outline_profile_curve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11215447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8a9dd54ff","size":5,"soft_fail":false,"target_va":5378864592,"va":5390080034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stone_foundation_frames::cottage_corner_window_foundation_size
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8845428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e88c0779ff","size":5,"soft_fail":false,"target_va":5381231376,"va":5390076799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stone_foundation_frames::frame_bottom_in_stone_foundation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8846031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8310579ff","size":5,"soft_fail":false,"target_va":5381231344,"va":5390077370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stone_foundation_frames::stone_foundation_replaces_gothic_half_dormer_hat
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8845990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e85a0579ff","size":5,"soft_fail":false,"target_va":5381231440,"va":5390077425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::try_dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11949677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e893a949ff","size":5,"soft_fail":false,"target_va":5378126928,"va":5390076600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8422217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8b77c7fff","size":5,"soft_fail":false,"target_va":5381654320,"va":5390076532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::u_space
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8842967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8291179ff","size":5,"soft_fail":false,"target_va":5381234640,"va":5390077602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t197299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8b3020300","size":5,"soft_fail":false,"target_va":5390273664,"va":5390076360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_x
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t196039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8c7fd0200","size":5,"soft_fail":false,"target_va":5390273920,"va":5390077876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_wooden_walls::construct_wall_boards::WallSampler::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t195396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e844fb0200","size":5,"soft_fail":false,"target_va":5390271776,"va":5390076375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17310672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8d0230801","size":5,"soft_fail":false,"target_va":5407389344,"va":5390078667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line<system_wall_constructor::construct_walls::construct_walls::closure_env$3>
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8d4ecffff","size":5,"soft_fail":false,"target_va":5390073920,"va":5390078823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6775544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8089d98ff","size":5,"soft_fail":false,"target_va":5383302240,"va":5390077779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6775116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e8b49e98ff","size":5,"soft_fail":false,"target_va":5383301904,"va":5390077015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6775428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e87c9d98ff","size":5,"soft_fail":false,"target_va":5383302288,"va":5390077711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6775475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390076208,"raw_hex":"e84d9d98ff","size":5,"soft_fail":false,"target_va":5383302224,"va":5390077694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1495,"record_offset":7864,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":19244,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::DecoratorRank::x
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1495,"record_offset":6732,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":79831,"source_line":266,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1495,"record_offset":8380,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":26107,"source_line":149,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1495,"record_offset":7124,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1495,"record_offset":7180,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1495,"record_offset":6788,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$35::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1495,"record_offset":5816,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":8500,"source_line":2025,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1495,"record_offset":6108,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::construct_wall_boards::hole_overlaps_stone_foundation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1495,"record_offset":7892,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":79833,"source_line":1316,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_cottage_door_frame_bricks
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":12652,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":79837,"source_line":348,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_cottage_window_outline_bricks
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":14428,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":79840,"source_line":279,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":8488,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":79836,"source_line":415,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":16172,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":79841,"source_line":507,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_window_bottom_bricks
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":7988,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":79835,"source_line":715,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::quantized_frame_position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":6588,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":79830,"source_line":962,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::stone_foundation_frame_region
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":6872,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":79832,"source_line":262,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::stone_foundation_window_bottom_frame_piece
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":7920,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":79834,"source_line":699,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1495,"record_offset":7268,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":26598,"source_line":1171,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1495,"record_offset":7568,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":8731,"source_line":1392,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1495,"record_offset":6760,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":11328,"source_line":84,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::as_vec2
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1495,"record_offset":6560,"parent_record_offset":5452,"ancestor_record_offset":5452,"inlinee":27051,"source_line":156,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":5452,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":9352,"parent_record_offset":9328,"ancestor_record_offset":5452,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":5452,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":9304,"parent_record_offset":9212,"ancestor_record_offset":5452,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":9240,"parent_record_offset":9212,"ancestor_record_offset":5452,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":9212,"parent_record_offset":9188,"ancestor_record_offset":5452,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":9188,"parent_record_offset":9164,"ancestor_record_offset":5452,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1495,"record_offset":9748,"parent_record_offset":9724,"ancestor_record_offset":5452,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1495,"record_offset":9948,"parent_record_offset":9920,"ancestor_record_offset":5452,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1495,"record_offset":9972,"parent_record_offset":9948,"ancestor_record_offset":5452,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1495,"record_offset":10088,"parent_record_offset":9996,"ancestor_record_offset":5452,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1495,"record_offset":6024,"parent_record_offset":6000,"ancestor_record_offset":5452,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1495,"record_offset":7480,"parent_record_offset":7268,"ancestor_record_offset":5452,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1495,"record_offset":9892,"parent_record_offset":8488,"ancestor_record_offset":5452,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1495,"record_offset":6640,"parent_record_offset":6616,"ancestor_record_offset":5452,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1495,"record_offset":9920,"parent_record_offset":8488,"ancestor_record_offset":5452,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1495,"record_offset":9772,"parent_record_offset":9748,"ancestor_record_offset":5452,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1495,"record_offset":9724,"parent_record_offset":8488,"ancestor_record_offset":5452,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1495,"record_offset":6904,"parent_record_offset":6872,"ancestor_record_offset":5452,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1495,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":5452,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1495,"record_offset":9272,"parent_record_offset":9240,"ancestor_record_offset":5452,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1495,"record_offset":6308,"parent_record_offset":6284,"ancestor_record_offset":5452,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1495,"record_offset":7208,"parent_record_offset":7180,"ancestor_record_offset":5452,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1495,"record_offset":7148,"parent_record_offset":7124,"ancestor_record_offset":5452,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1495,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":5452,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1495,"record_offset":10032,"parent_record_offset":9996,"ancestor_record_offset":5452,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1495,"record_offset":9996,"parent_record_offset":9972,"ancestor_record_offset":5452,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1495,"record_offset":7380,"parent_record_offset":7344,"ancestor_record_offset":5452,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1495,"record_offset":6056,"parent_record_offset":5972,"ancestor_record_offset":5452,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1495,"record_offset":10172,"parent_record_offset":10112,"ancestor_record_offset":5452,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":9140,"parent_record_offset":9088,"ancestor_record_offset":5452,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1495,"record_offset":16296,"parent_record_offset":16272,"ancestor_record_offset":5452,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1495,"record_offset":6428,"parent_record_offset":6404,"ancestor_record_offset":5452,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":10196,"parent_record_offset":10172,"ancestor_record_offset":5452,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1495,"record_offset":8612,"parent_record_offset":8488,"ancestor_record_offset":5452,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1495,"record_offset":6932,"parent_record_offset":6872,"ancestor_record_offset":5452,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1495,"record_offset":10112,"parent_record_offset":10088,"ancestor_record_offset":5452,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1495,"record_offset":16248,"parent_record_offset":16172,"ancestor_record_offset":5452,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1495,"record_offset":8640,"parent_record_offset":8488,"ancestor_record_offset":5452,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1495,"record_offset":7612,"parent_record_offset":7568,"ancestor_record_offset":5452,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1495,"record_offset":7012,"parent_record_offset":6932,"ancestor_record_offset":5452,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1495,"record_offset":10144,"parent_record_offset":10112,"ancestor_record_offset":5452,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1495,"record_offset":7640,"parent_record_offset":7612,"ancestor_record_offset":5452,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1495,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":5452,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1495,"record_offset":8668,"parent_record_offset":8488,"ancestor_record_offset":5452,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1495,"record_offset":6252,"parent_record_offset":6224,"ancestor_record_offset":5452,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1495,"record_offset":6284,"parent_record_offset":6252,"ancestor_record_offset":5452,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1495,"record_offset":6224,"parent_record_offset":6184,"ancestor_record_offset":5452,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1495,"record_offset":6000,"parent_record_offset":5972,"ancestor_record_offset":5452,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1495,"record_offset":5864,"parent_record_offset":5840,"ancestor_record_offset":5452,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1495,"record_offset":6132,"parent_record_offset":6108,"ancestor_record_offset":5452,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1495,"record_offset":6404,"parent_record_offset":6184,"ancestor_record_offset":5452,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1495,"record_offset":5972,"parent_record_offset":5912,"ancestor_record_offset":5452,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1495,"record_offset":6184,"parent_record_offset":6156,"ancestor_record_offset":5452,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1495,"record_offset":5888,"parent_record_offset":5864,"ancestor_record_offset":5452,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1495,"record_offset":5944,"parent_record_offset":5912,"ancestor_record_offset":5452,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1495,"record_offset":5912,"parent_record_offset":5888,"ancestor_record_offset":5452,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1495,"record_offset":6156,"parent_record_offset":6132,"ancestor_record_offset":5452,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1495,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":5452,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1495,"record_offset":6616,"parent_record_offset":6588,"ancestor_record_offset":5452,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_unwrapped_wallspace_pos
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1495,"record_offset":8108,"parent_record_offset":8068,"ancestor_record_offset":5452,"inlinee":79621,"source_line":1512,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_wallspace_pos
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1495,"record_offset":8160,"parent_record_offset":8108,"ancestor_record_offset":5452,"inlinee":79620,"source_line":1508,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_clipped_brick_line
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":9088,"parent_record_offset":8488,"ancestor_record_offset":5452,"inlinee":79839,"source_line":575,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":13068,"parent_record_offset":12652,"ancestor_record_offset":5452,"inlinee":79838,"source_line":763,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_pos
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":8068,"parent_record_offset":7988,"ancestor_record_offset":5452,"inlinee":79619,"source_line":774,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1495,"record_offset":7676,"parent_record_offset":7568,"ancestor_record_offset":5452,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1495,"record_offset":7536,"parent_record_offset":7268,"ancestor_record_offset":5452,"inlinee":15960,"source_line":338,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1495,"record_offset":7508,"parent_record_offset":7268,"ancestor_record_offset":5452,"inlinee":17828,"source_line":236,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1495,"record_offset":7312,"parent_record_offset":7268,"ancestor_record_offset":5452,"inlinee":15962,"source_line":89,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1495,"record_offset":7344,"parent_record_offset":7312,"ancestor_record_offset":5452,"inlinee":15948,"source_line":110,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1495,"record_offset":7444,"parent_record_offset":7416,"ancestor_record_offset":5452,"inlinee":15961,"source_line":196,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1495,"record_offset":7416,"parent_record_offset":7268,"ancestor_record_offset":5452,"inlinee":26154,"source_line":220,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::stone_foundation_frame_region
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 262
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137D8
// 首条位置: {"module":1495,"inlinee":79832}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1495,"record_offset":6904,"parent_record_offset":6872,"ancestor_record_offset":6872,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1495,"record_offset":6932,"parent_record_offset":6872,"ancestor_record_offset":6872,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1495,"record_offset":7012,"parent_record_offset":6932,"ancestor_record_offset":6872,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_cottage_window_outline_bricks
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 279
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137E0
// 首条位置: {"module":1495,"inlinee":79840}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1495,"record_offset":14516,"parent_record_offset":14428,"ancestor_record_offset":14428,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":15536,"parent_record_offset":14428,"ancestor_record_offset":14428,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_clipped_brick_line
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":14900,"parent_record_offset":14428,"ancestor_record_offset":14428,"inlinee":79839,"source_line":575,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":14788,"parent_record_offset":14428,"ancestor_record_offset":14428,"inlinee":79838,"source_line":763,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":15140,"parent_record_offset":15116,"ancestor_record_offset":14428,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":15164,"parent_record_offset":15140,"ancestor_record_offset":14428,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":15188,"parent_record_offset":15164,"ancestor_record_offset":14428,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":15116,"parent_record_offset":15024,"ancestor_record_offset":14428,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":15052,"parent_record_offset":15024,"ancestor_record_offset":14428,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":15024,"parent_record_offset":15000,"ancestor_record_offset":14428,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":15000,"parent_record_offset":14976,"ancestor_record_offset":14428,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1495,"record_offset":14836,"parent_record_offset":14812,"ancestor_record_offset":14428,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1495,"record_offset":14548,"parent_record_offset":14516,"ancestor_record_offset":14428,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1495,"record_offset":14628,"parent_record_offset":14580,"ancestor_record_offset":14428,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1495,"record_offset":14860,"parent_record_offset":14836,"ancestor_record_offset":14428,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1495,"record_offset":14812,"parent_record_offset":14788,"ancestor_record_offset":14428,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1495,"record_offset":15084,"parent_record_offset":15052,"ancestor_record_offset":14428,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1495,"record_offset":14580,"parent_record_offset":14548,"ancestor_record_offset":14428,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1495,"record_offset":14712,"parent_record_offset":14652,"ancestor_record_offset":14428,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":14952,"parent_record_offset":14900,"ancestor_record_offset":14428,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":14736,"parent_record_offset":14712,"ancestor_record_offset":14428,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1495,"record_offset":14652,"parent_record_offset":14628,"ancestor_record_offset":14428,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1495,"record_offset":14684,"parent_record_offset":14652,"ancestor_record_offset":14428,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_cottage_door_frame_bricks
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 348
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137DD
// 首条位置: {"module":1495,"inlinee":79837}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1495,"record_offset":12736,"parent_record_offset":12652,"ancestor_record_offset":12652,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":13816,"parent_record_offset":12652,"ancestor_record_offset":12652,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_clipped_brick_line
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":13180,"parent_record_offset":12652,"ancestor_record_offset":12652,"inlinee":79839,"source_line":575,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":13068,"parent_record_offset":12652,"ancestor_record_offset":12652,"inlinee":79838,"source_line":763,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":13420,"parent_record_offset":13396,"ancestor_record_offset":12652,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":13444,"parent_record_offset":13420,"ancestor_record_offset":12652,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":13468,"parent_record_offset":13444,"ancestor_record_offset":12652,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":13396,"parent_record_offset":13304,"ancestor_record_offset":12652,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":13332,"parent_record_offset":13304,"ancestor_record_offset":12652,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":13304,"parent_record_offset":13280,"ancestor_record_offset":12652,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":13280,"parent_record_offset":13256,"ancestor_record_offset":12652,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1495,"record_offset":13116,"parent_record_offset":13092,"ancestor_record_offset":12652,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1495,"record_offset":12764,"parent_record_offset":12736,"ancestor_record_offset":12652,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1495,"record_offset":12788,"parent_record_offset":12764,"ancestor_record_offset":12652,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1495,"record_offset":12904,"parent_record_offset":12812,"ancestor_record_offset":12652,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1495,"record_offset":13140,"parent_record_offset":13116,"ancestor_record_offset":12652,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1495,"record_offset":13092,"parent_record_offset":13068,"ancestor_record_offset":12652,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1495,"record_offset":12872,"parent_record_offset":12848,"ancestor_record_offset":12652,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1495,"record_offset":13364,"parent_record_offset":13332,"ancestor_record_offset":12652,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1495,"record_offset":12848,"parent_record_offset":12812,"ancestor_record_offset":12652,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1495,"record_offset":12812,"parent_record_offset":12788,"ancestor_record_offset":12652,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1495,"record_offset":12988,"parent_record_offset":12928,"ancestor_record_offset":12652,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":13232,"parent_record_offset":13180,"ancestor_record_offset":12652,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":13012,"parent_record_offset":12988,"ancestor_record_offset":12652,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1495,"record_offset":12928,"parent_record_offset":12904,"ancestor_record_offset":12652,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1495,"record_offset":12960,"parent_record_offset":12928,"ancestor_record_offset":12652,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 415
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137DC
// 首条位置: {"module":1495,"inlinee":79836}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1495,"record_offset":9892,"parent_record_offset":8488,"ancestor_record_offset":8488,"inlinee":5478,"source_line":1018,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1495,"record_offset":9920,"parent_record_offset":8488,"ancestor_record_offset":8488,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1495,"record_offset":9724,"parent_record_offset":8488,"ancestor_record_offset":8488,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":10884,"parent_record_offset":8488,"ancestor_record_offset":8488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1495,"record_offset":8612,"parent_record_offset":8488,"ancestor_record_offset":8488,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1495,"record_offset":8640,"parent_record_offset":8488,"ancestor_record_offset":8488,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1495,"record_offset":8668,"parent_record_offset":8488,"ancestor_record_offset":8488,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_clipped_brick_line
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":9088,"parent_record_offset":8488,"ancestor_record_offset":8488,"inlinee":79839,"source_line":575,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_pos
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":8724,"parent_record_offset":8488,"ancestor_record_offset":8488,"inlinee":79619,"source_line":774,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":8488,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":9352,"parent_record_offset":9328,"ancestor_record_offset":8488,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":8488,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":9304,"parent_record_offset":9212,"ancestor_record_offset":8488,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":9240,"parent_record_offset":9212,"ancestor_record_offset":8488,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":9212,"parent_record_offset":9188,"ancestor_record_offset":8488,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":9188,"parent_record_offset":9164,"ancestor_record_offset":8488,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1495,"record_offset":9748,"parent_record_offset":9724,"ancestor_record_offset":8488,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1495,"record_offset":9948,"parent_record_offset":9920,"ancestor_record_offset":8488,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1495,"record_offset":9972,"parent_record_offset":9948,"ancestor_record_offset":8488,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1495,"record_offset":10088,"parent_record_offset":9996,"ancestor_record_offset":8488,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1495,"record_offset":9772,"parent_record_offset":9748,"ancestor_record_offset":8488,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1495,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":8488,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1495,"record_offset":9272,"parent_record_offset":9240,"ancestor_record_offset":8488,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1495,"record_offset":8692,"parent_record_offset":8668,"ancestor_record_offset":8488,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1495,"record_offset":10032,"parent_record_offset":9996,"ancestor_record_offset":8488,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1495,"record_offset":9996,"parent_record_offset":9972,"ancestor_record_offset":8488,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1495,"record_offset":10172,"parent_record_offset":10112,"ancestor_record_offset":8488,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":9140,"parent_record_offset":9088,"ancestor_record_offset":8488,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":10196,"parent_record_offset":10172,"ancestor_record_offset":8488,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1495,"record_offset":10112,"parent_record_offset":10088,"ancestor_record_offset":8488,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1495,"record_offset":10144,"parent_record_offset":10112,"ancestor_record_offset":8488,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_unwrapped_wallspace_pos
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1495,"record_offset":8764,"parent_record_offset":8724,"ancestor_record_offset":8488,"inlinee":79621,"source_line":1512,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_wallspace_pos
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1495,"record_offset":8816,"parent_record_offset":8764,"ancestor_record_offset":8488,"inlinee":79620,"source_line":1508,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 470
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13725
// 首条位置: {"module":1482,"inlinee":79653}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1482,"record_offset":37216,"parent_record_offset":37048,"ancestor_record_offset":37048,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1482,"record_offset":37076,"parent_record_offset":37048,"ancestor_record_offset":37048,"inlinee":79619,"source_line":774,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1482,"record_offset":37244,"parent_record_offset":37216,"ancestor_record_offset":37048,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_unwrapped_wallspace_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1482,"record_offset":37136,"parent_record_offset":37076,"ancestor_record_offset":37048,"inlinee":79621,"source_line":1512,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_wallspace_pos
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1482,"record_offset":37108,"parent_record_offset":37076,"ancestor_record_offset":37048,"inlinee":79620,"source_line":1508,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 475
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13726
// 首条位置: {"module":1482,"inlinee":79654}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1482,"record_offset":38968,"parent_record_offset":38800,"ancestor_record_offset":38800,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1482,"record_offset":38828,"parent_record_offset":38800,"ancestor_record_offset":38800,"inlinee":79619,"source_line":774,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1482,"record_offset":38996,"parent_record_offset":38968,"ancestor_record_offset":38800,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_unwrapped_wallspace_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1482,"record_offset":38888,"parent_record_offset":38828,"ancestor_record_offset":38800,"inlinee":79621,"source_line":1512,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_wallspace_pos
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1482,"record_offset":38860,"parent_record_offset":38828,"ancestor_record_offset":38800,"inlinee":79620,"source_line":1508,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 507
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137E1
// 首条位置: {"module":1495,"inlinee":79841}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1495,"record_offset":16500,"parent_record_offset":16172,"ancestor_record_offset":16172,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1495,"record_offset":16332,"parent_record_offset":16172,"ancestor_record_offset":16172,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":17504,"parent_record_offset":16172,"ancestor_record_offset":16172,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1495,"record_offset":16248,"parent_record_offset":16172,"ancestor_record_offset":16172,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_clipped_brick_line
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":16872,"parent_record_offset":16172,"ancestor_record_offset":16172,"inlinee":79839,"source_line":575,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":17108,"parent_record_offset":17084,"ancestor_record_offset":16172,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":17132,"parent_record_offset":17108,"ancestor_record_offset":16172,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":17156,"parent_record_offset":17132,"ancestor_record_offset":16172,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":17084,"parent_record_offset":16992,"ancestor_record_offset":16172,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":17020,"parent_record_offset":16992,"ancestor_record_offset":16172,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":16992,"parent_record_offset":16968,"ancestor_record_offset":16172,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":16968,"parent_record_offset":16944,"ancestor_record_offset":16172,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1495,"record_offset":16356,"parent_record_offset":16332,"ancestor_record_offset":16172,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":1495,"record_offset":16540,"parent_record_offset":16500,"ancestor_record_offset":16172,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1495,"record_offset":16572,"parent_record_offset":16540,"ancestor_record_offset":16172,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":1495,"record_offset":16708,"parent_record_offset":16604,"ancestor_record_offset":16172,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1495,"record_offset":16380,"parent_record_offset":16356,"ancestor_record_offset":16172,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1495,"record_offset":16676,"parent_record_offset":16652,"ancestor_record_offset":16172,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1495,"record_offset":17052,"parent_record_offset":17020,"ancestor_record_offset":16172,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1495,"record_offset":16652,"parent_record_offset":16604,"ancestor_record_offset":16172,"inlinee":5122,"source_line":58,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":1495,"record_offset":16604,"parent_record_offset":16572,"ancestor_record_offset":16172,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1495,"record_offset":16792,"parent_record_offset":16732,"ancestor_record_offset":16172,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":16920,"parent_record_offset":16872,"ancestor_record_offset":16172,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1495,"record_offset":16296,"parent_record_offset":16272,"ancestor_record_offset":16172,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":16816,"parent_record_offset":16792,"ancestor_record_offset":16172,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1495,"record_offset":16732,"parent_record_offset":16708,"ancestor_record_offset":16172,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1495,"record_offset":16764,"parent_record_offset":16732,"ancestor_record_offset":16172,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1495,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":16172,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 528
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13713
// 首条位置: {"module":1482,"inlinee":79635}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1482,"record_offset":26464,"parent_record_offset":26296,"ancestor_record_offset":26296,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1482,"record_offset":26324,"parent_record_offset":26296,"ancestor_record_offset":26296,"inlinee":79619,"source_line":774,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1482,"record_offset":26492,"parent_record_offset":26464,"ancestor_record_offset":26296,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_unwrapped_wallspace_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1482,"record_offset":26384,"parent_record_offset":26324,"ancestor_record_offset":26296,"inlinee":79621,"source_line":1512,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_wallspace_pos
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1482,"record_offset":26356,"parent_record_offset":26324,"ancestor_record_offset":26296,"inlinee":79620,"source_line":1508,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 532
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13702
// 首条位置: {"module":1482,"inlinee":79618}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1482,"record_offset":10416,"parent_record_offset":10248,"ancestor_record_offset":10248,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1482,"record_offset":10276,"parent_record_offset":10248,"ancestor_record_offset":10248,"inlinee":79619,"source_line":774,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1482,"record_offset":10444,"parent_record_offset":10416,"ancestor_record_offset":10248,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_unwrapped_wallspace_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1482,"record_offset":10336,"parent_record_offset":10276,"ancestor_record_offset":10248,"inlinee":79621,"source_line":1512,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_wallspace_pos
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1482,"record_offset":10308,"parent_record_offset":10276,"ancestor_record_offset":10248,"inlinee":79620,"source_line":1508,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_clipped_brick_line
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 575
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137DF
// 首条位置: {"module":1495,"inlinee":79839}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":9140,"parent_record_offset":9088,"ancestor_record_offset":9088,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":9088,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":9352,"parent_record_offset":9328,"ancestor_record_offset":9088,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":9088,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":9304,"parent_record_offset":9212,"ancestor_record_offset":9088,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":9240,"parent_record_offset":9212,"ancestor_record_offset":9088,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":9212,"parent_record_offset":9188,"ancestor_record_offset":9088,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":9188,"parent_record_offset":9164,"ancestor_record_offset":9088,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1495,"record_offset":9272,"parent_record_offset":9240,"ancestor_record_offset":9088,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":9164,"parent_record_offset":9140,"ancestor_record_offset":9088,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y
// 记录形式: compiled_procedure; instances=1
// 源码行: 613
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3157,"record_offset":280,"rva":38680224,"va":5407389344}
// 编译实例 RVAs: 0x24e36a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-31710792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407389344,"raw_hex":"e8b8211cfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5407389475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-31711426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407389344,"raw_hex":"e83e1f1cfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5407390125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38667275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407389344,"raw_hex":"e8f5fbb1fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5407389462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t8990540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407389344,"raw_hex":"e84c2f8900","size":5,"soft_fail":false,"target_va":5416380224,"va":5407389679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9230777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407389344,"raw_hex":"e8b9d98c00","size":5,"soft_fail":false,"target_va":5416620794,"va":5407390012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3157,"record_offset":520,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3157,"record_offset":940,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3157,"record_offset":548,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3157,"record_offset":1644,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3157,"record_offset":1208,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":7338,"source_line":710,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3157,"record_offset":2400,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3157,"record_offset":1180,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$62::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3157,"record_offset":1152,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":18778,"source_line":1356,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":1676,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":1732,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":1588,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":280,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":280,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":2588,"parent_record_offset":2564,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":880,"parent_record_offset":856,"ancestor_record_offset":280,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":808,"parent_record_offset":664,"ancestor_record_offset":280,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":2564,"parent_record_offset":2472,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":604,"parent_record_offset":576,"ancestor_record_offset":280,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":2500,"parent_record_offset":2472,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":2472,"parent_record_offset":2448,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":664,"parent_record_offset":632,"ancestor_record_offset":280,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":632,"parent_record_offset":604,"ancestor_record_offset":280,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":2448,"parent_record_offset":2424,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":696,"parent_record_offset":664,"ancestor_record_offset":280,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3157,"record_offset":1004,"parent_record_offset":968,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3157,"record_offset":968,"parent_record_offset":940,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3157,"record_offset":576,"parent_record_offset":548,"ancestor_record_offset":280,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":3157,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":280,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3157,"record_offset":744,"parent_record_offset":720,"ancestor_record_offset":280,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3157,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":280,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3157,"record_offset":2532,"parent_record_offset":2500,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3157,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3157,"record_offset":1700,"parent_record_offset":1676,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3157,"record_offset":1312,"parent_record_offset":1284,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3157,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3157,"record_offset":1116,"parent_record_offset":968,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":1260,"parent_record_offset":1236,"ancestor_record_offset":280,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":1340,"parent_record_offset":1284,"ancestor_record_offset":280,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":280,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":3157,"record_offset":1236,"parent_record_offset":1208,"ancestor_record_offset":280,"inlinee":117809,"source_line":628,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":3157,"record_offset":1844,"parent_record_offset":1816,"ancestor_record_offset":280,"inlinee":117810,"source_line":638,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":3157,"record_offset":2252,"parent_record_offset":2224,"ancestor_record_offset":280,"inlinee":117811,"source_line":647,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 628
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CC31
// 首条位置: {"module":3157,"inlinee":117809}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":1260,"parent_record_offset":1236,"ancestor_record_offset":1236,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3157,"record_offset":1312,"parent_record_offset":1284,"ancestor_record_offset":1236,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":1340,"parent_record_offset":1284,"ancestor_record_offset":1236,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":1236,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 638
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CC32
// 首条位置: {"module":3157,"inlinee":117810}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":1844,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3157,"record_offset":1920,"parent_record_offset":1892,"ancestor_record_offset":1844,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":1948,"parent_record_offset":1892,"ancestor_record_offset":1844,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":1844,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 647
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CC33
// 首条位置: {"module":3157,"inlinee":117811}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":2252,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3157,"record_offset":2328,"parent_record_offset":2300,"ancestor_record_offset":2252,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":2356,"parent_record_offset":2300,"ancestor_record_offset":2252,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":2252,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y
// 记录形式: compiled_procedure; instances=1
// 源码行: 656
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3157,"record_offset":2696,"rva":38681072,"va":5407390192}
// 编译实例 RVAs: 0x24e39f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-31711640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407390192,"raw_hex":"e8681e1cfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5407390323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-31712274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407390192,"raw_hex":"e8ee1b1cfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5407390973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38668123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407390192,"raw_hex":"e8a5f8b1fd","size":5,"soft_fail":false,"target_va":5368722192,"va":5407390310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t8989692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407390192,"raw_hex":"e8fc2b8900","size":5,"soft_fail":false,"target_va":5416380224,"va":5407390527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9229929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407390192,"raw_hex":"e869d68c00","size":5,"soft_fail":false,"target_va":5416620794,"va":5407390860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3157,"record_offset":2936,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3157,"record_offset":3356,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3157,"record_offset":2964,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":3157,"record_offset":4060,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none_or
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":3157,"record_offset":3624,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":7338,"source_line":710,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3157,"record_offset":4816,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":3157,"record_offset":3596,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$62::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":3157,"record_offset":3568,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":18778,"source_line":1356,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":4092,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":4148,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":4004,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":2696,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":2696,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":5004,"parent_record_offset":4980,"ancestor_record_offset":2696,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":5028,"parent_record_offset":5004,"ancestor_record_offset":2696,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":2696,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":5052,"parent_record_offset":5028,"ancestor_record_offset":2696,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":3224,"parent_record_offset":3080,"ancestor_record_offset":2696,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":3157,"record_offset":4980,"parent_record_offset":4888,"ancestor_record_offset":2696,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":2696,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":3020,"parent_record_offset":2992,"ancestor_record_offset":2696,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":4916,"parent_record_offset":4888,"ancestor_record_offset":2696,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":4888,"parent_record_offset":4864,"ancestor_record_offset":2696,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":2696,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":2696,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":3080,"parent_record_offset":3048,"ancestor_record_offset":2696,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":3048,"parent_record_offset":3020,"ancestor_record_offset":2696,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":4864,"parent_record_offset":4840,"ancestor_record_offset":2696,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":3157,"record_offset":3112,"parent_record_offset":3080,"ancestor_record_offset":2696,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3157,"record_offset":3420,"parent_record_offset":3384,"ancestor_record_offset":2696,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3157,"record_offset":3384,"parent_record_offset":3356,"ancestor_record_offset":2696,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":3157,"record_offset":2992,"parent_record_offset":2964,"ancestor_record_offset":2696,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":3157,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":2696,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3157,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":2696,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3157,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":2696,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":3157,"record_offset":4948,"parent_record_offset":4916,"ancestor_record_offset":2696,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3157,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":2696,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3157,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":2696,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3157,"record_offset":3728,"parent_record_offset":3700,"ancestor_record_offset":2696,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3157,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":2696,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":3157,"record_offset":3532,"parent_record_offset":3384,"ancestor_record_offset":2696,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":3676,"parent_record_offset":3652,"ancestor_record_offset":2696,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":3756,"parent_record_offset":3700,"ancestor_record_offset":2696,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":3700,"parent_record_offset":3676,"ancestor_record_offset":2696,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":3157,"record_offset":3652,"parent_record_offset":3624,"ancestor_record_offset":2696,"inlinee":117813,"source_line":671,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":3157,"record_offset":4260,"parent_record_offset":4232,"ancestor_record_offset":2696,"inlinee":117814,"source_line":681,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":3157,"record_offset":4668,"parent_record_offset":4640,"ancestor_record_offset":2696,"inlinee":117815,"source_line":690,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 671
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CC35
// 首条位置: {"module":3157,"inlinee":117813}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":3676,"parent_record_offset":3652,"ancestor_record_offset":3652,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3157,"record_offset":3728,"parent_record_offset":3700,"ancestor_record_offset":3652,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":3756,"parent_record_offset":3700,"ancestor_record_offset":3652,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":3700,"parent_record_offset":3676,"ancestor_record_offset":3652,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 681
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CC36
// 首条位置: {"module":3157,"inlinee":117814}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":4284,"parent_record_offset":4260,"ancestor_record_offset":4260,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3157,"record_offset":4336,"parent_record_offset":4308,"ancestor_record_offset":4260,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":4364,"parent_record_offset":4308,"ancestor_record_offset":4260,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":4308,"parent_record_offset":4284,"ancestor_record_offset":4260,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 690
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CC37
// 首条位置: {"module":3157,"inlinee":117815}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":4692,"parent_record_offset":4668,"ancestor_record_offset":4668,"inlinee":29015,"source_line":494,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":3157,"record_offset":4744,"parent_record_offset":4716,"ancestor_record_offset":4668,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":4772,"parent_record_offset":4716,"ancestor_record_offset":4668,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":3157,"record_offset":4716,"parent_record_offset":4692,"ancestor_record_offset":4668,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/03157.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::stone_foundation_window_bottom_frame_piece
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 699
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137DA
// 首条位置: {"module":1495,"inlinee":79834}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_window_bottom_bricks
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 715
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137DB
// 首条位置: {"module":1495,"inlinee":79835}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_pos
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":8068,"parent_record_offset":7988,"ancestor_record_offset":7988,"inlinee":79619,"source_line":774,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_unwrapped_wallspace_pos
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1495,"record_offset":8108,"parent_record_offset":8068,"ancestor_record_offset":7988,"inlinee":79621,"source_line":1512,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_wallspace_pos
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1495,"record_offset":8160,"parent_record_offset":8108,"ancestor_record_offset":7988,"inlinee":79620,"source_line":1508,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 763
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137DE
// 首条位置: {"module":1495,"inlinee":79838}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1495,"record_offset":13092,"parent_record_offset":13068,"ancestor_record_offset":13068,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1495,"record_offset":13116,"parent_record_offset":13092,"ancestor_record_offset":13068,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1495,"record_offset":13140,"parent_record_offset":13116,"ancestor_record_offset":13068,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 770
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1370D
// 首条位置: {"module":1482,"inlinee":79629}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1482,"record_offset":17144,"parent_record_offset":17116,"ancestor_record_offset":17116,"inlinee":79619,"source_line":774,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_unwrapped_wallspace_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1482,"record_offset":17204,"parent_record_offset":17144,"ancestor_record_offset":17116,"inlinee":79621,"source_line":1512,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_wallspace_pos
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1482,"record_offset":17176,"parent_record_offset":17144,"ancestor_record_offset":17116,"inlinee":79620,"source_line":1508,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_pos
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 774
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13703
// 首条位置: {"module":1482,"inlinee":79619}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_unwrapped_wallspace_pos
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1482,"record_offset":10336,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":79621,"source_line":1512,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_wallspace_pos
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1482,"record_offset":10308,"parent_record_offset":10276,"ancestor_record_offset":10276,"inlinee":79620,"source_line":1508,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::remap_wallspace_pos
//     owner=private_source_ownership_candidate; count=9
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1482,"record_offset":10380,"parent_record_offset":10336,"ancestor_record_offset":10276,"inlinee":79620,"source_line":1508,"archive":"_pdb/C/modules/01482.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::normalized_brick_line_u
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 782
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1367F
// 首条位置: {"module":1462,"inlinee":79487}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":16580,"parent_record_offset":16480,"ancestor_record_offset":16480,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1462,"record_offset":16520,"parent_record_offset":16480,"ancestor_record_offset":16480,"inlinee":11231,"source_line":289,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::rem_euclid
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":16544,"parent_record_offset":16520,"ancestor_record_offset":16480,"inlinee":11232,"source_line":1982,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line<system_wall_constructor::construct_walls::construct_walls::closure_env$3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 795
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1495,"record_offset":280,"rva":21364800,"va":5390073920}
// 编译实例 RVAs: 0x1460040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t-14396898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390073920,"raw_hex":"e81e5224ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5390075597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=1
//     target_aliases=17 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20533176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390073920,"raw_hex":"e848b0c6fe","size":5,"soft_fail":false,"target_va":5369542000,"va":5390075171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14140fe70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-329791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390073920,"raw_hex":"e8c1f7faff","size":5,"soft_fail":false,"target_va":5389745776,"va":5390075562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14148ff80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t195557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390073920,"raw_hex":"e8e5fb0200","size":5,"soft_fail":false,"target_va":5390270336,"va":5390074774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26277184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390073920,"raw_hex":"e840f59001","size":5,"soft_fail":false,"target_va":5416352064,"va":5390074875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26983170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390073920,"raw_hex":"e802bb9b01","size":5,"soft_fail":false,"target_va":5417057664,"va":5390074489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26982826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390073920,"raw_hex":"e8aab99b01","size":5,"soft_fail":false,"target_va":5417057840,"va":5390075009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26291125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390073920,"raw_hex":"e8b52b9101","size":5,"soft_fail":false,"target_va":5416366080,"va":5390074950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26546395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390073920,"raw_hex":"e8db109501","size":5,"soft_fail":false,"target_va":5416622240,"va":5390075840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6692180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390073920,"raw_hex":"e8ace299ff","size":5,"soft_fail":false,"target_va":5383383152,"va":5390075327,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17315971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390073920,"raw_hex":"e883380801","size":5,"soft_fail":false,"target_va":5407390192,"va":5390074216,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1495,"record_offset":644,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1495,"record_offset":1332,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1495,"record_offset":1120,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1495,"record_offset":728,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1495,"record_offset":1548,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11646,"source_line":1259,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":3116,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1495,"record_offset":944,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1495,"record_offset":1148,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1495,"record_offset":1240,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1495,"record_offset":888,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1495,"record_offset":1064,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::arch_walker::ArchWalker::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/arch_walker.rs
//     evidence={"module":1495,"record_offset":3088,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":26102,"source_line":22,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1495,"record_offset":616,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79485,"source_line":1525,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::construct_wall_boards::WallSampler::pos_xz_at
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1495,"record_offset":1676,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79781,"source_line":1715,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::deterministic_frame_brick_size_jitter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":2592,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79488,"source_line":980,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::normalized_brick_line_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":2316,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":79487,"source_line":782,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1495,"record_offset":2444,"parent_record_offset":280,"ancestor_record_offset":280,"inlinee":9794,"source_line":89,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":3304,"parent_record_offset":3280,"ancestor_record_offset":280,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":3328,"parent_record_offset":3304,"ancestor_record_offset":280,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":3352,"parent_record_offset":3328,"ancestor_record_offset":280,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1495,"record_offset":3280,"parent_record_offset":3188,"ancestor_record_offset":280,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":1424,"parent_record_offset":1400,"ancestor_record_offset":280,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":3216,"parent_record_offset":3188,"ancestor_record_offset":280,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":3188,"parent_record_offset":3164,"ancestor_record_offset":280,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":280,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":1448,"parent_record_offset":1424,"ancestor_record_offset":280,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1495,"record_offset":3164,"parent_record_offset":3140,"ancestor_record_offset":280,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1495,"record_offset":1400,"parent_record_offset":1360,"ancestor_record_offset":280,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1495,"record_offset":1360,"parent_record_offset":1332,"ancestor_record_offset":280,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1495,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":280,"inlinee":6833,"source_line":2145,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1495,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":280,"inlinee":8914,"source_line":1918,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1495,"record_offset":1640,"parent_record_offset":1576,"ancestor_record_offset":280,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1495,"record_offset":2408,"parent_record_offset":2316,"ancestor_record_offset":280,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1495,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":280,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::rem_euclid
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1495,"record_offset":2372,"parent_record_offset":2348,"ancestor_record_offset":280,"inlinee":11232,"source_line":1982,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1495,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":280,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1495,"record_offset":1576,"parent_record_offset":1548,"ancestor_record_offset":280,"inlinee":11648,"source_line":1164,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1495,"record_offset":3248,"parent_record_offset":3216,"ancestor_record_offset":280,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1495,"record_offset":2704,"parent_record_offset":2672,"ancestor_record_offset":280,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1495,"record_offset":2732,"parent_record_offset":2672,"ancestor_record_offset":280,"inlinee":6600,"source_line":2532,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1495,"record_offset":1272,"parent_record_offset":1240,"ancestor_record_offset":280,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1495,"record_offset":1180,"parent_record_offset":1148,"ancestor_record_offset":280,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1495,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":280,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1495,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":280,"inlinee":6826,"source_line":868,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1495,"record_offset":2476,"parent_record_offset":2444,"ancestor_record_offset":280,"inlinee":9795,"source_line":522,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":1495,"record_offset":1608,"parent_record_offset":1576,"ancestor_record_offset":280,"inlinee":10957,"source_line":559,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":280,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1495,"record_offset":1512,"parent_record_offset":1360,"ancestor_record_offset":280,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1495,"record_offset":844,"parent_record_offset":816,"ancestor_record_offset":280,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1495,"record_offset":792,"parent_record_offset":760,"ancestor_record_offset":280,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1495,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":280,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$62::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1495,"record_offset":760,"parent_record_offset":728,"ancestor_record_offset":280,"inlinee":18778,"source_line":1356,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1495,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":280,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1495,"record_offset":1712,"parent_record_offset":1676,"ancestor_record_offset":280,"inlinee":10989,"source_line":796,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1495,"record_offset":1740,"parent_record_offset":1712,"ancestor_record_offset":280,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":1495,"record_offset":1824,"parent_record_offset":1712,"ancestor_record_offset":280,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::rem_euclid
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1495,"record_offset":2348,"parent_record_offset":2316,"ancestor_record_offset":280,"inlinee":11231,"source_line":289,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1495,"record_offset":1000,"parent_record_offset":968,"ancestor_record_offset":280,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::deterministic_frame_brick_hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":2620,"parent_record_offset":2592,"ancestor_record_offset":280,"inlinee":79489,"source_line":997,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::mix_frame_brick_hash
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1495,"record_offset":2672,"parent_record_offset":2620,"ancestor_record_offset":280,"inlinee":79490,"source_line":1013,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 874
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1462,"record_offset":16120,"rva":21043184,"va":5389752304}
// 编译实例 RVAs: 0x14117f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400cb570
//     owner=unresolved_namespace; count=3
//     target_aliases=17 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20211723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389752304,"raw_hex":"e8f597cbfe","size":5,"soft_fail":false,"target_va":5369542000,"va":5389753718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408fb460
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11627482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389752304,"raw_hex":"e826944eff","size":5,"soft_fail":false,"target_va":5378126944,"va":5389754421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140956fd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11251856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389752304,"raw_hex":"e8704f54ff","size":5,"soft_fail":false,"target_va":5378502608,"va":5389754459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de63a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6468840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389752304,"raw_hex":"e8184b9dff","size":5,"soft_fail":false,"target_va":5383283616,"va":5389752451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: stone_foundation_window_frames.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27305295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389752304,"raw_hex":"e84fa5a001","size":5,"soft_fail":false,"target_va":5417057840,"va":5389752540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26626770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389752304,"raw_hex":"e8d24a9601","size":5,"soft_fail":false,"target_va":5416381568,"va":5389754793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::geometry::instanced_wall::BrickTransform::encode
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11273101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389752304,"raw_hex":"e873fc53ff","size":5,"soft_fail":false,"target_va":5378481680,"va":5389754776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"fastrand","version":"1.8.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"fastrand","version":"2.3.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6370701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389752304,"raw_hex":"e873ca9eff","size":5,"soft_fail":false,"target_va":5383383152,"va":5389753848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::construct_wooden_walls::construct_wall_boards::WallSampler::tangent_between
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t518357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5389752304,"raw_hex":"e8d5e80700","size":5,"soft_fail":false,"target_va":5390270992,"va":5389752630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1462,"record_offset":20704,"parent_record_offset":16120,"ancestor_record_offset":16120,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::mat3::impl$16::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":1462,"record_offset":19116,"parent_record_offset":16120,"ancestor_record_offset":16120,"inlinee":9822,"source_line":843,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":16616,"parent_record_offset":16120,"ancestor_record_offset":16120,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":17168,"parent_record_offset":16120,"ancestor_record_offset":16120,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":16868,"parent_record_offset":16120,"ancestor_record_offset":16120,"inlinee":26155,"source_line":562,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":16652,"parent_record_offset":16120,"ancestor_record_offset":16120,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":16924,"parent_record_offset":16120,"ancestor_record_offset":16120,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":17020,"parent_record_offset":16120,"ancestor_record_offset":16120,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_walls::construct_walls::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_walls.rs
//     evidence={"module":1462,"record_offset":20760,"parent_record_offset":16120,"ancestor_record_offset":16120,"inlinee":79492,"source_line":158,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::construct_wall_boards::BoardWallSpace::length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/construct_wall_boards.rs
//     evidence={"module":1462,"record_offset":16452,"parent_record_offset":16120,"ancestor_record_offset":16120,"inlinee":79485,"source_line":1525,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::deterministic_frame_brick_seed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1462,"record_offset":20184,"parent_record_offset":16120,"ancestor_record_offset":16120,"inlinee":79491,"source_line":969,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::deterministic_frame_brick_size_jitter
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1462,"record_offset":18036,"parent_record_offset":16120,"ancestor_record_offset":16120,"inlinee":79488,"source_line":980,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::normalized_brick_line_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1462,"record_offset":16480,"parent_record_offset":16120,"ancestor_record_offset":16120,"inlinee":79487,"source_line":782,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1462,"record_offset":20876,"parent_record_offset":20844,"ancestor_record_offset":16120,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1462,"record_offset":20924,"parent_record_offset":20900,"ancestor_record_offset":16120,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1462,"record_offset":20900,"parent_record_offset":20876,"ancestor_record_offset":16120,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1462,"record_offset":20844,"parent_record_offset":20808,"ancestor_record_offset":16120,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1462,"record_offset":20784,"parent_record_offset":20760,"ancestor_record_offset":16120,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1462,"record_offset":20808,"parent_record_offset":20784,"ancestor_record_offset":16120,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":16580,"parent_record_offset":16480,"ancestor_record_offset":16120,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":17364,"parent_record_offset":17340,"ancestor_record_offset":16120,"inlinee":9829,"source_line":907,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::rem_euclid
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":16544,"parent_record_offset":16520,"ancestor_record_offset":16120,"inlinee":11232,"source_line":1982,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1462,"record_offset":16816,"parent_record_offset":16792,"ancestor_record_offset":16120,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1462,"record_offset":18168,"parent_record_offset":18128,"ancestor_record_offset":16120,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1462,"record_offset":18196,"parent_record_offset":18128,"ancestor_record_offset":16120,"inlinee":6600,"source_line":2532,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1462,"record_offset":17052,"parent_record_offset":17020,"ancestor_record_offset":16120,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":1462,"record_offset":16956,"parent_record_offset":16924,"ancestor_record_offset":16120,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1462,"record_offset":20964,"parent_record_offset":20808,"ancestor_record_offset":16120,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::Mat3::mul_vec3
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":1462,"record_offset":19196,"parent_record_offset":19168,"ancestor_record_offset":16120,"inlinee":9826,"source_line":625,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::mat3::impl$22::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/mat3.rs
//     evidence={"module":1462,"record_offset":19168,"parent_record_offset":19116,"ancestor_record_offset":16120,"inlinee":9824,"source_line":893,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":16760,"parent_record_offset":16728,"ancestor_record_offset":16120,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":16728,"parent_record_offset":16704,"ancestor_record_offset":16120,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":16704,"parent_record_offset":16676,"ancestor_record_offset":16120,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":16676,"parent_record_offset":16652,"ancestor_record_offset":16120,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":19236,"parent_record_offset":19196,"ancestor_record_offset":16120,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1462,"record_offset":19564,"parent_record_offset":19520,"ancestor_record_offset":16120,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::rem_euclid
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1462,"record_offset":16520,"parent_record_offset":16480,"ancestor_record_offset":16120,"inlinee":11231,"source_line":289,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1462,"record_offset":16792,"parent_record_offset":16760,"ancestor_record_offset":16120,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::deterministic_frame_brick_hash
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1462,"record_offset":18068,"parent_record_offset":18036,"ancestor_record_offset":16120,"inlinee":79489,"source_line":997,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::mix_frame_brick_hash
//     owner=private_source_ownership_candidate; count=15
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1462,"record_offset":18128,"parent_record_offset":18068,"ancestor_record_offset":16120,"inlinee":79490,"source_line":1013,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::quantized_frame_position
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 962
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x137D6
// 首条位置: {"module":1495,"inlinee":79830}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":1495,"record_offset":6616,"parent_record_offset":6588,"ancestor_record_offset":6588,"inlinee":9785,"source_line":49,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::floor
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":1495,"record_offset":6640,"parent_record_offset":6616,"ancestor_record_offset":6588,"inlinee":9786,"source_line":1721,"archive":"_pdb/C/modules/01495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::deterministic_frame_brick_seed
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 969
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13683
// 首条位置: {"module":1462,"inlinee":79491}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::deterministic_frame_brick_hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1462,"record_offset":20216,"parent_record_offset":20184,"ancestor_record_offset":20184,"inlinee":79489,"source_line":997,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1462,"record_offset":20312,"parent_record_offset":20276,"ancestor_record_offset":20184,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1462,"record_offset":20340,"parent_record_offset":20276,"ancestor_record_offset":20184,"inlinee":6600,"source_line":2532,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::mix_frame_brick_hash
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1462,"record_offset":20276,"parent_record_offset":20216,"ancestor_record_offset":20184,"inlinee":79490,"source_line":1013,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::deterministic_frame_brick_size_jitter
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 980
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13680
// 首条位置: {"module":1462,"inlinee":79488}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::deterministic_frame_brick_hash
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1462,"record_offset":18068,"parent_record_offset":18036,"ancestor_record_offset":18036,"inlinee":79489,"source_line":997,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1462,"record_offset":18168,"parent_record_offset":18128,"ancestor_record_offset":18036,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1462,"record_offset":18196,"parent_record_offset":18128,"ancestor_record_offset":18036,"inlinee":6600,"source_line":2532,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::mix_frame_brick_hash
//     owner=private_source_ownership_candidate; count=15
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1462,"record_offset":18128,"parent_record_offset":18068,"ancestor_record_offset":18036,"inlinee":79490,"source_line":1013,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::deterministic_frame_brick_hash
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 997
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13681
// 首条位置: {"module":1462,"inlinee":79489}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::mix_frame_brick_hash
//     owner=private_source_ownership_candidate; count=20
//     packages=[{"name":"system-wall-constructor","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_wooden_walls/stone_foundation_window_frames.rs
//     evidence={"module":1462,"record_offset":18128,"parent_record_offset":18068,"ancestor_record_offset":18068,"inlinee":79490,"source_line":1013,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1462,"record_offset":18168,"parent_record_offset":18128,"ancestor_record_offset":18068,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1462,"record_offset":18196,"parent_record_offset":18128,"ancestor_record_offset":18068,"inlinee":6600,"source_line":2532,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::mix_frame_brick_hash
// 记录形式: inline_instance, inlinee_declaration; instances=23
// 源码行: 1013
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13682
// 首条位置: {"module":1462,"inlinee":79490}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1462,"record_offset":18168,"parent_record_offset":18128,"ancestor_record_offset":18128,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/01462.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$9::wrapping_mul
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1462,"record_offset":18196,"parent_record_offset":18128,"ancestor_record_offset":18128,"inlinee":6600,"source_line":2532,"archive":"_pdb/C/modules/01462.jsonl.gz"}
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
// 完整结构及原始类型记录: stone_foundation_window_frames.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: stone_foundation_window_frames.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: stone_foundation_window_frames.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y
// 信息 ID: c5e5d1ccfd4b39334ef20a80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x14616d3 -> 0x24e36a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1461a15 -> 0x24e36a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1461c30 -> 0x24e36a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1461e68 -> 0x24e36a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x14612cb -> 0x24e36a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y::closure$0
// 信息 ID: 05c30917067d04b2b3309dff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y; module=3157, inline_record=1236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y::closure$1
// 信息 ID: 5f02fe2276ded0735d73844a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y; module=3157, inline_record=1844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y::closure$2
// 信息 ID: d86570b74d027eed9a87a60b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_max_y; module=3157, inline_record=2252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y
// 信息 ID: d344b055456e5ec40b161168
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1460168 -> 0x24e39f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y::closure$0
// 信息 ID: 53e463fd21302671fdef277a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y; module=3157, inline_record=3652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y::closure$1
// 信息 ID: 5de44d0c05c5f1dad4a557b7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y; module=3157, inline_record=4260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y::closure$2
// 信息 ID: 05223a37f9700ce667233a77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::clip_wallspace_polyline_to_min_y; module=3157, inline_record=4668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::deterministic_frame_brick_hash
// 信息 ID: cb3350a195882328c194cc57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1462, inline_record=18068; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1462, inline_record=18608; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1462, inline_record=20216; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=2620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::deterministic_frame_brick_seed
// 信息 ID: 950fd756fe26568925d11cd8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1462, inline_record=20184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::deterministic_frame_brick_size_jitter
// 信息 ID: e0362e5f853784a259eac411
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1462, inline_record=18036; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1462, inline_record=18580; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=2592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_cottage_door_frame_bricks
// 信息 ID: ae3ea48660a0a932d121ac6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=12652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_cottage_window_outline_bricks
// 信息 ID: 0e7cdf58466cfeb0b030ba05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=14428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks
// 信息 ID: e5aaf8b7a7201519c7a587fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=8488; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure$0
// 信息 ID: 73503a2867c27b9ca315ee86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::rev::Rev<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$0 <- core::iter::adapters::map::impl$3::rfold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::rev::Rev<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$0 <- core::iter::adapters::map::impl$3::rfold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::rev::Rev<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$3::rfold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::rev::Rev<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > > >; module=1482, inline_record=37048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure$1
// 信息 ID: 2ec4cf9506d0cd555283ed21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::rev::Rev<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$1 <- core::iter::adapters::map::impl$3::rfold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::rev::Rev<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$1 <- core::iter::adapters::map::impl$3::rfold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::rev::Rev<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$3::rfold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::rev::Rev<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > > >; module=1482, inline_record=38800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks
// 信息 ID: f8f55581b80b2bb2def02cb1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=16172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure$0
// 信息 ID: 9fd6e4ff2d542589dd46d4f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > >; module=1482, inline_record=26296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure$1
// 信息 ID: d85140d4a0b695ba263f67f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$1 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$1 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > >; module=1482, inline_record=10248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>
// 信息 ID: 948b25d979fd6873af05da8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0 -> ["system_wall_constructor::construct_walls::construct_walls::closure_env$3"]; 字段布局 = unknown。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure_env$0 <- system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > @ 0x1410398 -> 0x14117f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::arch_walker::ArchWalker::walk<system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > @ 0x14104f0 -> 0x14117f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line<system_wall_constructor::construct_walls::construct_walls::closure_env$3>
// 信息 ID: 96ba70a9944ff5f61543ce97
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line -> ["system_wall_constructor::construct_walls::construct_walls::closure_env$3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x146150b -> 0x1460040 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x146176f -> 0x1460040 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1461aa1 -> 0x1460040 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x14619bb -> 0x1460040 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1461cc7 -> 0x1460040 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1461ef3 -> 0x1460040 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1461367 -> 0x1460040 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_clipped_brick_line
// 信息 ID: 66e3a29ad08fec0f29effd67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=9088; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=10252; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=13180; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=14900; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=16872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>
// 信息 ID: 765102af2ffebb4698a66566
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames -> ["system_wall_constructor::construct_walls::construct_walls::closure_env$3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x805c51 -> 0x1460930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_window_bottom_bricks
// 信息 ID: 2b47a3d0699665b87f87d2d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=7988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::mix_frame_brick_hash
// 信息 ID: 7e89da5e4e20b13ada4372d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1462, inline_record=18128; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1462, inline_record=18228; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1462, inline_record=18324; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1462, inline_record=18416; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1462, inline_record=18512; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1462, inline_record=18664; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1462, inline_record=18764; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1462, inline_record=18860; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::normalized_brick_line_u
// 信息 ID: 7bf5d735edb57ce2be968a0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line::closure$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1462, inline_record=16480; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_brick_line<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=2316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::quantized_frame_position
// 信息 ID: c70b583f2d2716c85e53b30f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=6588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points
// 信息 ID: f2cef21a3ab2333385c5cf99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=13068; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=14788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points::closure$0
// 信息 ID: f85a969c54c730d04fd12625
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points::closure_env$0 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points::closure_env$0> > > >; module=1482, inline_record=17116; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_pos
// 信息 ID: 5e068a5d0035142bc6ba10a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > >; module=1482, inline_record=10276; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::remap_frame_line_points::closure_env$0> > > >; module=1482, inline_record=17144; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_window_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > >; module=1482, inline_record=26324; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$3::rfold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::rev::Rev<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$0<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > > >; module=1482, inline_record=37076; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$3::rfold<glam::f32::vec2::Vec2,core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::rev::Rev<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_gothic_door_frame_bricks::closure_env$1<system_wall_constructor::construct_walls::construct_walls::closure_env$3> > > > > >; module=1482, inline_record=38828; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=8068; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=8224; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=8724; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::stone_foundation_frame_region
// 信息 ID: c1698031c5ec7445051fe720
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=6872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::stone_foundation_window_bottom_frame_piece
// 信息 ID: 3c905b42c267bee9358739f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_wooden_walls::stone_foundation_window_frames::emit_stone_foundation_decorator_frames<system_wall_constructor::construct_walls::construct_walls::closure_env$3>; module=1495, inline_record=7920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
