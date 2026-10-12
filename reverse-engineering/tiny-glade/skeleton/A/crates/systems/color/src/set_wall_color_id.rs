// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/color/src/set_wall_color_id.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 05d44566281349b224dd70dbfff534895080685ef34fac6b20416f42d9419ae6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: set_wall_color_id.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3468
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
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::set_wall_color_id::init_color_ids_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3468,"record_offset":1372,"rva":27963856,"va":5396672976}
// 编译实例 RVAs: 0x1aab1d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14065ad20
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21300439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396672976,"raw_hex":"e829fbbafe","size":5,"soft_fail":false,"target_va":5375372576,"va":5396673010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140669fe0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21238335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396672976,"raw_hex":"e8c1edbbfe","size":5,"soft_fail":false,"target_va":5375434720,"va":5396673050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dc7820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3261982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396672976,"raw_hex":"e81ec63100","size":5,"soft_fail":false,"target_va":5399935008,"va":5396673021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dc7a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3262587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396672976,"raw_hex":"e87bc83100","size":5,"soft_fail":false,"target_va":5399935632,"va":5396673040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dc7d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3262809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396672976,"raw_hex":"e859c93100","size":5,"soft_fail":false,"target_va":5399936256,"va":5396673442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dc7f70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3263632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396672976,"raw_hex":"e890cc3100","size":5,"soft_fail":false,"target_va":5399936880,"va":5396673243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209b690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6227117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396672976,"raw_hex":"e8ad045f00","size":5,"soft_fail":false,"target_va":5402900112,"va":5396672990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6920235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396672976,"raw_hex":"e82b986900","size":5,"soft_fail":false,"target_va":5403593360,"va":5396673120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t6245489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396672976,"raw_hex":"e8714c5f00","size":5,"soft_fail":false,"target_va":5402919072,"va":5396673578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-19426624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396672976,"raw_hex":"e8c092d7fe","size":5,"soft_fail":false,"target_va":5377246528,"va":5396673147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::insert_resource
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3468,"record_offset":1512,"parent_record_offset":1372,"ancestor_record_offset":1372,"inlinee":12145,"source_line":598,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3468,"record_offset":1752,"parent_record_offset":1724,"ancestor_record_offset":1372,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3468,"record_offset":1724,"parent_record_offset":1688,"ancestor_record_offset":1372,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3468,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":1372,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3468,"record_offset":1688,"parent_record_offset":1656,"ancestor_record_offset":1372,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3468,"record_offset":1828,"parent_record_offset":1608,"ancestor_record_offset":1372,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3468,"record_offset":1800,"parent_record_offset":1608,"ancestor_record_offset":1372,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3468,"record_offset":1656,"parent_record_offset":1608,"ancestor_record_offset":1372,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3468,"record_offset":1968,"parent_record_offset":1608,"ancestor_record_offset":1372,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3468,"record_offset":1552,"parent_record_offset":1512,"ancestor_record_offset":1372,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3468,"record_offset":2000,"parent_record_offset":1552,"ancestor_record_offset":1372,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":3468,"record_offset":1608,"parent_record_offset":1552,"ancestor_record_offset":1372,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3468,"record_offset":1932,"parent_record_offset":1908,"ancestor_record_offset":1372,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_unaligned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3468,"record_offset":1884,"parent_record_offset":1608,"ancestor_record_offset":1372,"inlinee":12143,"source_line":1546,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write_unaligned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3468,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":1372,"inlinee":12144,"source_line":1579,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::set_wall_color_id::set_wall_color_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3468,"record_offset":3444,"rva":27964560,"va":5396673680}
// 编译实例 RVAs: 0x1aab490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26860842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e8d62266fe","size":5,"soft_fail":false,"target_va":5369813552,"va":5396674389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bd730
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24044648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e8981b91fe","size":5,"soft_fail":false,"target_va":5372630832,"va":5396675475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14051aa00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22613453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e833f2a6fe","size":5,"soft_fail":false,"target_va":5374061056,"va":5396674504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14065ad20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21302425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e867f3bafe","size":5,"soft_fail":false,"target_va":5375372576,"va":5396674996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140739e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20388817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e82fe4c8fe","size":5,"soft_fail":false,"target_va":5376286288,"va":5396675100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19730692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e8fceed2fe","size":5,"soft_fail":false,"target_va":5376944000,"va":5396674687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141dc81e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3263697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e8d1cc3100","size":5,"soft_fail":false,"target_va":5399937504,"va":5396673802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: set_wall_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6919473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e831956900","size":5,"soft_fail":false,"target_va":5403593344,"va":5396673866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-25369509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e85be47cfe","size":5,"soft_fail":false,"target_va":5371305232,"va":5396674736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6303632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e8902f6000","size":5,"soft_fail":false,"target_va":5402978880,"va":5396675243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6739340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e88cd56600","size":5,"soft_fail":false,"target_va":5403414576,"va":5396675231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6303964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e8dc306000","size":5,"soft_fail":false,"target_va":5402979152,"va":5396675183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6304713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e8c9336000","size":5,"soft_fail":false,"target_va":5402980128,"va":5396675410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-14741801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e8d70e1fff","size":5,"soft_fail":false,"target_va":5381932864,"va":5396674660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21239490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e83ee9bbfe","size":5,"soft_fail":false,"target_va":5375434976,"va":5396674461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12254449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396673680,"raw_hex":"e80f0345ff","size":5,"soft_fail":false,"target_va":5384420192,"va":5396674636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5396673680,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5396673886,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3468,"record_offset":5260,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::is_empty
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3468,"record_offset":3620,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":29646,"source_line":544,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3468,"record_offset":4092,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3468,"record_offset":4624,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3468,"record_offset":4888,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3468,"record_offset":4056,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3468,"record_offset":6460,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3468,"record_offset":5916,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3468,"record_offset":3816,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3468,"record_offset":6352,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":7514,"source_line":155,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::bounding_box_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3468,"record_offset":5316,"parent_record_offset":3444,"ancestor_record_offset":3444,"inlinee":27397,"source_line":875,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3468,"record_offset":4760,"parent_record_offset":4728,"ancestor_record_offset":3444,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3468,"record_offset":4728,"parent_record_offset":4688,"ancestor_record_offset":3444,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3468,"record_offset":3740,"parent_record_offset":3712,"ancestor_record_offset":3444,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3468,"record_offset":4688,"parent_record_offset":4652,"ancestor_record_offset":3444,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3468,"record_offset":4852,"parent_record_offset":4828,"ancestor_record_offset":3444,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3468,"record_offset":4828,"parent_record_offset":4624,"ancestor_record_offset":3444,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3468,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3444,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::len
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3468,"record_offset":3712,"parent_record_offset":3684,"ancestor_record_offset":3444,"inlinee":29648,"source_line":331,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3468,"record_offset":4652,"parent_record_offset":4624,"ancestor_record_offset":3444,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::len
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3468,"record_offset":3684,"parent_record_offset":3644,"ancestor_record_offset":3444,"inlinee":29647,"source_line":733,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3468,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":3444,"inlinee":6368,"source_line":717,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3468,"record_offset":4140,"parent_record_offset":4116,"ancestor_record_offset":3444,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3468,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":3444,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3468,"record_offset":5220,"parent_record_offset":5196,"ancestor_record_offset":3444,"inlinee":10427,"source_line":846,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$13::deref
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3468,"record_offset":3644,"parent_record_offset":3620,"ancestor_record_offset":3444,"inlinee":23969,"source_line":798,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3468,"record_offset":3884,"parent_record_offset":3844,"ancestor_record_offset":3444,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3468,"record_offset":5556,"parent_record_offset":5528,"ancestor_record_offset":3444,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3468,"record_offset":5616,"parent_record_offset":5592,"ancestor_record_offset":3444,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3468,"record_offset":4968,"parent_record_offset":4944,"ancestor_record_offset":3444,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3468,"record_offset":4944,"parent_record_offset":4912,"ancestor_record_offset":3444,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3468,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":3444,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":3468,"record_offset":5412,"parent_record_offset":5384,"ancestor_record_offset":3444,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3468,"record_offset":5384,"parent_record_offset":5340,"ancestor_record_offset":3444,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3468,"record_offset":3772,"parent_record_offset":3684,"ancestor_record_offset":3444,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3468,"record_offset":4992,"parent_record_offset":4968,"ancestor_record_offset":3444,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3468,"record_offset":5196,"parent_record_offset":4912,"ancestor_record_offset":3444,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3468,"record_offset":5024,"parent_record_offset":4944,"ancestor_record_offset":3444,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3468,"record_offset":4516,"parent_record_offset":4492,"ancestor_record_offset":3444,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3468,"record_offset":5148,"parent_record_offset":5124,"ancestor_record_offset":3444,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3468,"record_offset":5484,"parent_record_offset":5440,"ancestor_record_offset":3444,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3468,"record_offset":4792,"parent_record_offset":4688,"ancestor_record_offset":3444,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3468,"record_offset":6416,"parent_record_offset":6380,"ancestor_record_offset":3444,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3468,"record_offset":3968,"parent_record_offset":3940,"ancestor_record_offset":3444,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3468,"record_offset":5944,"parent_record_offset":5916,"ancestor_record_offset":3444,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3468,"record_offset":3996,"parent_record_offset":3968,"ancestor_record_offset":3444,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3468,"record_offset":4276,"parent_record_offset":4164,"ancestor_record_offset":3444,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3468,"record_offset":4468,"parent_record_offset":4164,"ancestor_record_offset":3444,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3468,"record_offset":4340,"parent_record_offset":4316,"ancestor_record_offset":3444,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3468,"record_offset":4316,"parent_record_offset":4276,"ancestor_record_offset":3444,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3468,"record_offset":4492,"parent_record_offset":4468,"ancestor_record_offset":3444,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3468,"record_offset":5124,"parent_record_offset":5100,"ancestor_record_offset":3444,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3468,"record_offset":5972,"parent_record_offset":5944,"ancestor_record_offset":3444,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3468,"record_offset":5100,"parent_record_offset":5048,"ancestor_record_offset":3444,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3468,"record_offset":3912,"parent_record_offset":3884,"ancestor_record_offset":3444,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3468,"record_offset":3844,"parent_record_offset":3816,"ancestor_record_offset":3444,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3468,"record_offset":6380,"parent_record_offset":6352,"ancestor_record_offset":3444,"inlinee":7500,"source_line":207,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3468,"record_offset":5528,"parent_record_offset":5340,"ancestor_record_offset":3444,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3468,"record_offset":5592,"parent_record_offset":5340,"ancestor_record_offset":3444,"inlinee":23218,"source_line":173,"archive":"_pdb/A/modules/03468.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::aabb::Aabb2::from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":3468,"record_offset":5340,"parent_record_offset":5316,"ancestor_record_offset":3444,"inlinee":23220,"source_line":91,"archive":"_pdb/A/modules/03468.jsonl.gz"}
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
// 完整结构及原始类型记录: set_wall_color_id.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: set_wall_color_id.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: set_wall_color_id.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::CreatePlasterCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::DeletePlasterCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::event::EventReader<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::CreatePlasterCmd>", "bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::DeletePlasterCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::CreatePlasterCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::DeletePlasterCmd>

// ------------------------------------------------------------------
// 函数: system_color::set_wall_color_id::init_color_ids_buffer
// 信息 ID: 65d33b3450db4d546bb7eee5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 308e88593d87089a280329f2: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)> > @ 0x149e581 -> 0x1aab1d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::system::commands::Commands)> @ 0x14a0efc -> 0x1aab1d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::set_wall_color_id::set_wall_color_id
// 信息 ID: 9d2a1d94c90714d30169bd4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 080c54359eb920c6258a50ff: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::CreatePlasterCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::DeletePlasterCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::CreatePlasterCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::DeletePlasterCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::CreatePlasterCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::DeletePlasterCmd>)> > @ 0x149b173 -> 0x1aab490 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::CreatePlasterCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::DeletePlasterCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::CreatePlasterCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::DeletePlasterCmd>)> @ 0x14a0a8f -> 0x1aab490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
