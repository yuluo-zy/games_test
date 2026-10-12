// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/timberframe/src/timberframe_circle.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: a3ecbf125c8bdd9831953383c292ec57b40ca6ed2ec19ed3b588c42f1ebac233
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: timberframe_circle.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1959, 3215, 3217
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
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use std; // namespace candidate
//   package identities: []
// use timberframe; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: timberframe::timberframe_circle::construct_timberframe_circle
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3217,"record_offset":248,"rva":26803968,"va":5395513088}
// 编译实例 RVAs: 0x198ff00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22766239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e8619da4fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5395515946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18570907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e865a1e4fe","size":5,"soft_fail":false,"target_va":5376943968,"va":5395514870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18570145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e85fa4e4fe","size":5,"soft_fail":false,"target_va":5376944000,"va":5395514140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8d2a0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13644075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e8d5ce2fff","size":5,"soft_fail":false,"target_va":5381870240,"va":5395514310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14198f130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e8a4f1ffff","size":5,"soft_fail":false,"target_va":5395509552,"va":5395513223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14198f400
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e8f1efffff","size":5,"soft_fail":false,"target_va":5395510272,"va":5395514378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14198f9c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e866f3ffff","size":5,"soft_fail":false,"target_va":5395511744,"va":5395514965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: timberframe_circle.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8080046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e8ae4a7b00","size":5,"soft_fail":false,"target_va":5403593344,"va":5395513293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t7461071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e8cfd87100","size":5,"soft_fail":false,"target_va":5402978880,"va":5395517804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7811355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e81b317700","size":5,"soft_fail":false,"target_va":5403329152,"va":5395517792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t7461938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e832dc7100","size":5,"soft_fail":false,"target_va":5402979152,"va":5395517209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t7461617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e8f1da7100","size":5,"soft_fail":false,"target_va":5402978944,"va":5395517322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::roof_height_ws_from_wall_shape
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19750011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e885a3d2fe","size":5,"soft_fail":false,"target_va":5375765952,"va":5395515958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::split_height_into_skirts
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20693025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e8df3fc4fe","size":5,"soft_fail":false,"target_va":5374821408,"va":5395514428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18163288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e8a8d9eafe","size":5,"soft_fail":false,"target_va":5377353776,"va":5395517059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7455588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e864c37100","size":5,"soft_fail":false,"target_va":5402968992,"va":5395513399,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7453612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e8acbb7100","size":5,"soft_fail":false,"target_va":5402970800,"va":5395517183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_points2_ptcount
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-17544903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e83949f4fe","size":5,"soft_fail":false,"target_va":5377969024,"va":5395513922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-17546190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e83244f4fe","size":5,"soft_fail":false,"target_va":5377967920,"va":5395514105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17545351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e87947f4fe","size":5,"soft_fail":false,"target_va":5377968464,"va":5395513810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-18574145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395513088,"raw_hex":"e8bf94e4fe","size":5,"soft_fail":false,"target_va":5376941312,"va":5395515452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5395513088,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5395513313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3217,"record_offset":8632,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3217,"record_offset":1336,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3217,"record_offset":1884,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3217,"record_offset":8772,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3217,"record_offset":3128,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3217,"record_offset":8800,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3217,"record_offset":3268,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3217,"record_offset":2268,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":3217,"record_offset":2240,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3217,"record_offset":3520,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3217,"record_offset":784,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3217,"record_offset":748,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3217,"record_offset":6364,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3217,"record_offset":3184,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3217,"record_offset":508,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3217,"record_offset":848,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14713,"source_line":158,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3217,"record_offset":5292,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3217,"record_offset":5204,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3217,"record_offset":6124,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3217,"record_offset":7392,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3217,"record_offset":4352,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3217,"record_offset":7304,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3217,"record_offset":5116,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":3217,"record_offset":3576,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":3217,"record_offset":4408,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":3217,"record_offset":8828,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":3217,"record_offset":8688,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] timberframe::timberframe_circle::construct_timberframe_circle::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/timberframe/src/timberframe_circle.rs
//     evidence={"module":3217,"record_offset":3156,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":101599,"source_line":38,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3217,"record_offset":2724,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":23062,"source_line":28,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3217,"record_offset":6528,"parent_record_offset":6504,"ancestor_record_offset":248,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3217,"record_offset":6504,"parent_record_offset":6412,"ancestor_record_offset":248,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3217,"record_offset":6440,"parent_record_offset":6412,"ancestor_record_offset":248,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3217,"record_offset":6412,"parent_record_offset":6388,"ancestor_record_offset":248,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3217,"record_offset":8656,"parent_record_offset":8632,"ancestor_record_offset":248,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3217,"record_offset":1424,"parent_record_offset":1400,"ancestor_record_offset":248,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3217,"record_offset":1400,"parent_record_offset":1360,"ancestor_record_offset":248,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3217,"record_offset":1916,"parent_record_offset":1884,"ancestor_record_offset":248,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3217,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":248,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3217,"record_offset":2584,"parent_record_offset":2560,"ancestor_record_offset":248,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3217,"record_offset":2608,"parent_record_offset":2584,"ancestor_record_offset":248,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3217,"record_offset":2360,"parent_record_offset":2332,"ancestor_record_offset":248,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3217,"record_offset":576,"parent_record_offset":536,"ancestor_record_offset":248,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3217,"record_offset":3036,"parent_record_offset":2748,"ancestor_record_offset":248,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3217,"record_offset":2836,"parent_record_offset":2784,"ancestor_record_offset":248,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3217,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":248,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3217,"record_offset":2648,"parent_record_offset":2500,"ancestor_record_offset":248,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":3217,"record_offset":2676,"parent_record_offset":2500,"ancestor_record_offset":248,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3217,"record_offset":4312,"parent_record_offset":4288,"ancestor_record_offset":248,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3217,"record_offset":1988,"parent_record_offset":1956,"ancestor_record_offset":248,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3217,"record_offset":1956,"parent_record_offset":1916,"ancestor_record_offset":248,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3217,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":248,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3217,"record_offset":3544,"parent_record_offset":3520,"ancestor_record_offset":248,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":3217,"record_offset":3652,"parent_record_offset":3608,"ancestor_record_offset":248,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3217,"record_offset":3796,"parent_record_offset":3740,"ancestor_record_offset":248,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3217,"record_offset":2332,"parent_record_offset":2304,"ancestor_record_offset":248,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":3217,"record_offset":2304,"parent_record_offset":2268,"ancestor_record_offset":248,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3217,"record_offset":4288,"parent_record_offset":3608,"ancestor_record_offset":248,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3217,"record_offset":4956,"parent_record_offset":4928,"ancestor_record_offset":248,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3217,"record_offset":1232,"parent_record_offset":1204,"ancestor_record_offset":248,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3217,"record_offset":1256,"parent_record_offset":1232,"ancestor_record_offset":248,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3217,"record_offset":1472,"parent_record_offset":1424,"ancestor_record_offset":248,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3217,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":248,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3217,"record_offset":6468,"parent_record_offset":6440,"ancestor_record_offset":248,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3217,"record_offset":5448,"parent_record_offset":5316,"ancestor_record_offset":248,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3217,"record_offset":4376,"parent_record_offset":4352,"ancestor_record_offset":248,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3217,"record_offset":5144,"parent_record_offset":5116,"ancestor_record_offset":248,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3217,"record_offset":5232,"parent_record_offset":5204,"ancestor_record_offset":248,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":3217,"record_offset":2432,"parent_record_offset":2400,"ancestor_record_offset":248,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3217,"record_offset":2560,"parent_record_offset":2536,"ancestor_record_offset":248,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3217,"record_offset":2536,"parent_record_offset":2500,"ancestor_record_offset":248,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3217,"record_offset":2020,"parent_record_offset":1988,"ancestor_record_offset":248,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3217,"record_offset":1136,"parent_record_offset":848,"ancestor_record_offset":248,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3217,"record_offset":2400,"parent_record_offset":2268,"ancestor_record_offset":248,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3217,"record_offset":2060,"parent_record_offset":1956,"ancestor_record_offset":248,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3217,"record_offset":920,"parent_record_offset":848,"ancestor_record_offset":248,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3217,"record_offset":1176,"parent_record_offset":848,"ancestor_record_offset":248,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3217,"record_offset":6388,"parent_record_offset":6364,"ancestor_record_offset":248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3217,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3217,"record_offset":2184,"parent_record_offset":2160,"ancestor_record_offset":248,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3217,"record_offset":4984,"parent_record_offset":4956,"ancestor_record_offset":248,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3217,"record_offset":1044,"parent_record_offset":956,"ancestor_record_offset":248,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3217,"record_offset":1068,"parent_record_offset":1044,"ancestor_record_offset":248,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3217,"record_offset":660,"parent_record_offset":632,"ancestor_record_offset":248,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3217,"record_offset":688,"parent_record_offset":660,"ancestor_record_offset":248,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3217,"record_offset":1536,"parent_record_offset":1424,"ancestor_record_offset":248,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3217,"record_offset":1728,"parent_record_offset":1424,"ancestor_record_offset":248,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3217,"record_offset":1104,"parent_record_offset":848,"ancestor_record_offset":248,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3217,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":248,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3217,"record_offset":1576,"parent_record_offset":1536,"ancestor_record_offset":248,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3217,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":248,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3217,"record_offset":2160,"parent_record_offset":2136,"ancestor_record_offset":248,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3217,"record_offset":2136,"parent_record_offset":2084,"ancestor_record_offset":248,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3217,"record_offset":604,"parent_record_offset":576,"ancestor_record_offset":248,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3217,"record_offset":8944,"parent_record_offset":8912,"ancestor_record_offset":248,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3217,"record_offset":988,"parent_record_offset":956,"ancestor_record_offset":248,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3217,"record_offset":8712,"parent_record_offset":8688,"ancestor_record_offset":248,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3217,"record_offset":8968,"parent_record_offset":8944,"ancestor_record_offset":248,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3217,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":248,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":3217,"record_offset":8736,"parent_record_offset":8712,"ancestor_record_offset":248,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3217,"record_offset":536,"parent_record_offset":508,"ancestor_record_offset":248,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3217,"record_offset":1204,"parent_record_offset":1176,"ancestor_record_offset":248,"inlinee":14715,"source_line":175,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":3217,"record_offset":8884,"parent_record_offset":8856,"ancestor_record_offset":248,"inlinee":14717,"source_line":435,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":3217,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":248,"inlinee":23063,"source_line":231,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":3217,"record_offset":5596,"parent_record_offset":5540,"ancestor_record_offset":248,"inlinee":23064,"source_line":168,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3217,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":248,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3217,"record_offset":2784,"parent_record_offset":2748,"ancestor_record_offset":248,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3217,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":248,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3217,"record_offset":5380,"parent_record_offset":5348,"ancestor_record_offset":248,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3217,"record_offset":5348,"parent_record_offset":5316,"ancestor_record_offset":248,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3217,"record_offset":5316,"parent_record_offset":5292,"ancestor_record_offset":248,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3217,"record_offset":6236,"parent_record_offset":6148,"ancestor_record_offset":248,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3217,"record_offset":7456,"parent_record_offset":7420,"ancestor_record_offset":248,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3217,"record_offset":7420,"parent_record_offset":7392,"ancestor_record_offset":248,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3217,"record_offset":6148,"parent_record_offset":6124,"ancestor_record_offset":248,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$26::left_shift_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":3217,"record_offset":4928,"parent_record_offset":4408,"ancestor_record_offset":248,"inlinee":8082,"source_line":318,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":3217,"record_offset":3608,"parent_record_offset":3576,"ancestor_record_offset":248,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":3217,"record_offset":956,"parent_record_offset":848,"ancestor_record_offset":248,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":3217,"record_offset":8912,"parent_record_offset":8884,"ancestor_record_offset":248,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3217,"record_offset":5412,"parent_record_offset":5380,"ancestor_record_offset":248,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] timberframe::timberframe_circle::construct_timberframe_circle::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/timberframe/src/timberframe_circle.rs
//     evidence={"module":3217,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":248,"inlinee":101592,"source_line":80,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] timberframe::timberframe_circle::construct_timberframe_circle::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/timberframe/src/timberframe_circle.rs
//     evidence={"module":3217,"record_offset":7264,"parent_record_offset":7240,"ancestor_record_offset":248,"inlinee":77263,"source_line":104,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":3217,"record_offset":2500,"parent_record_offset":2464,"ancestor_record_offset":248,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::circle::impl$0::as_points2_ptcount::closure$0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/circle.rs
//     evidence={"module":3217,"record_offset":2464,"parent_record_offset":2432,"ancestor_record_offset":248,"inlinee":61743,"source_line":163,"archive":"_pdb/A/modules/03217.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: timberframe::timberframe_circle::construct_timberframe_circle::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18CDF
// 首条位置: {"module":3217,"inlinee":101599}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: timberframe::timberframe_circle::construct_timberframe_circle::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18CD8
// 首条位置: {"module":3215,"inlinee":101592}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: timberframe::timberframe_circle::construct_timberframe_circle::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12DCF
// 首条位置: {"module":1959,"inlinee":77263}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: timberframe_circle.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: timberframe_circle.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: timberframe_circle.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>", "bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>", "bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>

