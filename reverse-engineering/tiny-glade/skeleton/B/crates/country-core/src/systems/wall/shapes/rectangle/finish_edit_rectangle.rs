// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/wall/shapes/rectangle/finish_edit_rectangle.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c0783e1928170ace4d4eea5c3459d13b875fb9dc5089f9882f3bf0d9f71a453c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: finish_edit_rectangle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1606
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::finish_edit_rectangle
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1606,"record_offset":106788,"rva":21383776,"va":5390092896}
// 编译实例 RVAs: 0x1464a60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: finish_edit_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15417385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390092896,"raw_hex":"e8d7bf14ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5390093716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140898470
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: finish_edit_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12373236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390092896,"raw_hex":"e80c3343ff","size":5,"soft_fail":false,"target_va":5377721456,"va":5390094687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141696f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: finish_edit_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2301946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390092896,"raw_hex":"e8fa1f2300","size":5,"soft_fail":false,"target_va":5392396112,"va":5390094161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923de0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: finish_edit_rectangle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21754043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390092896,"raw_hex":"e8bbf04b01","size":5,"soft_fail":false,"target_va":5411847648,"va":5390093600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-15735927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390092896,"raw_hex":"e889e30fff","size":5,"soft_fail":false,"target_va":5374357296,"va":5390093218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21447405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390092896,"raw_hex":"e8ed424701","size":5,"soft_fail":false,"target_va":5411541776,"va":5390094366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t21447484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390092896,"raw_hex":"e83c434701","size":5,"soft_fail":false,"target_va":5411542064,"va":5390094575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21447348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390092896,"raw_hex":"e8b4424701","size":5,"soft_fail":false,"target_va":5411541904,"va":5390094551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10354880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390092896,"raw_hex":"e840ff61ff","size":5,"soft_fail":false,"target_va":5379739328,"va":5390094203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10363806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390092896,"raw_hex":"e862dc61ff","size":5,"soft_fail":false,"target_va":5379730240,"va":5390094041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_and_restore_to_storage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9669535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390092896,"raw_hex":"e861746cff","size":5,"soft_fail":false,"target_va":5380424880,"va":5390094410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1606,"record_offset":109644,"parent_record_offset":106788,"ancestor_record_offset":106788,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":1606,"record_offset":108072,"parent_record_offset":106788,"ancestor_record_offset":106788,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":1606,"record_offset":107508,"parent_record_offset":106788,"ancestor_record_offset":106788,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":1606,"record_offset":107180,"parent_record_offset":106788,"ancestor_record_offset":106788,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1606,"record_offset":108320,"parent_record_offset":106788,"ancestor_record_offset":106788,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::to_degrees
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1606,"record_offset":108348,"parent_record_offset":106788,"ancestor_record_offset":106788,"inlinee":29988,"source_line":847,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1606,"record_offset":110848,"parent_record_offset":106788,"ancestor_record_offset":106788,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::build
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1606,"record_offset":109704,"parent_record_offset":106788,"ancestor_record_offset":106788,"inlinee":27922,"source_line":62,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::wall_compound_edit::CompoundEditBuilder::new_edit_sequence
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1606,"record_offset":109616,"parent_record_offset":106788,"ancestor_record_offset":106788,"inlinee":27917,"source_line":43,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::inner_walls::InnerWalls::get_unwrapped
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_walls.rs
//     evidence={"module":1606,"record_offset":108524,"parent_record_offset":106788,"ancestor_record_offset":106788,"inlinee":26531,"source_line":116,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::shapes::rectangle::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/shapes/rectangle/mod.rs
//     evidence={"module":1606,"record_offset":107020,"parent_record_offset":106788,"ancestor_record_offset":106788,"inlinee":73729,"source_line":30,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1606,"record_offset":108264,"parent_record_offset":106788,"ancestor_record_offset":106788,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1606,"record_offset":108432,"parent_record_offset":106788,"ancestor_record_offset":106788,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1606,"record_offset":108376,"parent_record_offset":106788,"ancestor_record_offset":106788,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::acos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1606,"record_offset":108292,"parent_record_offset":106788,"ancestor_record_offset":106788,"inlinee":31906,"source_line":760,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1606,"record_offset":107376,"parent_record_offset":107344,"ancestor_record_offset":106788,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1606,"record_offset":107424,"parent_record_offset":107400,"ancestor_record_offset":106788,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1606,"record_offset":107400,"parent_record_offset":107376,"ancestor_record_offset":106788,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1606,"record_offset":107344,"parent_record_offset":107304,"ancestor_record_offset":106788,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1606,"record_offset":107744,"parent_record_offset":107720,"ancestor_record_offset":106788,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1606,"record_offset":107304,"parent_record_offset":107268,"ancestor_record_offset":106788,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1606,"record_offset":109876,"parent_record_offset":109852,"ancestor_record_offset":106788,"inlinee":4883,"source_line":3450,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1606,"record_offset":107720,"parent_record_offset":107620,"ancestor_record_offset":106788,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1606,"record_offset":107240,"parent_record_offset":107204,"ancestor_record_offset":106788,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1606,"record_offset":107204,"parent_record_offset":107180,"ancestor_record_offset":106788,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":1606,"record_offset":107268,"parent_record_offset":107204,"ancestor_record_offset":106788,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":1606,"record_offset":107588,"parent_record_offset":107540,"ancestor_record_offset":106788,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":1606,"record_offset":107620,"parent_record_offset":107588,"ancestor_record_offset":106788,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":1606,"record_offset":108104,"parent_record_offset":108072,"ancestor_record_offset":106788,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":1606,"record_offset":107540,"parent_record_offset":107508,"ancestor_record_offset":106788,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1606,"record_offset":109340,"parent_record_offset":109316,"ancestor_record_offset":106788,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1606,"record_offset":108848,"parent_record_offset":108824,"ancestor_record_offset":106788,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1606,"record_offset":108932,"parent_record_offset":108908,"ancestor_record_offset":106788,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1606,"record_offset":108872,"parent_record_offset":108848,"ancestor_record_offset":106788,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1606,"record_offset":109408,"parent_record_offset":108712,"ancestor_record_offset":106788,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1606,"record_offset":108140,"parent_record_offset":108104,"ancestor_record_offset":106788,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1606,"record_offset":109956,"parent_record_offset":109932,"ancestor_record_offset":106788,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1606,"record_offset":109988,"parent_record_offset":109852,"ancestor_record_offset":106788,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1606,"record_offset":109932,"parent_record_offset":109852,"ancestor_record_offset":106788,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1606,"record_offset":109760,"parent_record_offset":109736,"ancestor_record_offset":106788,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1606,"record_offset":109736,"parent_record_offset":109704,"ancestor_record_offset":106788,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":107848,"parent_record_offset":107824,"ancestor_record_offset":106788,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1606,"record_offset":107664,"parent_record_offset":107620,"ancestor_record_offset":106788,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1606,"record_offset":109080,"parent_record_offset":109052,"ancestor_record_offset":106788,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1606,"record_offset":109052,"parent_record_offset":109024,"ancestor_record_offset":106788,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1606,"record_offset":108400,"parent_record_offset":108376,"ancestor_record_offset":106788,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1606,"record_offset":108168,"parent_record_offset":108140,"ancestor_record_offset":106788,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1606,"record_offset":109584,"parent_record_offset":108524,"ancestor_record_offset":106788,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1606,"record_offset":108228,"parent_record_offset":108104,"ancestor_record_offset":106788,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1606,"record_offset":109900,"parent_record_offset":109876,"ancestor_record_offset":106788,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1606,"record_offset":109200,"parent_record_offset":109176,"ancestor_record_offset":106788,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1606,"record_offset":109788,"parent_record_offset":109760,"ancestor_record_offset":106788,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1606,"record_offset":107464,"parent_record_offset":107304,"ancestor_record_offset":106788,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1606,"record_offset":107776,"parent_record_offset":107620,"ancestor_record_offset":106788,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1606,"record_offset":107824,"parent_record_offset":107800,"ancestor_record_offset":106788,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1606,"record_offset":107800,"parent_record_offset":107776,"ancestor_record_offset":106788,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::wall_compound_edit::WallHistoryCompoundEdit::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/wall_compound_edit.rs
//     evidence={"module":1606,"record_offset":109852,"parent_record_offset":109704,"ancestor_record_offset":106788,"inlinee":27915,"source_line":28,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1606,"record_offset":109364,"parent_record_offset":109340,"ancestor_record_offset":106788,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1606,"record_offset":108456,"parent_record_offset":108432,"ancestor_record_offset":106788,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1606,"record_offset":108992,"parent_record_offset":108964,"ancestor_record_offset":106788,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1606,"record_offset":109024,"parent_record_offset":108992,"ancestor_record_offset":106788,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1606,"record_offset":108964,"parent_record_offset":108712,"ancestor_record_offset":106788,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1606,"record_offset":108824,"parent_record_offset":108712,"ancestor_record_offset":106788,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1606,"record_offset":109436,"parent_record_offset":108712,"ancestor_record_offset":106788,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1606,"record_offset":108908,"parent_record_offset":108712,"ancestor_record_offset":106788,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1606,"record_offset":108796,"parent_record_offset":108712,"ancestor_record_offset":106788,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1606,"record_offset":109316,"parent_record_offset":109292,"ancestor_record_offset":106788,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1606,"record_offset":108580,"parent_record_offset":108556,"ancestor_record_offset":106788,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1606,"record_offset":108604,"parent_record_offset":108580,"ancestor_record_offset":106788,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1606,"record_offset":109292,"parent_record_offset":109124,"ancestor_record_offset":106788,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1606,"record_offset":108636,"parent_record_offset":108604,"ancestor_record_offset":106788,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1606,"record_offset":109232,"parent_record_offset":109148,"ancestor_record_offset":106788,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1606,"record_offset":109176,"parent_record_offset":109148,"ancestor_record_offset":106788,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1606,"record_offset":109528,"parent_record_offset":108712,"ancestor_record_offset":106788,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1606,"record_offset":109148,"parent_record_offset":109124,"ancestor_record_offset":106788,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1606,"record_offset":108688,"parent_record_offset":108664,"ancestor_record_offset":106788,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1606,"record_offset":108664,"parent_record_offset":108604,"ancestor_record_offset":106788,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1606,"record_offset":108712,"parent_record_offset":108688,"ancestor_record_offset":106788,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1606,"record_offset":108768,"parent_record_offset":108712,"ancestor_record_offset":106788,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1606,"record_offset":109124,"parent_record_offset":108712,"ancestor_record_offset":106788,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1606,"record_offset":108556,"parent_record_offset":108524,"ancestor_record_offset":106788,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1606,"record_offset":108488,"parent_record_offset":108456,"ancestor_record_offset":106788,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::impl$6::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1606,"record_offset":107064,"parent_record_offset":107020,"ancestor_record_offset":106788,"inlinee":25619,"source_line":114,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 完整结构及原始类型记录: finish_edit_rectangle.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: finish_edit_rectangle.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: finish_edit_rectangle.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::wall::shapes::rectangle::WipRectangle> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::FinishEditRectangleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnRectangleEdited>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::wall::shapes::rectangle::WipRectangle> >", "bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::FinishEditRectangleCmd>", "bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnRectangleEdited>", "bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>", "bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::wall::shapes::rectangle::WipRectangle> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::FinishEditRectangleCmd>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnRectangleEdited>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::finish_edit_rectangle
// 信息 ID: dbb64da6f89a793ca853542e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0b99e539830a9f9e0fd59eff: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::wall::shapes::rectangle::WipRectangle> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::FinishEditRectangleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnRectangleEdited>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::wall::shapes::rectangle::WipRectangle> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::FinishEditRectangleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnRectangleEdited>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::wall::shapes::rectangle::WipRectangle> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::FinishEditRectangleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnRectangleEdited>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>)> > > @ 0x14f48f7 -> 0x1464a60 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::wall::shapes::rectangle::WipRectangle> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::FinishEditRectangleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnRectangleEdited>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::wall::shapes::rectangle::WipRectangle> >,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::shapes::rectangle::FinishEditRectangleCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::events::OnRectangleEdited>,bevy_ecs::event::writer::EventWriter<country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed>,bevy_ecs::event::writer::EventWriter<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>)> > @ 0x179dfe5 -> 0x1464a60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
