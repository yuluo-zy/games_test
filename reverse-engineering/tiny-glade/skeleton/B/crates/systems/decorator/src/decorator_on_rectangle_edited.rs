// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/decorator_on_rectangle_edited.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a6d152b74f270744315ff91459bb4657f5f412094da4ac3875f6296c019c4b8d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decorator_on_rectangle_edited.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 462
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
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":462,"record_offset":264,"rva":8494080,"va":5377203200}
// 编译实例 RVAs: 0x819c00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=1
//     target_aliases=532 candidates; complete list: decorator_on_rectangle_edited.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8251142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e8fa1882ff","size":5,"soft_fail":false,"target_va":5368953728,"va":5377204865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400885d0
//     owner=unresolved_namespace; count=1
//     target_aliases=12 candidates; complete list: decorator_on_rectangle_edited.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7936528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e8f0e586ff","size":5,"soft_fail":false,"target_va":5369267664,"va":5377204187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: decorator_on_rectangle_edited.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2527604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e88c6ed9ff","size":5,"soft_fail":false,"target_va":5374676688,"va":5377204287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1b200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_on_rectangle_edited.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2102041","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e819132000","size":5,"soft_fail":false,"target_va":5379305984,"va":5377203938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1e190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_on_rectangle_edited.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2113908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e874412000","size":5,"soft_fail":false,"target_va":5379318160,"va":5377204247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_on_rectangle_edited.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4730028","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e8ac2c4800","size":5,"soft_fail":false,"target_va":5381934560,"va":5377204527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412b4f10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_on_rectangle_edited.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11120958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e83eb1a900","size":5,"soft_fail":false,"target_va":5388324624,"va":5377203661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34336783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e80ff00b02","size":5,"soft_fail":false,"target_va":5411541808,"va":5377205020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34336715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e8cbef0b02","size":5,"soft_fail":false,"target_va":5411541776,"va":5377205056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34337015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e8f7f00b02","size":5,"soft_fail":false,"target_va":5411542064,"va":5377205044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorDst::as_wall_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t103735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e837950100","size":5,"soft_fail":false,"target_va":5377307968,"va":5377204228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t102775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e877910100","size":5,"soft_fail":false,"target_va":5377307232,"va":5377204452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3218976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e8201e3100","size":5,"soft_fail":false,"target_va":5380422656,"va":5377203675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2101727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e8df112000","size":5,"soft_fail":false,"target_va":5379305952,"va":5377204220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2115061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e8f5452000","size":5,"soft_fail":false,"target_va":5379318816,"va":5377203750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_wall_state::WallState::get_enclosure_params
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t702644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e8b4b80a00","size":5,"soft_fail":false,"target_va":5377906352,"va":5377203703,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::inner_walls::InnerWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t264053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e875070400","size":5,"soft_fail":false,"target_va":5377467744,"va":5377203686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3192301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e8edb53000","size":5,"soft_fail":false,"target_va":5380396784,"va":5377204478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4722343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e8a70e4800","size":5,"soft_fail":false,"target_va":5381926912,"va":5377204564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4715754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e8eaf44700","size":5,"soft_fail":false,"target_va":5381920256,"va":5377204497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4716268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e8ecf64700","size":5,"soft_fail":false,"target_va":5381920064,"va":5377203791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4715467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377203200,"raw_hex":"e8cbf34700","size":5,"soft_fail":false,"target_va":5381920048,"va":5377204576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":462,"record_offset":1476,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":462,"record_offset":1232,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":10715,"source_line":26,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":462,"record_offset":440,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_on_rectangle_edited::on_rectangle_edited
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_on_rectangle_edited.rs
//     evidence={"module":462,"record_offset":1592,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":25729,"source_line":35,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":264,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":462,"record_offset":800,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":462,"record_offset":1504,"parent_record_offset":1476,"ancestor_record_offset":264,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":462,"record_offset":528,"parent_record_offset":476,"ancestor_record_offset":264,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":462,"record_offset":564,"parent_record_offset":528,"ancestor_record_offset":264,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::last
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":462,"record_offset":1272,"parent_record_offset":1232,"ancestor_record_offset":264,"inlinee":10716,"source_line":112,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":462,"record_offset":476,"parent_record_offset":440,"ancestor_record_offset":264,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":462,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":264,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":462,"record_offset":2140,"parent_record_offset":2112,"ancestor_record_offset":264,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":462,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":264,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":462,"record_offset":2200,"parent_record_offset":2168,"ancestor_record_offset":264,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":462,"record_offset":3572,"parent_record_offset":3452,"ancestor_record_offset":264,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":462,"record_offset":3512,"parent_record_offset":3452,"ancestor_record_offset":264,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":462,"record_offset":3484,"parent_record_offset":3452,"ancestor_record_offset":264,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":462,"record_offset":3540,"parent_record_offset":3452,"ancestor_record_offset":264,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":462,"record_offset":3840,"parent_record_offset":1592,"ancestor_record_offset":264,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":462,"record_offset":3776,"parent_record_offset":3748,"ancestor_record_offset":264,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":462,"record_offset":3804,"parent_record_offset":3748,"ancestor_record_offset":264,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":462,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":264,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":462,"record_offset":3224,"parent_record_offset":2528,"ancestor_record_offset":264,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":462,"record_offset":1312,"parent_record_offset":1272,"ancestor_record_offset":264,"inlinee":10717,"source_line":176,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":462,"record_offset":752,"parent_record_offset":720,"ancestor_record_offset":264,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":462,"record_offset":628,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":462,"record_offset":2896,"parent_record_offset":2868,"ancestor_record_offset":264,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":462,"record_offset":2868,"parent_record_offset":2840,"ancestor_record_offset":264,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":462,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":264,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":462,"record_offset":3692,"parent_record_offset":3668,"ancestor_record_offset":264,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":462,"record_offset":1344,"parent_record_offset":1312,"ancestor_record_offset":264,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":462,"record_offset":1400,"parent_record_offset":1312,"ancestor_record_offset":264,"inlinee":10718,"source_line":1524,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":462,"record_offset":1784,"parent_record_offset":1592,"ancestor_record_offset":264,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":462,"record_offset":1432,"parent_record_offset":1272,"ancestor_record_offset":264,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":462,"record_offset":2084,"parent_record_offset":2048,"ancestor_record_offset":264,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":462,"record_offset":908,"parent_record_offset":884,"ancestor_record_offset":264,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":462,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":264,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":462,"record_offset":656,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":462,"record_offset":856,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":462,"record_offset":720,"parent_record_offset":688,"ancestor_record_offset":264,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":462,"record_offset":688,"parent_record_offset":656,"ancestor_record_offset":264,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":462,"record_offset":884,"parent_record_offset":856,"ancestor_record_offset":264,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":462,"record_offset":3180,"parent_record_offset":3156,"ancestor_record_offset":264,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_storage::Decorators::iter_maybe_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":462,"record_offset":1856,"parent_record_offset":1592,"ancestor_record_offset":264,"inlinee":25693,"source_line":217,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine2::Affine2::transform_point2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine2.rs
//     evidence={"module":462,"record_offset":3404,"parent_record_offset":1592,"ancestor_record_offset":264,"inlinee":25731,"source_line":255,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":462,"record_offset":3452,"parent_record_offset":3428,"ancestor_record_offset":264,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":462,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":264,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":462,"record_offset":3724,"parent_record_offset":1592,"ancestor_record_offset":264,"inlinee":10966,"source_line":210,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":462,"record_offset":3748,"parent_record_offset":3724,"ancestor_record_offset":264,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":462,"record_offset":3668,"parent_record_offset":1592,"ancestor_record_offset":264,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":462,"record_offset":3608,"parent_record_offset":3404,"ancestor_record_offset":264,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":462,"record_offset":2808,"parent_record_offset":2780,"ancestor_record_offset":264,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":462,"record_offset":2840,"parent_record_offset":2808,"ancestor_record_offset":264,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":462,"record_offset":2780,"parent_record_offset":2528,"ancestor_record_offset":264,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":462,"record_offset":2640,"parent_record_offset":2528,"ancestor_record_offset":264,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":462,"record_offset":2112,"parent_record_offset":2048,"ancestor_record_offset":264,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":462,"record_offset":3252,"parent_record_offset":2528,"ancestor_record_offset":264,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":462,"record_offset":2168,"parent_record_offset":2112,"ancestor_record_offset":264,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":462,"record_offset":2724,"parent_record_offset":2528,"ancestor_record_offset":264,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":462,"record_offset":2612,"parent_record_offset":2528,"ancestor_record_offset":264,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":462,"record_offset":3132,"parent_record_offset":3108,"ancestor_record_offset":264,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":462,"record_offset":2392,"parent_record_offset":2356,"ancestor_record_offset":264,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":462,"record_offset":2420,"parent_record_offset":2392,"ancestor_record_offset":264,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":462,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":264,"inlinee":13574,"source_line":799,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":462,"record_offset":3108,"parent_record_offset":2940,"ancestor_record_offset":264,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":462,"record_offset":2452,"parent_record_offset":2420,"ancestor_record_offset":264,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":3048,"parent_record_offset":2964,"ancestor_record_offset":264,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2992,"parent_record_offset":2964,"ancestor_record_offset":264,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":3344,"parent_record_offset":2528,"ancestor_record_offset":264,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2048,"parent_record_offset":1960,"ancestor_record_offset":264,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2964,"parent_record_offset":2940,"ancestor_record_offset":264,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2504,"parent_record_offset":2480,"ancestor_record_offset":264,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2480,"parent_record_offset":2420,"ancestor_record_offset":264,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":1936,"parent_record_offset":1908,"ancestor_record_offset":264,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2020,"parent_record_offset":1960,"ancestor_record_offset":264,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":1992,"parent_record_offset":1960,"ancestor_record_offset":264,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2528,"parent_record_offset":2504,"ancestor_record_offset":264,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":264,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2584,"parent_record_offset":2528,"ancestor_record_offset":264,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2940,"parent_record_offset":2528,"ancestor_record_offset":264,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":462,"record_offset":2356,"parent_record_offset":2324,"ancestor_record_offset":264,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":462,"record_offset":1884,"parent_record_offset":1856,"ancestor_record_offset":264,"inlinee":13573,"source_line":570,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":462,"record_offset":2324,"parent_record_offset":1592,"ancestor_record_offset":264,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_on_rectangle_edited::on_rectangle_edited
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6481
// 首条位置: {"module":462,"inlinee":25729}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":462,"record_offset":3840,"parent_record_offset":1592,"ancestor_record_offset":1592,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":462,"record_offset":1784,"parent_record_offset":1592,"ancestor_record_offset":1592,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_storage::Decorators::iter_maybe_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_storage.rs
//     evidence={"module":462,"record_offset":1856,"parent_record_offset":1592,"ancestor_record_offset":1592,"inlinee":25693,"source_line":217,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::affine2::Affine2::transform_point2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine2.rs
//     evidence={"module":462,"record_offset":3404,"parent_record_offset":1592,"ancestor_record_offset":1592,"inlinee":25731,"source_line":255,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":462,"record_offset":3724,"parent_record_offset":1592,"ancestor_record_offset":1592,"inlinee":10966,"source_line":210,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":462,"record_offset":3668,"parent_record_offset":1592,"ancestor_record_offset":1592,"inlinee":10956,"source_line":1013,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":462,"record_offset":2324,"parent_record_offset":1592,"ancestor_record_offset":1592,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":462,"record_offset":3156,"parent_record_offset":3132,"ancestor_record_offset":1592,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":462,"record_offset":2140,"parent_record_offset":2112,"ancestor_record_offset":1592,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":462,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":1592,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":462,"record_offset":2200,"parent_record_offset":2168,"ancestor_record_offset":1592,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":462,"record_offset":3572,"parent_record_offset":3452,"ancestor_record_offset":1592,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":462,"record_offset":3512,"parent_record_offset":3452,"ancestor_record_offset":1592,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":462,"record_offset":3484,"parent_record_offset":3452,"ancestor_record_offset":1592,"inlinee":25736,"source_line":948,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":462,"record_offset":3540,"parent_record_offset":3452,"ancestor_record_offset":1592,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":462,"record_offset":3776,"parent_record_offset":3748,"ancestor_record_offset":1592,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":462,"record_offset":3804,"parent_record_offset":3748,"ancestor_record_offset":1592,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":462,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":1592,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":462,"record_offset":3224,"parent_record_offset":2528,"ancestor_record_offset":1592,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":462,"record_offset":2896,"parent_record_offset":2868,"ancestor_record_offset":1592,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":462,"record_offset":2868,"parent_record_offset":2840,"ancestor_record_offset":1592,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":462,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":1592,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":462,"record_offset":3692,"parent_record_offset":3668,"ancestor_record_offset":1592,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":462,"record_offset":2084,"parent_record_offset":2048,"ancestor_record_offset":1592,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":462,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":1592,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":462,"record_offset":3180,"parent_record_offset":3156,"ancestor_record_offset":1592,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":462,"record_offset":3452,"parent_record_offset":3428,"ancestor_record_offset":1592,"inlinee":25735,"source_line":318,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat2.rs
//     evidence={"module":462,"record_offset":3428,"parent_record_offset":3404,"ancestor_record_offset":1592,"inlinee":25733,"source_line":472,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":462,"record_offset":3748,"parent_record_offset":3724,"ancestor_record_offset":1592,"inlinee":10960,"source_line":194,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":462,"record_offset":3608,"parent_record_offset":3404,"ancestor_record_offset":1592,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":462,"record_offset":2808,"parent_record_offset":2780,"ancestor_record_offset":1592,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":462,"record_offset":2840,"parent_record_offset":2808,"ancestor_record_offset":1592,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":462,"record_offset":2780,"parent_record_offset":2528,"ancestor_record_offset":1592,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":462,"record_offset":2640,"parent_record_offset":2528,"ancestor_record_offset":1592,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":462,"record_offset":2112,"parent_record_offset":2048,"ancestor_record_offset":1592,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":462,"record_offset":3252,"parent_record_offset":2528,"ancestor_record_offset":1592,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":462,"record_offset":2168,"parent_record_offset":2112,"ancestor_record_offset":1592,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":462,"record_offset":2724,"parent_record_offset":2528,"ancestor_record_offset":1592,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":462,"record_offset":2612,"parent_record_offset":2528,"ancestor_record_offset":1592,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":462,"record_offset":3132,"parent_record_offset":3108,"ancestor_record_offset":1592,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":462,"record_offset":2392,"parent_record_offset":2356,"ancestor_record_offset":1592,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":462,"record_offset":2420,"parent_record_offset":2392,"ancestor_record_offset":1592,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":462,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":1592,"inlinee":13574,"source_line":799,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":462,"record_offset":3108,"parent_record_offset":2940,"ancestor_record_offset":1592,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":462,"record_offset":2452,"parent_record_offset":2420,"ancestor_record_offset":1592,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":3048,"parent_record_offset":2964,"ancestor_record_offset":1592,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2992,"parent_record_offset":2964,"ancestor_record_offset":1592,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":3344,"parent_record_offset":2528,"ancestor_record_offset":1592,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2048,"parent_record_offset":1960,"ancestor_record_offset":1592,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2964,"parent_record_offset":2940,"ancestor_record_offset":1592,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2504,"parent_record_offset":2480,"ancestor_record_offset":1592,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2480,"parent_record_offset":2420,"ancestor_record_offset":1592,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":1936,"parent_record_offset":1908,"ancestor_record_offset":1592,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2020,"parent_record_offset":1960,"ancestor_record_offset":1592,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":1992,"parent_record_offset":1960,"ancestor_record_offset":1592,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2528,"parent_record_offset":2504,"ancestor_record_offset":1592,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":1592,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2584,"parent_record_offset":2528,"ancestor_record_offset":1592,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":462,"record_offset":2940,"parent_record_offset":2528,"ancestor_record_offset":1592,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":462,"record_offset":2356,"parent_record_offset":2324,"ancestor_record_offset":1592,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00462.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":462,"record_offset":1884,"parent_record_offset":1856,"ancestor_record_offset":1592,"inlinee":13573,"source_line":570,"archive":"_pdb/B/modules/00462.jsonl.gz"}
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
// 完整结构及原始类型记录: decorator_on_rectangle_edited.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: decorator_on_rectangle_edited.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: decorator_on_rectangle_edited.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "country_core::resources::walls::decorator_archivist::DecoratorArchivalResources"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited
// 信息 ID: 1d1aa558dd595ae761be7c53
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4f851667bdc57db3cd8b630f: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources)> > @ 0x47715 -> 0x819c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::reader::EventReader<country_core::resources::events::OnRectangleEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources)> > > @ 0x46a9bd -> 0x819c00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_on_rectangle_edited::on_rectangle_edited
// 信息 ID: d4420e1578edd7e75eb24766
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_on_rectangle_edited::decorator_on_rectangle_edited; module=462, inline_record=1592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
