// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/color/src/set_gate_color_id.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 98c405ce0b15cdaed9d240fb344e1444bb9a7593096923acac1f28ba27b069c2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: set_gate_color_id.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3762
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::set_gate_color_id::set_gate_color_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3762,"record_offset":1384,"rva":36280384,"va":5404989504}
// 编译实例 RVAs: 0x2299840
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: set_gate_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35722465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e81febdefd","size":5,"soft_fail":false,"target_va":5369267664,"va":5404990124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407b1810
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: set_gate_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28214677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e86b7a51fe","size":5,"soft_fail":false,"target_va":5376776208,"va":5404990880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140987ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_gate_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26287943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e8b9e06efe","size":5,"soft_fail":false,"target_va":5378702496,"va":5404990434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140beefa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_gate_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23768231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e8595395fe","size":5,"soft_fail":false,"target_va":5381222304,"va":5404990530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_gate_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23055809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e83f32a0fe","size":5,"soft_fail":false,"target_va":5381934560,"va":5404990364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1425f18b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: set_gate_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3506155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e8eb7f3500","size":5,"soft_fail":false,"target_va":5408495792,"va":5404989632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: set_gate_color_id.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6857451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e8eba26800","size":5,"soft_fail":false,"target_va":5411847152,"va":5404989696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-30633894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e85a902cfe","size":5,"soft_fail":false,"target_va":5374356528,"va":5404990417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6551092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e834f66300","size":5,"soft_fail":false,"target_va":5411541776,"va":5404990679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6491776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e8800e6300","size":5,"soft_fail":false,"target_va":5411482433,"va":5404990652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6551449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e899f76300","size":5,"soft_fail":false,"target_va":5411542064,"va":5404990610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6551624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e848f86300","size":5,"soft_fail":false,"target_va":5411542296,"va":5404990667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6552053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e8f5f96300","size":5,"soft_fail":false,"target_va":5411542880,"va":5404990822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24573289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e8970a89fe","size":5,"soft_fail":false,"target_va":5380417056,"va":5404990340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25197485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e853847ffe","size":5,"soft_fail":false,"target_va":5379792624,"va":5404990104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17144497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404989504,"raw_hex":"e84f65fafe","size":5,"soft_fail":false,"target_va":5387845824,"va":5404990316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5404989504,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5404989716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3762,"record_offset":3492,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3762,"record_offset":3436,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3762,"record_offset":3096,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3762,"record_offset":1560,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":37037,"source_line":102,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3762,"record_offset":2060,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3762,"record_offset":2744,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3762,"record_offset":2024,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3762,"record_offset":4128,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3762,"record_offset":1784,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::TypedStorageBuffer::gpu_write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3762,"record_offset":3924,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":11642,"source_line":156,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::bounding_box_2d
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":3762,"record_offset":3588,"parent_record_offset":1384,"ancestor_record_offset":1384,"inlinee":30460,"source_line":1082,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3762,"record_offset":2936,"parent_record_offset":2904,"ancestor_record_offset":1384,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3762,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":1384,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3762,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":1384,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3762,"record_offset":2904,"parent_record_offset":2860,"ancestor_record_offset":1384,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3762,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":1384,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3762,"record_offset":1684,"parent_record_offset":1656,"ancestor_record_offset":1384,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3762,"record_offset":2860,"parent_record_offset":2816,"ancestor_record_offset":1384,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3762,"record_offset":2364,"parent_record_offset":2132,"ancestor_record_offset":1384,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3762,"record_offset":3060,"parent_record_offset":2772,"ancestor_record_offset":1384,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3762,"record_offset":1656,"parent_record_offset":1624,"ancestor_record_offset":1384,"inlinee":37039,"source_line":232,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3762,"record_offset":2772,"parent_record_offset":2744,"ancestor_record_offset":1384,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3762,"record_offset":2816,"parent_record_offset":2772,"ancestor_record_offset":1384,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3762,"record_offset":1624,"parent_record_offset":1584,"ancestor_record_offset":1384,"inlinee":37038,"source_line":113,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3762,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":1384,"inlinee":8036,"source_line":78,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3762,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1384,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3762,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":1384,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3762,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":1384,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$18::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3762,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":1384,"inlinee":27686,"source_line":1227,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3762,"record_offset":1744,"parent_record_offset":1624,"ancestor_record_offset":1384,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3762,"record_offset":1852,"parent_record_offset":1812,"ancestor_record_offset":1384,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3762,"record_offset":3824,"parent_record_offset":3796,"ancestor_record_offset":1384,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3762,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":1384,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3762,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":1384,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3762,"record_offset":3148,"parent_record_offset":3120,"ancestor_record_offset":1384,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3762,"record_offset":3252,"parent_record_offset":3228,"ancestor_record_offset":1384,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":3762,"record_offset":3684,"parent_record_offset":3656,"ancestor_record_offset":1384,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3762,"record_offset":3656,"parent_record_offset":3612,"ancestor_record_offset":1384,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3762,"record_offset":4052,"parent_record_offset":4028,"ancestor_record_offset":1384,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3762,"record_offset":2320,"parent_record_offset":2292,"ancestor_record_offset":1384,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3762,"record_offset":1716,"parent_record_offset":1624,"ancestor_record_offset":1384,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3762,"record_offset":3196,"parent_record_offset":3172,"ancestor_record_offset":1384,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3762,"record_offset":3228,"parent_record_offset":3148,"ancestor_record_offset":1384,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3762,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":1384,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3762,"record_offset":3376,"parent_record_offset":3352,"ancestor_record_offset":1384,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3762,"record_offset":3752,"parent_record_offset":3712,"ancestor_record_offset":1384,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3762,"record_offset":3024,"parent_record_offset":2860,"ancestor_record_offset":1384,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":3762,"record_offset":4084,"parent_record_offset":3952,"ancestor_record_offset":1384,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3762,"record_offset":1936,"parent_record_offset":1908,"ancestor_record_offset":1384,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3762,"record_offset":1964,"parent_record_offset":1936,"ancestor_record_offset":1384,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3762,"record_offset":2236,"parent_record_offset":2132,"ancestor_record_offset":1384,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3762,"record_offset":4000,"parent_record_offset":3952,"ancestor_record_offset":1384,"inlinee":4614,"source_line":134,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3762,"record_offset":2588,"parent_record_offset":2132,"ancestor_record_offset":1384,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3762,"record_offset":3520,"parent_record_offset":3492,"ancestor_record_offset":1384,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3762,"record_offset":3548,"parent_record_offset":3520,"ancestor_record_offset":1384,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3762,"record_offset":2292,"parent_record_offset":2264,"ancestor_record_offset":1384,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3762,"record_offset":2264,"parent_record_offset":2236,"ancestor_record_offset":1384,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3762,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":1384,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3762,"record_offset":3352,"parent_record_offset":3328,"ancestor_record_offset":1384,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3762,"record_offset":3304,"parent_record_offset":3252,"ancestor_record_offset":1384,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3762,"record_offset":3328,"parent_record_offset":3304,"ancestor_record_offset":1384,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3762,"record_offset":1880,"parent_record_offset":1852,"ancestor_record_offset":1384,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3762,"record_offset":1812,"parent_record_offset":1784,"ancestor_record_offset":1384,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::ssbo::TypedStorageBuffer::gpu_write_with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3762,"record_offset":3952,"parent_record_offset":3924,"ancestor_record_offset":1384,"inlinee":11643,"source_line":208,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3762,"record_offset":3796,"parent_record_offset":3612,"ancestor_record_offset":1384,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3762,"record_offset":3860,"parent_record_offset":3612,"ancestor_record_offset":1384,"inlinee":10957,"source_line":182,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":3762,"record_offset":4028,"parent_record_offset":3952,"ancestor_record_offset":1384,"inlinee":8265,"source_line":8,"archive":"_pdb/B/modules/03762.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::aabb::Aabb2::from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":3762,"record_offset":3612,"parent_record_offset":3588,"ancestor_record_offset":1384,"inlinee":30462,"source_line":92,"archive":"_pdb/B/modules/03762.jsonl.gz"}
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
// 完整结构及原始类型记录: set_gate_color_id.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: set_gate_color_id.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: set_gate_color_id.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::event::reader::EventReader<country_core::resources::color_id::SetGateColorIdCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>", "bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::color_id::SetGateColorIdCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>

// ------------------------------------------------------------------
// 函数: system_color::set_gate_color_id::set_gate_color_id
// 信息 ID: 8341de73437310292d371a67
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b3b07744230bc3e04d48a746: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>)> > > @ 0x1c9107c -> 0x2299840 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::reader::EventReader<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::ResMut<country_core::resources::color_id::GateColorIds>)> > @ 0x1ca962a -> 0x2299840 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
