// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/systems/wall/eraser/delete_focused.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 453d604c79ee4222baebe92f74a24c0be413e20560fd3a94e376f7f7f0f37e98
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: delete_focused.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 525, 560, 577, 1643, 1682, 1822
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
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::delete_focused::delete_action_used
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":525,"record_offset":150740,"rva":10406672,"va":5379115792}
// 编译实例 RVAs: 0x9ecb10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::input::action_set::DigitalActionState::just_pressed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/input/action_set.rs
//     evidence={"module":525,"record_offset":150896,"parent_record_offset":150740,"ancestor_record_offset":150740,"inlinee":29411,"source_line":260,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::delete_focused::delete_focused
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":525,"record_offset":141456,"rva":10402192,"va":5379111312}
// 编译实例 RVAs: 0x9eb990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3433621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e86b9bcbff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379112304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-3433255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8d99ccbff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379111954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10390099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8ad7561ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379112286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cb60
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5632244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e80c0faaff","size":5,"soft_fail":false,"target_va":5373479776,"va":5379112015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-500655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8515cf8ff","size":5,"soft_fail":false,"target_va":5378614272,"va":5379114922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140979360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-472856","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8e8c8f8ff","size":5,"soft_fail":false,"target_va":5378642784,"va":5379115635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14097ae00
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-464733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8a3e8f8ff","size":5,"soft_fail":false,"target_va":5378649600,"va":5379114328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a62c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-286230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8eaa1fbff","size":5,"soft_fail":false,"target_va":5378826944,"va":5379113169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409a7df0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-281160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8b8b5fbff","size":5,"soft_fail":false,"target_va":5378833904,"va":5379115059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd7690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2012431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e80fb51e00","size":5,"soft_fail":false,"target_va":5381125776,"va":5379113340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c916c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2772210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8f24c2a00","size":5,"soft_fail":false,"target_va":5381887680,"va":5379115465,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f5840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12622940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e85c9cc000","size":5,"soft_fail":false,"target_va":5391734848,"va":5379111903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141618a50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12767196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8dccfc200","size":5,"soft_fail":false,"target_va":5391878736,"va":5379111535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419af9b0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16530519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8573cfc00","size":5,"soft_fail":false,"target_va":5395642800,"va":5379112276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f2b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37236920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8b8303802","size":5,"soft_fail":false,"target_va":5416350384,"va":5379113459,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37236612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8842f3802","size":5,"soft_fail":false,"target_va":5416350496,"va":5379113879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37237182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8be313802","size":5,"soft_fail":false,"target_va":5416350720,"va":5379113533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f4e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37238540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e80c373802","size":5,"soft_fail":false,"target_va":5416350944,"va":5379112399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: delete_focused.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37944512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8c0fc4202","size":5,"soft_fail":false,"target_va":5417057056,"va":5379112539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37288221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e81df93802","size":5,"soft_fail":false,"target_va":5416400928,"va":5379112702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37509054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8be573c02","size":5,"soft_fail":false,"target_va":5416620794,"va":5379111735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc::new<enum2$<country_core::resources::history::terrain_edit::TerrainEdit> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e80d0a0000","size":5,"soft_fail":false,"target_va":5379116576,"va":5379113998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t1432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e898050000","size":5,"soft_fail":false,"target_va":5379116176,"va":5379114739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1438810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e85af41500","size":5,"soft_fail":false,"target_va":5380552224,"va":5379113409,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::start_input_recording
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1438560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e860f31500","size":5,"soft_fail":false,"target_va":5380550144,"va":5379111579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::with_concurrent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1717294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e82e341a00","size":5,"soft_fail":false,"target_va":5380829968,"va":5379112669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::get_all_connected_nodes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t437680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8b0ad0600","size":5,"soft_fail":false,"target_va":5379549488,"va":5379111803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::stairs::stairs_state::StairsState::remove_node
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t431789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8ad960600","size":5,"soft_fail":false,"target_va":5379545616,"va":5379113822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation::start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14895717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8654ae300","size":5,"soft_fail":false,"target_va":5394009936,"va":5379114214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::bulldoze_params::BulldozeParams::play_bulldoze_sound
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1514826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e84a1d1700","size":5,"soft_fail":false,"target_va":5380626464,"va":5379111633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::bulldoze_params::BulldozeParams::queue_wall_bulldoze
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1514523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e81b1c1700","size":5,"soft_fail":false,"target_va":5380627136,"va":5379112608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::eraser::bulldoze_params::BulldozeParams::try_decorator_delete_cmd
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1521830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379111312,"raw_hex":"e8a6381700","size":5,"soft_fail":false,"target_va":5380634016,"va":5379112181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":525,"record_offset":148628,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":525,"record_offset":141904,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":525,"record_offset":144636,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":525,"record_offset":142020,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":525,"record_offset":142744,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":525,"record_offset":144780,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":525,"record_offset":141932,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":525,"record_offset":147004,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":525,"record_offset":141820,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":525,"record_offset":142452,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":525,"record_offset":141736,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::History::is_recording
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":525,"record_offset":141680,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":31010,"source_line":984,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::build
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":525,"record_offset":146732,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":30599,"source_line":63,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::new_edit_sequence
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":525,"record_offset":141992,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":29291,"source_line":44,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next_empty
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":525,"record_offset":144292,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":30597,"source_line":49,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_editing::TerrainEditingData::get_stroke
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":525,"record_offset":143264,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":29030,"source_line":79,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_any
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_archivist.rs
//     evidence={"module":525,"record_offset":148540,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":31013,"source_line":294,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::mode_manager::WallGroupSelection::wall_ids
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/mode_manager.rs
//     evidence={"module":525,"record_offset":141708,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":29274,"source_line":73,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation::contains
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/terrain/animate_terrain_delete.rs
//     evidence={"module":525,"record_offset":148364,"parent_record_offset":141456,"ancestor_record_offset":141456,"inlinee":31012,"source_line":47,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":525,"record_offset":142344,"parent_record_offset":142320,"ancestor_record_offset":141456,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":525,"record_offset":142368,"parent_record_offset":142344,"ancestor_record_offset":141456,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":525,"record_offset":142640,"parent_record_offset":142616,"ancestor_record_offset":141456,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":525,"record_offset":142664,"parent_record_offset":142640,"ancestor_record_offset":141456,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":525,"record_offset":142392,"parent_record_offset":142368,"ancestor_record_offset":141456,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":525,"record_offset":142688,"parent_record_offset":142664,"ancestor_record_offset":141456,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":525,"record_offset":142320,"parent_record_offset":142140,"ancestor_record_offset":141456,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":525,"record_offset":142616,"parent_record_offset":142524,"ancestor_record_offset":141456,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":525,"record_offset":144452,"parent_record_offset":144420,"ancestor_record_offset":141456,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":525,"record_offset":142076,"parent_record_offset":142048,"ancestor_record_offset":141456,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":525,"record_offset":142552,"parent_record_offset":142524,"ancestor_record_offset":141456,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":525,"record_offset":142524,"parent_record_offset":142500,"ancestor_record_offset":141456,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":525,"record_offset":144500,"parent_record_offset":144476,"ancestor_record_offset":141456,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":525,"record_offset":144476,"parent_record_offset":144452,"ancestor_record_offset":141456,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":525,"record_offset":142140,"parent_record_offset":142104,"ancestor_record_offset":141456,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":525,"record_offset":142104,"parent_record_offset":142076,"ancestor_record_offset":141456,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":525,"record_offset":142500,"parent_record_offset":142476,"ancestor_record_offset":141456,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":525,"record_offset":142176,"parent_record_offset":142140,"ancestor_record_offset":141456,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":525,"record_offset":150496,"parent_record_offset":150472,"ancestor_record_offset":141456,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":525,"record_offset":144420,"parent_record_offset":144376,"ancestor_record_offset":141456,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":525,"record_offset":144320,"parent_record_offset":144292,"ancestor_record_offset":141456,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":525,"record_offset":144348,"parent_record_offset":144292,"ancestor_record_offset":141456,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":525,"record_offset":144376,"parent_record_offset":144348,"ancestor_record_offset":141456,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":525,"record_offset":142048,"parent_record_offset":142020,"ancestor_record_offset":141456,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":525,"record_offset":141844,"parent_record_offset":141820,"ancestor_record_offset":141456,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":525,"record_offset":146856,"parent_record_offset":146824,"ancestor_record_offset":141456,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":525,"record_offset":145804,"parent_record_offset":145768,"ancestor_record_offset":141456,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":525,"record_offset":144920,"parent_record_offset":144896,"ancestor_record_offset":141456,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":525,"record_offset":144968,"parent_record_offset":144944,"ancestor_record_offset":141456,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":525,"record_offset":145876,"parent_record_offset":145852,"ancestor_record_offset":141456,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":525,"record_offset":141960,"parent_record_offset":141932,"ancestor_record_offset":141456,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":525,"record_offset":147028,"parent_record_offset":147004,"ancestor_record_offset":141456,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":525,"record_offset":147936,"parent_record_offset":147908,"ancestor_record_offset":141456,"inlinee":8680,"source_line":125,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":525,"record_offset":147092,"parent_record_offset":147028,"ancestor_record_offset":141456,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":525,"record_offset":142200,"parent_record_offset":142176,"ancestor_record_offset":141456,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":525,"record_offset":143988,"parent_record_offset":143964,"ancestor_record_offset":141456,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":525,"record_offset":143556,"parent_record_offset":143532,"ancestor_record_offset":141456,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":525,"record_offset":143640,"parent_record_offset":143616,"ancestor_record_offset":141456,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":525,"record_offset":144056,"parent_record_offset":143420,"ancestor_record_offset":141456,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":525,"record_offset":142280,"parent_record_offset":142224,"ancestor_record_offset":141456,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":525,"record_offset":146936,"parent_record_offset":146912,"ancestor_record_offset":141456,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::try_process
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\mod.rs
//     evidence={"module":525,"record_offset":142924,"parent_record_offset":142892,"ancestor_record_offset":141456,"inlinee":11759,"source_line":152,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":525,"record_offset":141868,"parent_record_offset":141844,"ancestor_record_offset":141456,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":525,"record_offset":145852,"parent_record_offset":145828,"ancestor_record_offset":141456,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":525,"record_offset":145828,"parent_record_offset":145804,"ancestor_record_offset":141456,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":525,"record_offset":142988,"parent_record_offset":142964,"ancestor_record_offset":141456,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":525,"record_offset":146912,"parent_record_offset":146824,"ancestor_record_offset":141456,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\manually_drop.rs
//     evidence={"module":525,"record_offset":142772,"parent_record_offset":142744,"ancestor_record_offset":141456,"inlinee":5072,"source_line":184,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_dangling::MaybeDangling::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_dangling.rs
//     evidence={"module":525,"record_offset":142796,"parent_record_offset":142772,"ancestor_record_offset":141456,"inlinee":5074,"source_line":80,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":525,"record_offset":146788,"parent_record_offset":146764,"ancestor_record_offset":141456,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":525,"record_offset":146764,"parent_record_offset":146732,"ancestor_record_offset":141456,"inlinee":4795,"source_line":820,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":525,"record_offset":142224,"parent_record_offset":142200,"ancestor_record_offset":141456,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":525,"record_offset":142252,"parent_record_offset":142224,"ancestor_record_offset":141456,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":525,"record_offset":142584,"parent_record_offset":142552,"ancestor_record_offset":141456,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":525,"record_offset":143788,"parent_record_offset":143760,"ancestor_record_offset":141456,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":525,"record_offset":143760,"parent_record_offset":143732,"ancestor_record_offset":141456,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":525,"record_offset":148388,"parent_record_offset":148364,"ancestor_record_offset":141456,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$46::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":525,"record_offset":142892,"parent_record_offset":142860,"ancestor_record_offset":141456,"inlinee":11757,"source_line":2745,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$46::from_iter::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":525,"record_offset":142964,"parent_record_offset":142924,"ancestor_record_offset":141456,"inlinee":11761,"source_line":2749,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":525,"record_offset":145948,"parent_record_offset":145924,"ancestor_record_offset":141456,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":525,"record_offset":143580,"parent_record_offset":143556,"ancestor_record_offset":141456,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":525,"record_offset":142476,"parent_record_offset":142452,"ancestor_record_offset":141456,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":525,"record_offset":141784,"parent_record_offset":141760,"ancestor_record_offset":141456,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":525,"record_offset":145924,"parent_record_offset":145900,"ancestor_record_offset":141456,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":525,"record_offset":143904,"parent_record_offset":143880,"ancestor_record_offset":141456,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":525,"record_offset":145900,"parent_record_offset":145876,"ancestor_record_offset":141456,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":525,"record_offset":144836,"parent_record_offset":144780,"ancestor_record_offset":141456,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":525,"record_offset":144808,"parent_record_offset":144780,"ancestor_record_offset":141456,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":525,"record_offset":144860,"parent_record_offset":144836,"ancestor_record_offset":141456,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":525,"record_offset":144540,"parent_record_offset":144376,"ancestor_record_offset":141456,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":525,"record_offset":141760,"parent_record_offset":141736,"ancestor_record_offset":141456,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":525,"record_offset":148656,"parent_record_offset":148628,"ancestor_record_offset":141456,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":525,"record_offset":150532,"parent_record_offset":150496,"ancestor_record_offset":141456,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":525,"record_offset":148680,"parent_record_offset":148656,"ancestor_record_offset":141456,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":525,"record_offset":150556,"parent_record_offset":150532,"ancestor_record_offset":141456,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":525,"record_offset":150588,"parent_record_offset":150496,"ancestor_record_offset":141456,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":525,"record_offset":146824,"parent_record_offset":146732,"ancestor_record_offset":141456,"inlinee":30601,"source_line":29,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_editing::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_editing.rs
//     evidence={"module":525,"record_offset":144012,"parent_record_offset":143988,"ancestor_record_offset":141456,"inlinee":29024,"source_line":38,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":525,"record_offset":143700,"parent_record_offset":143672,"ancestor_record_offset":141456,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":525,"record_offset":143732,"parent_record_offset":143700,"ancestor_record_offset":141456,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":525,"record_offset":143672,"parent_record_offset":143420,"ancestor_record_offset":141456,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":525,"record_offset":143532,"parent_record_offset":143420,"ancestor_record_offset":141456,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":525,"record_offset":144084,"parent_record_offset":143420,"ancestor_record_offset":141456,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":525,"record_offset":143616,"parent_record_offset":143420,"ancestor_record_offset":141456,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":525,"record_offset":143476,"parent_record_offset":143420,"ancestor_record_offset":141456,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":525,"record_offset":143964,"parent_record_offset":143940,"ancestor_record_offset":141456,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":525,"record_offset":143312,"parent_record_offset":143288,"ancestor_record_offset":141456,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":525,"record_offset":143940,"parent_record_offset":143832,"ancestor_record_offset":141456,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":525,"record_offset":143344,"parent_record_offset":143312,"ancestor_record_offset":141456,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":525,"record_offset":143880,"parent_record_offset":143856,"ancestor_record_offset":141456,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":525,"record_offset":144176,"parent_record_offset":143420,"ancestor_record_offset":141456,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":525,"record_offset":143856,"parent_record_offset":143832,"ancestor_record_offset":141456,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":525,"record_offset":143396,"parent_record_offset":143372,"ancestor_record_offset":141456,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":525,"record_offset":143372,"parent_record_offset":143312,"ancestor_record_offset":141456,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":525,"record_offset":143420,"parent_record_offset":143396,"ancestor_record_offset":141456,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":525,"record_offset":143504,"parent_record_offset":143420,"ancestor_record_offset":141456,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":525,"record_offset":143832,"parent_record_offset":143420,"ancestor_record_offset":141456,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":525,"record_offset":143288,"parent_record_offset":143264,"ancestor_record_offset":141456,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00525.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::delete_focused::delete_focused::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8F68
// 首条位置: {"module":577,"inlinee":36712}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::contains
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":577,"record_offset":90308,"parent_record_offset":90044,"ancestor_record_offset":90044,"inlinee":35865,"source_line":132,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":577,"record_offset":91268,"parent_record_offset":90044,"ancestor_record_offset":90044,"inlinee":28986,"source_line":38,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer::contains
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs
//     evidence={"module":577,"record_offset":90104,"parent_record_offset":90044,"ancestor_record_offset":90044,"inlinee":36476,"source_line":67,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":577,"record_offset":92272,"parent_record_offset":90044,"ancestor_record_offset":90044,"inlinee":26304,"source_line":739,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":577,"record_offset":92364,"parent_record_offset":92304,"ancestor_record_offset":90044,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":577,"record_offset":91024,"parent_record_offset":91000,"ancestor_record_offset":90044,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":577,"record_offset":90592,"parent_record_offset":90568,"ancestor_record_offset":90044,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":577,"record_offset":90676,"parent_record_offset":90652,"ancestor_record_offset":90044,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":577,"record_offset":91092,"parent_record_offset":90460,"ancestor_record_offset":90044,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":577,"record_offset":90824,"parent_record_offset":90796,"ancestor_record_offset":90044,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":577,"record_offset":90796,"parent_record_offset":90768,"ancestor_record_offset":90044,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":577,"record_offset":92856,"parent_record_offset":92824,"ancestor_record_offset":90044,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":577,"record_offset":92700,"parent_record_offset":92664,"ancestor_record_offset":90044,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":577,"record_offset":90616,"parent_record_offset":90592,"ancestor_record_offset":90044,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":577,"record_offset":90940,"parent_record_offset":90916,"ancestor_record_offset":90044,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":90216,"parent_record_offset":90160,"ancestor_record_offset":90044,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":577,"record_offset":90188,"parent_record_offset":90160,"ancestor_record_offset":90044,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":577,"record_offset":92396,"parent_record_offset":92364,"ancestor_record_offset":90044,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":577,"record_offset":92428,"parent_record_offset":92396,"ancestor_record_offset":90044,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":90128,"parent_record_offset":90104,"ancestor_record_offset":90044,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":577,"record_offset":90160,"parent_record_offset":90128,"ancestor_record_offset":90044,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$10::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":577,"record_offset":90268,"parent_record_offset":90248,"ancestor_record_offset":90044,"inlinee":12117,"source_line":82,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::eraser::animate_wall_bulldoze::impl$2::contains::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/animate_wall_bulldoze.rs
//     evidence={"module":577,"record_offset":90248,"parent_record_offset":90128,"ancestor_record_offset":90044,"inlinee":36478,"source_line":68,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":577,"record_offset":92636,"parent_record_offset":92608,"ancestor_record_offset":90044,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":577,"record_offset":92664,"parent_record_offset":92636,"ancestor_record_offset":90044,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":577,"record_offset":92824,"parent_record_offset":92636,"ancestor_record_offset":90044,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":577,"record_offset":90736,"parent_record_offset":90708,"ancestor_record_offset":90044,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":577,"record_offset":90768,"parent_record_offset":90736,"ancestor_record_offset":90044,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":577,"record_offset":90708,"parent_record_offset":90460,"ancestor_record_offset":90044,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":577,"record_offset":90568,"parent_record_offset":90460,"ancestor_record_offset":90044,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":577,"record_offset":91120,"parent_record_offset":90460,"ancestor_record_offset":90044,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":577,"record_offset":90652,"parent_record_offset":90460,"ancestor_record_offset":90044,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":577,"record_offset":90512,"parent_record_offset":90460,"ancestor_record_offset":90044,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":577,"record_offset":91000,"parent_record_offset":90976,"ancestor_record_offset":90044,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":577,"record_offset":90356,"parent_record_offset":90332,"ancestor_record_offset":90044,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":577,"record_offset":91328,"parent_record_offset":91304,"ancestor_record_offset":90044,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":577,"record_offset":90976,"parent_record_offset":90868,"ancestor_record_offset":90044,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":577,"record_offset":90384,"parent_record_offset":90356,"ancestor_record_offset":90044,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":577,"record_offset":90916,"parent_record_offset":90892,"ancestor_record_offset":90044,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":577,"record_offset":91212,"parent_record_offset":90460,"ancestor_record_offset":90044,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":577,"record_offset":90892,"parent_record_offset":90868,"ancestor_record_offset":90044,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":577,"record_offset":90436,"parent_record_offset":90412,"ancestor_record_offset":90044,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":577,"record_offset":90412,"parent_record_offset":90356,"ancestor_record_offset":90044,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":577,"record_offset":90460,"parent_record_offset":90436,"ancestor_record_offset":90044,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":577,"record_offset":90540,"parent_record_offset":90460,"ancestor_record_offset":90044,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":577,"record_offset":90868,"parent_record_offset":90460,"ancestor_record_offset":90044,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":577,"record_offset":90332,"parent_record_offset":90308,"ancestor_record_offset":90044,"inlinee":21897,"source_line":1231,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":577,"record_offset":91304,"parent_record_offset":91268,"ancestor_record_offset":90044,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":577,"record_offset":92304,"parent_record_offset":92272,"ancestor_record_offset":90044,"inlinee":14727,"source_line":799,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":577,"record_offset":92240,"parent_record_offset":91268,"ancestor_record_offset":90044,"inlinee":28987,"source_line":165,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":577,"record_offset":92336,"parent_record_offset":92304,"ancestor_record_offset":90044,"inlinee":8705,"source_line":1332,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":577,"record_offset":92472,"parent_record_offset":92304,"ancestor_record_offset":90044,"inlinee":8706,"source_line":1337,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":577,"record_offset":92608,"parent_record_offset":92304,"ancestor_record_offset":90044,"inlinee":26201,"source_line":109,"archive":"_pdb/C/modules/00577.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::eraser::delete_focused::delete_focused::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x894A
// 首条位置: {"module":560,"inlinee":35146}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::wall::eraser::bulldoze_params::BulldozeParams::try_decorator_delete_cmd
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/eraser/bulldoze_params.rs
//     evidence={"module":1643,"record_offset":226760,"parent_record_offset":226740,"ancestor_record_offset":226740,"inlinee":35148,"source_line":165,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$47::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1643,"record_offset":226792,"parent_record_offset":226760,"ancestor_record_offset":226740,"inlinee":5260,"source_line":2765,"archive":"_pdb/C/modules/01643.jsonl.gz"}
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
// 完整结构及原始类型记录: delete_focused.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: delete_focused.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: delete_focused.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 33; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

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
// 函数: country_core::systems::wall::eraser::delete_focused::delete_action_used
// 信息 ID: 9b175cb3cdce1c36b4aabe40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 23c5cf19d22456f299aa91e3: bool (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<bool (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>),bool,bool (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>)> @ 0x1c7069 -> 0x9ecb10 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<bool (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>),bool,bool (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>)> @ 0x24c6f9 -> 0x9ecb10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::delete_focused::delete_focused
// 信息 ID: ddcb43a13f868b2e9ddee959
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c81113270a38c24a5a5f1be1: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,country_core::systems::wall::eraser::bulldoze_params::BulldozeParams,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui_edit_focus::TerrainFocusModeOrigin>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,country_core::systems::wall::eraser::bulldoze_params::BulldozeParams,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui_edit_focus::TerrainFocusModeOrigin>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,country_core::systems::wall::eraser::bulldoze_params::BulldozeParams,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui_edit_focus::TerrainFocusModeOrigin>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>)> @ 0x1e3a90 -> 0x9eb990 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,country_core::systems::wall::eraser::bulldoze_params::BulldozeParams,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui_edit_focus::TerrainFocusModeOrigin>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::mode_manager::WallGroupSelection>,country_core::systems::wall::eraser::bulldoze_params::BulldozeParams,bevy_ecs::change_detection::Res<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation>,bevy_ecs::change_detection::Res<country_core::systems::ui_edit_focus::TerrainFocusModeOrigin>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::history::FinishInputRecordingCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::mode_manager::RequestAppModeChange>)> > @ 0x296498 -> 0x9eb990 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::delete_focused::delete_focused::closure$0
// 信息 ID: 6ff06c3c5d5d3ccc89053398
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::eraser::delete_focused::delete_focused", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::eraser::delete_focused::delete_focused::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<country_core::resources::walls::WallId>,country_core::systems::wall::eraser::delete_focused::delete_focused::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::eraser::delete_focused::delete_focused::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<country_core::resources::walls::WallId,glam::f32::vec3::Vec3>,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<country_core::resources::walls::WallId> >,country_core::systems::wall::eraser::delete_focused::delete_focused::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<country_core::resources::walls::WallId>,country_core::systems::wall::eraser::delete_focused::delete_focused::closure_env$0>; module=577, inline_record=90044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::eraser::delete_focused::delete_focused::closure$1
// 信息 ID: ab730af40ea978bed1e059bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::eraser::delete_focused::delete_focused", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::eraser::delete_focused::delete_focused::closure_env$1 <- alloc::vec::in_place_collect::impl$1::from_iter<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::resources::walls::decorator::DecoratorId,alloc::alloc::Global>,country_core::systems::wall::eraser::delete_focused::delete_focused::closure_env$1>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::in_place_collect::impl$1::from_iter<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::resources::walls::decorator::DecoratorId,alloc::alloc::Global>,country_core::systems::wall::eraser::delete_focused::delete_focused::closure_env$1>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > > > >; module=1643, inline_record=226740; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::in_place_collect::impl$1::from_iter<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::resources::walls::decorator::DecoratorId,alloc::alloc::Global>,country_core::systems::wall::eraser::delete_focused::delete_focused::closure_env$1>,enum2$<core::option::Option<enum2$<core::convert::Infallible> > > > >; module=1643, inline_record=228128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