// ------------------------------------------------------------------
// 函数: timberframe::timberframe_circle::construct_timberframe_circle
// 信息 ID: a5aafda04fe0dc9f75f0946e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] dab57860fd81109c8fc13195: void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>),void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>)> > @ 0x12ed931 -> 0x198ff00 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>),void (*)(bevy_ecs::event::EventReader<country_core::resources::events::ConstructTimberframeCmd>,bevy_ecs::change_detection::Res<timberframe::TimberframeInstanceBuffer>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>,bevy_ecs::change_detection::ResMut<timberframe::timberframe_beam_count_change_audio::TimberframeBeamCount>)> @ 0x12edf5f -> 0x198ff00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: timberframe::timberframe_circle::construct_timberframe_circle::closure$0
// 信息 ID: 07d3331f21005a86196fd2be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "timberframe::timberframe_circle::construct_timberframe_circle", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] timberframe::timberframe_circle::construct_timberframe_circle; module=3217, inline_record=3156; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_circle::construct_timberframe_circle; module=3217, inline_record=6336; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_circle::construct_timberframe_circle; module=3217, inline_record=8604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: timberframe::timberframe_circle::construct_timberframe_circle::closure$1
// 信息 ID: 137d383ac67ef95a233d300c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "timberframe::timberframe_circle::construct_timberframe_circle", "captures": "unknown"}
// [闭包环境类型引用] timberframe::timberframe_circle::construct_timberframe_circle::closure_env$1 <- itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,timberframe::timberframe_circle::construct_timberframe_circle::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,timberframe::timberframe_circle::construct_timberframe_circle::closure_env$1> > > >; module=3215, inline_record=3372; 不是独立调用边。
// [PDB 内联宿主] itertools::tuple_impl::impl$26::collect_from_iter_no_buf<glam::f32::vec3::Vec3,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<glam::f32::vec3::Vec3>,core::iter::sources::once::Once<glam::f32::vec3::Vec3> >,ref_mut$<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,timberframe::timberframe_circle::construct_timberframe_circle::closure_env$1> > > >; module=3215, inline_record=4584; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_circle::construct_timberframe_circle; module=3217, inline_record=4248; 不是独立调用边。
// [PDB 内联宿主] timberframe::timberframe_circle::construct_timberframe_circle; module=3217, inline_record=4888; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: timberframe::timberframe_circle::construct_timberframe_circle::closure$2
// 信息 ID: 7bb749236bb5639e22e12858
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "timberframe::timberframe_circle::construct_timberframe_circle", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] timberframe::timberframe_circle::construct_timberframe_circle; module=3217, inline_record=7264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
