// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/toolbar/src/photomode/photomode_buttons.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ea24c224ece6f6fdb776d417623d5554e4ae995552c6c7eebcd0c6de697f6289
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: photomode_buttons.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3916
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use open; // namespace candidate
//   package identities: [{"name":"open","source":"registry+https://github.com/rust-lang/crates.io-index","version":"5.0.0"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::photomode::photomode_buttons::photomode_buttons
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3916,"record_offset":256,"rva":37012016,"va":5405721136}
// 编译实例 RVAs: 0x234c230
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31045553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e84f4826fe","size":5,"soft_fail":false,"target_va":5374676336,"va":5405721884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bd910
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30993096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8381527fe","size":5,"soft_fail":false,"target_va":5374728464,"va":5405721555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdda0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30991738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8861a27fe","size":5,"soft_fail":false,"target_va":5374729632,"va":5405721365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdf20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30991518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8621b27fe","size":5,"soft_fail":false,"target_va":5374730016,"va":5405721529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=3
//     target_aliases=92 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30823774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8a2aa29fe","size":5,"soft_fail":false,"target_va":5374900736,"va":5405724505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30643155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e82d6c2cfe","size":5,"soft_fail":false,"target_va":5375081504,"va":5405724654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14068bfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30147879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8d9fa33fe","size":5,"soft_fail":false,"target_va":5375573952,"va":5405721826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408aea50
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27912214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8ea1756fe","size":5,"soft_fail":false,"target_va":5377813072,"va":5405725281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140911240
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27506706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8ee475cfe","size":5,"soft_fail":false,"target_va":5378216512,"va":5405723213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093fad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27316291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8bd2f5ffe","size":5,"soft_fail":false,"target_va":5378407120,"va":5405723406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a86b30
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25976968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8789f73fe","size":5,"soft_fail":false,"target_va":5379746608,"va":5405723571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22896683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8d59fa2fe","size":5,"soft_fail":false,"target_va":5382827728,"va":5405724406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9e970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20634418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8ce24c5fe","size":5,"soft_fail":false,"target_va":5385087344,"va":5405721757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fb8070
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20530254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8b2bbc6fe","size":5,"soft_fail":false,"target_va":5385191536,"va":5405721785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fcdf40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: photomode_buttons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20443204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8bc0fc8fe","size":5,"soft_fail":false,"target_va":5385281344,"va":5405724543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t-30517642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e876562efe","size":5,"soft_fail":false,"target_va":5375203792,"va":5405721429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5605448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e848885500","size":5,"soft_fail":false,"target_va":5411330256,"va":5405724803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::writer::EventWriter$LT$E$GT$::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-20435858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e86e2cc8fe","size":5,"soft_fail":false,"target_va":5385288496,"va":5405724349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5815618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e842bd5800","size":5,"soft_fail":false,"target_va":5411541808,"va":5405726185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5815850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e82abe5800","size":5,"soft_fail":false,"target_va":5411542064,"va":5405726209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::image_formation::ImageFormationConfigs::iter_non_default_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27910209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8bf1f56fe","size":5,"soft_fail":false,"target_va":5377813088,"va":5405723292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::photo_mode::PhotoModeCameraMode::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25114209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e89fc980fe","size":5,"soft_fail":false,"target_va":5380609824,"va":5405724028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::photo_mode::PhotoModeCameraMode::next_demo
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25113477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e87bcc80fe","size":5,"soft_fail":false,"target_va":5380609712,"va":5405723184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::VermeerState::activate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27938225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e84fb255fe","size":5,"soft_fail":false,"target_va":5377785952,"va":5405724172,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::VermeerState::deactivate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27937840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8d0b355fe","size":5,"soft_fail":false,"target_va":5377786176,"va":5405724011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::vermeer::VermeerState::is_active
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27937524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e80cb555fe","size":5,"soft_fail":false,"target_va":5377786464,"va":5405723983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24834127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8b10f85fe","size":5,"soft_fail":false,"target_va":5380887680,"va":5405721802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] open::that
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"open","version":"5.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2988799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8ff9a2d00","size":5,"soft_fail":false,"target_va":5408713280,"va":5405724476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::config
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24834967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8690c85fe","size":5,"soft_fail":false,"target_va":5380889392,"va":5405724354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::create_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2990620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e81ca22d00","size":5,"soft_fail":false,"target_va":5408715200,"va":5405724575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-22980316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e82459a1fe","size":5,"soft_fail":false,"target_va":5382744080,"va":5405724391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_dir
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-22978810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8065fa1fe","size":5,"soft_fail":false,"target_va":5382745616,"va":5405724421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30180466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e88e7b33fe","size":5,"soft_fail":false,"target_va":5375540928,"va":5405721389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30171704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8c89d33fe","size":5,"soft_fail":false,"target_va":5375549872,"va":5405721571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::MenuButtonView::hotkey
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30203433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405721136,"raw_hex":"e8d72133fe","size":5,"soft_fail":false,"target_va":5375518416,"va":5405721844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3916,"record_offset":472,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3916,"record_offset":12368,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3916,"record_offset":5028,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3916,"record_offset":2404,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3916,"record_offset":6044,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3916,"record_offset":9112,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3916,"record_offset":7792,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3916,"record_offset":3104,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":13654,"source_line":1576,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3916,"record_offset":8768,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":3916,"record_offset":5696,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":9843,"source_line":63,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3916,"record_offset":8256,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::photo_mode::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/photo_mode.rs
//     evidence={"module":3916,"record_offset":12336,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":122703,"source_line":19,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3916,"record_offset":8088,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3916,"record_offset":8284,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3916,"record_offset":8116,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$18::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":3916,"record_offset":1044,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":20074,"source_line":167,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3916,"record_offset":532,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":10705,"source_line":445,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3916,"record_offset":2264,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":20032,"source_line":331,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3916,"record_offset":820,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$31::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":3916,"record_offset":1072,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":19767,"source_line":192,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3916,"record_offset":8676,"parent_record_offset":8648,"ancestor_record_offset":256,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3916,"record_offset":8648,"parent_record_offset":8592,"ancestor_record_offset":256,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3916,"record_offset":692,"parent_record_offset":660,"ancestor_record_offset":256,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3916,"record_offset":7936,"parent_record_offset":7908,"ancestor_record_offset":256,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3916,"record_offset":8020,"parent_record_offset":7996,"ancestor_record_offset":256,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3916,"record_offset":8620,"parent_record_offset":8592,"ancestor_record_offset":256,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3916,"record_offset":8592,"parent_record_offset":8568,"ancestor_record_offset":256,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3916,"record_offset":7996,"parent_record_offset":7964,"ancestor_record_offset":256,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3916,"record_offset":740,"parent_record_offset":716,"ancestor_record_offset":256,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3916,"record_offset":716,"parent_record_offset":692,"ancestor_record_offset":256,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3916,"record_offset":7964,"parent_record_offset":7936,"ancestor_record_offset":256,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3916,"record_offset":8568,"parent_record_offset":8544,"ancestor_record_offset":256,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3916,"record_offset":660,"parent_record_offset":624,"ancestor_record_offset":256,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3916,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":256,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3916,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":256,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3916,"record_offset":9408,"parent_record_offset":9384,"ancestor_record_offset":256,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3916,"record_offset":9432,"parent_record_offset":9408,"ancestor_record_offset":256,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3916,"record_offset":624,"parent_record_offset":556,"ancestor_record_offset":256,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3916,"record_offset":7908,"parent_record_offset":7876,"ancestor_record_offset":256,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3916,"record_offset":2724,"parent_record_offset":2492,"ancestor_record_offset":256,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3916,"record_offset":3312,"parent_record_offset":3280,"ancestor_record_offset":256,"inlinee":5673,"source_line":621,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3916,"record_offset":12076,"parent_record_offset":9144,"ancestor_record_offset":256,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3916,"record_offset":10996,"parent_record_offset":9144,"ancestor_record_offset":256,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3916,"record_offset":11080,"parent_record_offset":9144,"ancestor_record_offset":256,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3916,"record_offset":12272,"parent_record_offset":9144,"ancestor_record_offset":256,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3916,"record_offset":4648,"parent_record_offset":3132,"ancestor_record_offset":256,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3916,"record_offset":4436,"parent_record_offset":4100,"ancestor_record_offset":256,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3916,"record_offset":500,"parent_record_offset":472,"ancestor_record_offset":256,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3916,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":256,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3916,"record_offset":12396,"parent_record_offset":12368,"ancestor_record_offset":256,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3916,"record_offset":4100,"parent_record_offset":3132,"ancestor_record_offset":256,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3916,"record_offset":5440,"parent_record_offset":5368,"ancestor_record_offset":256,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3916,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":256,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3916,"record_offset":2468,"parent_record_offset":2428,"ancestor_record_offset":256,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3916,"record_offset":2492,"parent_record_offset":2468,"ancestor_record_offset":256,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3916,"record_offset":5052,"parent_record_offset":5028,"ancestor_record_offset":256,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3916,"record_offset":10744,"parent_record_offset":10720,"ancestor_record_offset":256,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::matches_component_set
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3916,"record_offset":3240,"parent_record_offset":3204,"ancestor_record_offset":256,"inlinee":5720,"source_line":1210,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3916,"record_offset":3580,"parent_record_offset":3176,"ancestor_record_offset":256,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3916,"record_offset":3608,"parent_record_offset":3580,"ancestor_record_offset":256,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3916,"record_offset":10720,"parent_record_offset":10696,"ancestor_record_offset":256,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3916,"record_offset":10768,"parent_record_offset":10744,"ancestor_record_offset":256,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3916,"record_offset":11720,"parent_record_offset":11172,"ancestor_record_offset":256,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3916,"record_offset":9984,"parent_record_offset":9468,"ancestor_record_offset":256,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3916,"record_offset":3204,"parent_record_offset":3176,"ancestor_record_offset":256,"inlinee":17008,"source_line":1799,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3916,"record_offset":3280,"parent_record_offset":3240,"ancestor_record_offset":256,"inlinee":17010,"source_line":1805,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3916,"record_offset":10468,"parent_record_offset":9468,"ancestor_record_offset":256,"inlinee":17014,"source_line":1815,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3916,"record_offset":10516,"parent_record_offset":10492,"ancestor_record_offset":256,"inlinee":17016,"source_line":1816,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$63::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3916,"record_offset":10696,"parent_record_offset":9144,"ancestor_record_offset":256,"inlinee":13690,"source_line":2083,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3916,"record_offset":8816,"parent_record_offset":8792,"ancestor_record_offset":256,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3916,"record_offset":8840,"parent_record_offset":8816,"ancestor_record_offset":256,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3916,"record_offset":9144,"parent_record_offset":9112,"ancestor_record_offset":256,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3916,"record_offset":3176,"parent_record_offset":3132,"ancestor_record_offset":256,"inlinee":11680,"source_line":182,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3916,"record_offset":9468,"parent_record_offset":9144,"ancestor_record_offset":256,"inlinee":11676,"source_line":195,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3916,"record_offset":4044,"parent_record_offset":4020,"ancestor_record_offset":256,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3916,"record_offset":3376,"parent_record_offset":3344,"ancestor_record_offset":256,"inlinee":5675,"source_line":49,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3916,"record_offset":3724,"parent_record_offset":3696,"ancestor_record_offset":256,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3916,"record_offset":3344,"parent_record_offset":3312,"ancestor_record_offset":256,"inlinee":5674,"source_line":388,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3916,"record_offset":3696,"parent_record_offset":3668,"ancestor_record_offset":256,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3916,"record_offset":3904,"parent_record_offset":3880,"ancestor_record_offset":256,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3916,"record_offset":3668,"parent_record_offset":3636,"ancestor_record_offset":256,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3916,"record_offset":3636,"parent_record_offset":3608,"ancestor_record_offset":256,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3916,"record_offset":10540,"parent_record_offset":10516,"ancestor_record_offset":256,"inlinee":17017,"source_line":496,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3916,"record_offset":9384,"parent_record_offset":9144,"ancestor_record_offset":256,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3916,"record_offset":10792,"parent_record_offset":10768,"ancestor_record_offset":256,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3916,"record_offset":4760,"parent_record_offset":3132,"ancestor_record_offset":256,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3916,"record_offset":10408,"parent_record_offset":9984,"ancestor_record_offset":256,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3916,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":256,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3916,"record_offset":3996,"parent_record_offset":3972,"ancestor_record_offset":256,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3916,"record_offset":10432,"parent_record_offset":10408,"ancestor_record_offset":256,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3916,"record_offset":7848,"parent_record_offset":7820,"ancestor_record_offset":256,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3916,"record_offset":7876,"parent_record_offset":7848,"ancestor_record_offset":256,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":3916,"record_offset":7820,"parent_record_offset":7792,"ancestor_record_offset":256,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3916,"record_offset":3132,"parent_record_offset":3104,"ancestor_record_offset":256,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3916,"record_offset":8792,"parent_record_offset":8768,"ancestor_record_offset":256,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3916,"record_offset":2428,"parent_record_offset":2404,"ancestor_record_offset":256,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3916,"record_offset":4496,"parent_record_offset":4468,"ancestor_record_offset":256,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3916,"record_offset":4404,"parent_record_offset":4380,"ancestor_record_offset":256,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3916,"record_offset":2008,"parent_record_offset":1984,"ancestor_record_offset":256,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3916,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":256,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3916,"record_offset":1596,"parent_record_offset":1572,"ancestor_record_offset":256,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3916,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":256,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3916,"record_offset":2076,"parent_record_offset":1376,"ancestor_record_offset":256,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3916,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":256,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3916,"record_offset":5080,"parent_record_offset":5052,"ancestor_record_offset":256,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3916,"record_offset":5184,"parent_record_offset":5160,"ancestor_record_offset":256,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3916,"record_offset":5744,"parent_record_offset":5720,"ancestor_record_offset":256,"inlinee":6091,"source_line":73,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3916,"record_offset":5796,"parent_record_offset":5768,"ancestor_record_offset":256,"inlinee":6094,"source_line":85,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3916,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":256,"inlinee":6090,"source_line":2852,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3916,"record_offset":5824,"parent_record_offset":5796,"ancestor_record_offset":256,"inlinee":6096,"source_line":2859,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3916,"record_offset":5768,"parent_record_offset":5744,"ancestor_record_offset":256,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3916,"record_offset":2680,"parent_record_offset":2652,"ancestor_record_offset":256,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3916,"record_offset":2540,"parent_record_offset":2492,"ancestor_record_offset":256,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3916,"record_offset":1748,"parent_record_offset":1720,"ancestor_record_offset":256,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3916,"record_offset":1720,"parent_record_offset":1692,"ancestor_record_offset":256,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3916,"record_offset":4380,"parent_record_offset":4100,"ancestor_record_offset":256,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3916,"record_offset":3496,"parent_record_offset":3412,"ancestor_record_offset":256,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3916,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":256,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3916,"record_offset":10108,"parent_record_offset":10084,"ancestor_record_offset":256,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3916,"record_offset":5128,"parent_record_offset":5104,"ancestor_record_offset":256,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3916,"record_offset":3840,"parent_record_offset":3816,"ancestor_record_offset":256,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3916,"record_offset":3520,"parent_record_offset":3496,"ancestor_record_offset":256,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3916,"record_offset":3880,"parent_record_offset":3696,"ancestor_record_offset":256,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3916,"record_offset":5160,"parent_record_offset":5080,"ancestor_record_offset":256,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3916,"record_offset":8448,"parent_record_offset":8424,"ancestor_record_offset":256,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3916,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":256,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3916,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":256,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3916,"record_offset":5308,"parent_record_offset":5284,"ancestor_record_offset":256,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3916,"record_offset":1136,"parent_record_offset":1104,"ancestor_record_offset":256,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3916,"record_offset":5940,"parent_record_offset":5912,"ancestor_record_offset":256,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3916,"record_offset":780,"parent_record_offset":624,"ancestor_record_offset":256,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3916,"record_offset":2596,"parent_record_offset":2492,"ancestor_record_offset":256,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3916,"record_offset":2948,"parent_record_offset":2492,"ancestor_record_offset":256,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3916,"record_offset":4204,"parent_record_offset":4176,"ancestor_record_offset":256,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3916,"record_offset":3460,"parent_record_offset":3412,"ancestor_record_offset":256,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3916,"record_offset":10276,"parent_record_offset":10244,"ancestor_record_offset":256,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3916,"record_offset":2652,"parent_record_offset":2624,"ancestor_record_offset":256,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3916,"record_offset":2624,"parent_record_offset":2596,"ancestor_record_offset":256,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3916,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":256,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3916,"record_offset":5284,"parent_record_offset":5260,"ancestor_record_offset":256,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3916,"record_offset":5236,"parent_record_offset":5184,"ancestor_record_offset":256,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3916,"record_offset":5260,"parent_record_offset":5236,"ancestor_record_offset":256,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3916,"record_offset":5912,"parent_record_offset":5768,"ancestor_record_offset":256,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$1::iter_non_default_mut::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":3916,"record_offset":5872,"parent_record_offset":5848,"ancestor_record_offset":256,"inlinee":26595,"source_line":110,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":3916,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":256,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3916,"record_offset":4468,"parent_record_offset":3132,"ancestor_record_offset":256,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3916,"record_offset":4544,"parent_record_offset":4520,"ancestor_record_offset":256,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3916,"record_offset":4568,"parent_record_offset":4544,"ancestor_record_offset":256,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3916,"record_offset":4520,"parent_record_offset":4496,"ancestor_record_offset":256,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3916,"record_offset":1656,"parent_record_offset":1628,"ancestor_record_offset":256,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3916,"record_offset":1692,"parent_record_offset":1656,"ancestor_record_offset":256,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":3916,"record_offset":1628,"parent_record_offset":1376,"ancestor_record_offset":256,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3916,"record_offset":1488,"parent_record_offset":1376,"ancestor_record_offset":256,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3916,"record_offset":2104,"parent_record_offset":1376,"ancestor_record_offset":256,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":3916,"record_offset":1572,"parent_record_offset":1376,"ancestor_record_offset":256,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":3916,"record_offset":1460,"parent_record_offset":1376,"ancestor_record_offset":256,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":3916,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":256,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3916,"record_offset":1240,"parent_record_offset":1200,"ancestor_record_offset":256,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3916,"record_offset":1268,"parent_record_offset":1240,"ancestor_record_offset":256,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3916,"record_offset":1960,"parent_record_offset":1792,"ancestor_record_offset":256,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":3916,"record_offset":1300,"parent_record_offset":1268,"ancestor_record_offset":256,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3916,"record_offset":1900,"parent_record_offset":1816,"ancestor_record_offset":256,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3916,"record_offset":1844,"parent_record_offset":1816,"ancestor_record_offset":256,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3916,"record_offset":2196,"parent_record_offset":1376,"ancestor_record_offset":256,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3916,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":256,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3916,"record_offset":1352,"parent_record_offset":1328,"ancestor_record_offset":256,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3916,"record_offset":1328,"parent_record_offset":1268,"ancestor_record_offset":256,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3916,"record_offset":1376,"parent_record_offset":1352,"ancestor_record_offset":256,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3916,"record_offset":1432,"parent_record_offset":1376,"ancestor_record_offset":256,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":3916,"record_offset":1792,"parent_record_offset":1376,"ancestor_record_offset":256,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3916,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":256,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3916,"record_offset":1200,"parent_record_offset":1164,"ancestor_record_offset":256,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":3916,"record_offset":1164,"parent_record_offset":1104,"ancestor_record_offset":256,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3916,"record_offset":8308,"parent_record_offset":8284,"ancestor_record_offset":256,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3916,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":256,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3916,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":256,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":3916,"record_offset":8188,"parent_record_offset":8164,"ancestor_record_offset":256,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":3916,"record_offset":8140,"parent_record_offset":8116,"ancestor_record_offset":256,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":3916,"record_offset":8332,"parent_record_offset":8308,"ancestor_record_offset":256,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":3916,"record_offset":8356,"parent_record_offset":8332,"ancestor_record_offset":256,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3916,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":256,"inlinee":10701,"source_line":494,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3916,"record_offset":1104,"parent_record_offset":1072,"ancestor_record_offset":256,"inlinee":19719,"source_line":1521,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":3916,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":256,"inlinee":19915,"source_line":597,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":3916,"record_offset":2312,"parent_record_offset":2288,"ancestor_record_offset":256,"inlinee":19916,"source_line":585,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3916,"record_offset":596,"parent_record_offset":556,"ancestor_record_offset":256,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3916,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":256,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3916,"record_offset":896,"parent_record_offset":844,"ancestor_record_offset":256,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/03916.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":3916,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":256,"inlinee":7072,"source_line":154,"archive":"_pdb/B/modules/03916.jsonl.gz"}
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
// 完整结构及原始类型记录: photomode_buttons.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: photomode_buttons.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: photomode_buttons.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >", "bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>", "bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>", "bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >", "bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>", "tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> > >", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>", "bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: toolbar::photomode::photomode_buttons::photomode_buttons
// 信息 ID: d58f5fd5483521f53da7e092
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 7c0e2980706fff2710ab7f23: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)> > > @ 0x1e09c85 -> 0x234c230 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::system::query::Query<tuple$<ref$<enum2$<toolbar::resouces_and_components::PhotomodeButton> >,ref_mut$<country_core::components::UiInteractionState>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiLocked> > > >,tuple$<> >,bevy_ecs::event::reader::EventReader<toolbar::ui_interaction::UiClickEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::PhotomodeScreen>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>,bevy_ecs::change_detection::ResMut<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::ResMut<country_core::resources::image_formation::ImageFormationConfigs>,tuple$<bevy_ecs::event::writer::EventWriter<country_core::systems::mode_manager::RequestAppModeChange>,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::photo_mode::PhotoModeCmd> >,bevy_ecs::event::writer::EventWriter<enum2$<country_core::resources::window::OsWindowCmd> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::platform::RuntimePlatform>,bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)> > @ 0x23a452f -> 0x234c230 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
