// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f133f6e92527e7e77dad820caef916f4b5650484bd5fd67b4df0f2efde6d01e4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: split_wall_path_segment_into_platform_stairs.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 362, 1226, 1234, 1264
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":362,"record_offset":260,"rva":5094320,"va":5373803440}
// 编译实例 RVAs: 0x4dbbb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4191436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e8340bc0ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373804951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=20
//     target_aliases=1 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1055127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e869e6efff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373804834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408cb310
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4126308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e864f63e00","size":5,"soft_fail":false,"target_va":5377930000,"va":5373803687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8082194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e812537b00","size":5,"soft_fail":false,"target_va":5381888304,"va":5373806105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c971d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8103320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e898a57b00","size":5,"soft_fail":false,"target_va":5381910992,"va":5373807667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c973d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8103657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e8e9a67b00","size":5,"soft_fail":false,"target_va":5381911504,"va":5373807842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c974a0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8103311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e88fa57b00","size":5,"soft_fail":false,"target_va":5381911712,"va":5373808396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97760
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8103433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e809a67b00","size":5,"soft_fail":false,"target_va":5381912416,"va":5373808978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c977d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8103774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e85ea77b00","size":5,"soft_fail":false,"target_va":5381912528,"va":5373808749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca9b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8181131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e88bd57c00","size":5,"soft_fail":false,"target_va":5381987216,"va":5373806080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140caa570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8183055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e80fdd7c00","size":5,"soft_fail":false,"target_va":5381989744,"va":5373806684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cabba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8188931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e803f47c00","size":5,"soft_fail":false,"target_va":5381995424,"va":5373806488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb1700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8214390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e876577d00","size":5,"soft_fail":false,"target_va":5382018816,"va":5373804421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cbbc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8254342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e886f37d00","size":5,"soft_fail":false,"target_va":5382061152,"va":5373806805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420c12e0
//     owner=unresolved_namespace; count=1
//     target_aliases=24 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29247483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e8fb47be01","size":5,"soft_fail":false,"target_va":5403054816,"va":5373807328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: split_wall_path_segment_into_platform_stairs.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29786399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e81f81c601","size":5,"soft_fail":false,"target_va":5403593360,"va":5373806956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t29115915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e80b46bc01","size":5,"soft_fail":false,"target_va":5402919776,"va":5373803856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2500138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e8d6d9d9ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5373805189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29170666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e8ea1bbd01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373808209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1957660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e81cdf1d00","size":5,"soft_fail":false,"target_va":5375763552,"va":5373805887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1629837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e88dde1800","size":5,"soft_fail":false,"target_va":5375434976,"va":5373805134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1630351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e88fe01800","size":5,"soft_fail":false,"target_va":5375436032,"va":5373805676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8236158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373803440,"raw_hex":"e87eac7d00","size":5,"soft_fail":false,"target_va":5382043936,"va":5373807773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":9040,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":4652,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":12918,"source_line":2673,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":10932,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":6512,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":4964,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":1732,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":362,"record_offset":5020,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":362,"record_offset":5448,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":362,"record_offset":564,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":362,"record_offset":1140,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":362,"record_offset":12656,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":362,"record_offset":9868,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":362,"record_offset":10876,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":362,"record_offset":6420,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":362,"record_offset":5156,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":362,"record_offset":10988,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":362,"record_offset":4560,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21341,"source_line":2912,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":362,"record_offset":12580,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":362,"record_offset":9840,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":362,"record_offset":7732,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6546,"source_line":2798,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":362,"record_offset":6688,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":362,"record_offset":5504,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":362,"record_offset":11128,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":362,"record_offset":5392,"parent_record_offset":5368,"ancestor_record_offset":260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":362,"record_offset":5368,"parent_record_offset":5276,"ancestor_record_offset":260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":362,"record_offset":13396,"parent_record_offset":13372,"ancestor_record_offset":260,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":362,"record_offset":5304,"parent_record_offset":5276,"ancestor_record_offset":260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":362,"record_offset":13372,"parent_record_offset":13344,"ancestor_record_offset":260,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":362,"record_offset":2380,"parent_record_offset":2348,"ancestor_record_offset":260,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":362,"record_offset":2316,"parent_record_offset":2292,"ancestor_record_offset":260,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":362,"record_offset":5276,"parent_record_offset":5252,"ancestor_record_offset":260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":13284,"parent_record_offset":13252,"ancestor_record_offset":260,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":2348,"parent_record_offset":1780,"ancestor_record_offset":260,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":9072,"parent_record_offset":9040,"ancestor_record_offset":260,"inlinee":4126,"source_line":1244,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":4676,"parent_record_offset":4652,"ancestor_record_offset":260,"inlinee":8654,"source_line":1815,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":260,"inlinee":8573,"source_line":2980,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":2836,"parent_record_offset":1780,"ancestor_record_offset":260,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":2292,"parent_record_offset":1780,"ancestor_record_offset":260,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":2412,"parent_record_offset":1780,"ancestor_record_offset":260,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":1164,"parent_record_offset":1140,"ancestor_record_offset":260,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":13192,"parent_record_offset":13160,"ancestor_record_offset":260,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":11204,"parent_record_offset":11180,"ancestor_record_offset":260,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$5::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":362,"record_offset":4732,"parent_record_offset":4676,"ancestor_record_offset":260,"inlinee":8528,"source_line":2674,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":362,"record_offset":5180,"parent_record_offset":5156,"ancestor_record_offset":260,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":362,"record_offset":5228,"parent_record_offset":5204,"ancestor_record_offset":260,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":362,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":260,"inlinee":8592,"source_line":16,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":362,"record_offset":13252,"parent_record_offset":13160,"ancestor_record_offset":260,"inlinee":4434,"source_line":31,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":362,"record_offset":10132,"parent_record_offset":9988,"ancestor_record_offset":260,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=5
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":362,"record_offset":5472,"parent_record_offset":5448,"ancestor_record_offset":260,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":362,"record_offset":9988,"parent_record_offset":9944,"ancestor_record_offset":260,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":362,"record_offset":676,"parent_record_offset":628,"ancestor_record_offset":260,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":362,"record_offset":628,"parent_record_offset":588,"ancestor_record_offset":260,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":362,"record_offset":2464,"parent_record_offset":2440,"ancestor_record_offset":260,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$20::size_hint
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":362,"record_offset":1856,"parent_record_offset":1828,"ancestor_record_offset":260,"inlinee":21339,"source_line":774,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":362,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":260,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::size_hint
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":362,"record_offset":1884,"parent_record_offset":1856,"ancestor_record_offset":260,"inlinee":21340,"source_line":858,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":362,"record_offset":10776,"parent_record_offset":9944,"ancestor_record_offset":260,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":362,"record_offset":10408,"parent_record_offset":9944,"ancestor_record_offset":260,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":362,"record_offset":10432,"parent_record_offset":10408,"ancestor_record_offset":260,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":362,"record_offset":9944,"parent_record_offset":9920,"ancestor_record_offset":260,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":362,"record_offset":10736,"parent_record_offset":10712,"ancestor_record_offset":260,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":362,"record_offset":10508,"parent_record_offset":10484,"ancestor_record_offset":260,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":362,"record_offset":10484,"parent_record_offset":10460,"ancestor_record_offset":260,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":362,"record_offset":10556,"parent_record_offset":10532,"ancestor_record_offset":260,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":362,"record_offset":10644,"parent_record_offset":10620,"ancestor_record_offset":260,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":362,"record_offset":10712,"parent_record_offset":10432,"ancestor_record_offset":260,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":362,"record_offset":10460,"parent_record_offset":10432,"ancestor_record_offset":260,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":362,"record_offset":10380,"parent_record_offset":9944,"ancestor_record_offset":260,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":362,"record_offset":9920,"parent_record_offset":9896,"ancestor_record_offset":260,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":362,"record_offset":588,"parent_record_offset":564,"ancestor_record_offset":260,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":362,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":260,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":362,"record_offset":10196,"parent_record_offset":10164,"ancestor_record_offset":260,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":362,"record_offset":4756,"parent_record_offset":4732,"ancestor_record_offset":260,"inlinee":8530,"source_line":1709,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":362,"record_offset":10100,"parent_record_offset":10076,"ancestor_record_offset":260,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":362,"record_offset":6204,"parent_record_offset":6180,"ancestor_record_offset":260,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":362,"record_offset":8460,"parent_record_offset":8436,"ancestor_record_offset":260,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":362,"record_offset":5756,"parent_record_offset":5732,"ancestor_record_offset":260,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":362,"record_offset":5840,"parent_record_offset":5816,"ancestor_record_offset":260,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":362,"record_offset":4900,"parent_record_offset":4676,"ancestor_record_offset":260,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":362,"record_offset":2540,"parent_record_offset":2516,"ancestor_record_offset":260,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":362,"record_offset":2516,"parent_record_offset":2488,"ancestor_record_offset":260,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":362,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":260,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":362,"record_offset":1912,"parent_record_offset":1884,"ancestor_record_offset":260,"inlinee":8623,"source_line":151,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":362,"record_offset":12680,"parent_record_offset":12656,"ancestor_record_offset":260,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":362,"record_offset":2440,"parent_record_offset":1780,"ancestor_record_offset":260,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":362,"record_offset":1828,"parent_record_offset":1780,"ancestor_record_offset":260,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":362,"record_offset":12760,"parent_record_offset":12736,"ancestor_record_offset":260,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":362,"record_offset":15548,"parent_record_offset":15520,"ancestor_record_offset":260,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::leading_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":362,"record_offset":4612,"parent_record_offset":4584,"ancestor_record_offset":260,"inlinee":17359,"source_line":148,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":362,"record_offset":2212,"parent_record_offset":1912,"ancestor_record_offset":260,"inlinee":4980,"source_line":1499,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":362,"record_offset":728,"parent_record_offset":676,"ancestor_record_offset":260,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":362,"record_offset":5332,"parent_record_offset":5304,"ancestor_record_offset":260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":362,"record_offset":8692,"parent_record_offset":8664,"ancestor_record_offset":260,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":362,"record_offset":5952,"parent_record_offset":5928,"ancestor_record_offset":260,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":362,"record_offset":10076,"parent_record_offset":9988,"ancestor_record_offset":260,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":362,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":260,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":362,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":260,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":362,"record_offset":10580,"parent_record_offset":10556,"ancestor_record_offset":260,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":362,"record_offset":2776,"parent_record_offset":2440,"ancestor_record_offset":260,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":362,"record_offset":2596,"parent_record_offset":2516,"ancestor_record_offset":260,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":362,"record_offset":2008,"parent_record_offset":1984,"ancestor_record_offset":260,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=43
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":362,"record_offset":5204,"parent_record_offset":5180,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":362,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":260,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":362,"record_offset":6116,"parent_record_offset":6092,"ancestor_record_offset":260,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":362,"record_offset":6092,"parent_record_offset":6068,"ancestor_record_offset":260,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":362,"record_offset":2720,"parent_record_offset":2696,"ancestor_record_offset":260,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":362,"record_offset":5096,"parent_record_offset":5020,"ancestor_record_offset":260,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":362,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":260,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":362,"record_offset":5060,"parent_record_offset":5020,"ancestor_record_offset":260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":362,"record_offset":5120,"parent_record_offset":5096,"ancestor_record_offset":260,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":362,"record_offset":6600,"parent_record_offset":6512,"ancestor_record_offset":260,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":362,"record_offset":10352,"parent_record_offset":9944,"ancestor_record_offset":260,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":362,"record_offset":792,"parent_record_offset":676,"ancestor_record_offset":260,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":362,"record_offset":980,"parent_record_offset":676,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":362,"record_offset":10044,"parent_record_offset":10020,"ancestor_record_offset":260,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":362,"record_offset":852,"parent_record_offset":828,"ancestor_record_offset":260,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":362,"record_offset":828,"parent_record_offset":792,"ancestor_record_offset":260,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":362,"record_offset":1008,"parent_record_offset":980,"ancestor_record_offset":260,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":362,"record_offset":2696,"parent_record_offset":2672,"ancestor_record_offset":260,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":362,"record_offset":2672,"parent_record_offset":2620,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":362,"record_offset":1960,"parent_record_offset":1912,"ancestor_record_offset":260,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::quicksort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\sort.rs
//     evidence={"module":362,"record_offset":4584,"parent_record_offset":4560,"ancestor_record_offset":260,"inlinee":17358,"source_line":892,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":362,"record_offset":4780,"parent_record_offset":4756,"ancestor_record_offset":260,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":362,"record_offset":10164,"parent_record_offset":9944,"ancestor_record_offset":260,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":362,"record_offset":10248,"parent_record_offset":10224,"ancestor_record_offset":260,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":362,"record_offset":10272,"parent_record_offset":10248,"ancestor_record_offset":260,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":362,"record_offset":10224,"parent_record_offset":10196,"ancestor_record_offset":260,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":362,"record_offset":6180,"parent_record_offset":6156,"ancestor_record_offset":260,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":362,"record_offset":5552,"parent_record_offset":5528,"ancestor_record_offset":260,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":362,"record_offset":5528,"parent_record_offset":5504,"ancestor_record_offset":260,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":362,"record_offset":7656,"parent_record_offset":6724,"ancestor_record_offset":260,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":362,"record_offset":6724,"parent_record_offset":6688,"ancestor_record_offset":260,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":362,"record_offset":6156,"parent_record_offset":6020,"ancestor_record_offset":260,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":362,"record_offset":5584,"parent_record_offset":5552,"ancestor_record_offset":260,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":6068,"parent_record_offset":6044,"ancestor_record_offset":260,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":260,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":260,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":8876,"parent_record_offset":7880,"ancestor_record_offset":260,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":6364,"parent_record_offset":5660,"ancestor_record_offset":260,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":7540,"parent_record_offset":6796,"ancestor_record_offset":260,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":5636,"parent_record_offset":5612,"ancestor_record_offset":260,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":5612,"parent_record_offset":5552,"ancestor_record_offset":260,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":7880,"parent_record_offset":7852,"ancestor_record_offset":260,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":7684,"parent_record_offset":7656,"ancestor_record_offset":260,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":8736,"parent_record_offset":7880,"ancestor_record_offset":260,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":5660,"parent_record_offset":5636,"ancestor_record_offset":260,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":7960,"parent_record_offset":7924,"ancestor_record_offset":260,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":8128,"parent_record_offset":7960,"ancestor_record_offset":260,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":8348,"parent_record_offset":7960,"ancestor_record_offset":260,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":8380,"parent_record_offset":8348,"ancestor_record_offset":260,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":7924,"parent_record_offset":7880,"ancestor_record_offset":260,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":6900,"parent_record_offset":6824,"ancestor_record_offset":260,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":8664,"parent_record_offset":8640,"ancestor_record_offset":260,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":8640,"parent_record_offset":7924,"ancestor_record_offset":260,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":362,"record_offset":5900,"parent_record_offset":5872,"ancestor_record_offset":260,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":362,"record_offset":5928,"parent_record_offset":5900,"ancestor_record_offset":260,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":362,"record_offset":5988,"parent_record_offset":5872,"ancestor_record_offset":260,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":362,"record_offset":5872,"parent_record_offset":5660,"ancestor_record_offset":260,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":5704,"parent_record_offset":5660,"ancestor_record_offset":260,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":6020,"parent_record_offset":5660,"ancestor_record_offset":260,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":8404,"parent_record_offset":8380,"ancestor_record_offset":260,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":362,"record_offset":8848,"parent_record_offset":7880,"ancestor_record_offset":260,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":362,"record_offset":5732,"parent_record_offset":5660,"ancestor_record_offset":260,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":362,"record_offset":8436,"parent_record_offset":8348,"ancestor_record_offset":260,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":362,"record_offset":5816,"parent_record_offset":5660,"ancestor_record_offset":260,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":362,"record_offset":6272,"parent_record_offset":5660,"ancestor_record_offset":260,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":362,"record_offset":8172,"parent_record_offset":8128,"ancestor_record_offset":260,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::into_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":362,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":260,"inlinee":6550,"source_line":412,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":362,"record_offset":7852,"parent_record_offset":7824,"ancestor_record_offset":260,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":362,"record_offset":7352,"parent_record_offset":7216,"ancestor_record_offset":260,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":362,"record_offset":10668,"parent_record_offset":10644,"ancestor_record_offset":260,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::into_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":362,"record_offset":7768,"parent_record_offset":7732,"ancestor_record_offset":260,"inlinee":6548,"source_line":2927,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":362,"record_offset":7824,"parent_record_offset":7732,"ancestor_record_offset":260,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs
//     evidence={"module":362,"record_offset":9896,"parent_record_offset":9868,"ancestor_record_offset":260,"inlinee":21401,"source_line":67,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$4
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs
//     evidence={"module":362,"record_offset":10900,"parent_record_offset":10876,"ancestor_record_offset":260,"inlinee":21402,"source_line":68,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5399
// 首条位置: {"module":362,"inlinee":21401}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":362,"record_offset":9920,"parent_record_offset":9896,"ancestor_record_offset":9896,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":362,"record_offset":10132,"parent_record_offset":9988,"ancestor_record_offset":9896,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":362,"record_offset":9988,"parent_record_offset":9944,"ancestor_record_offset":9896,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":362,"record_offset":10776,"parent_record_offset":9944,"ancestor_record_offset":9896,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":362,"record_offset":10408,"parent_record_offset":9944,"ancestor_record_offset":9896,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":362,"record_offset":10432,"parent_record_offset":10408,"ancestor_record_offset":9896,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":362,"record_offset":9944,"parent_record_offset":9920,"ancestor_record_offset":9896,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":362,"record_offset":10736,"parent_record_offset":10712,"ancestor_record_offset":9896,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":362,"record_offset":10508,"parent_record_offset":10484,"ancestor_record_offset":9896,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":362,"record_offset":10484,"parent_record_offset":10460,"ancestor_record_offset":9896,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":362,"record_offset":10556,"parent_record_offset":10532,"ancestor_record_offset":9896,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":362,"record_offset":10644,"parent_record_offset":10620,"ancestor_record_offset":9896,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":362,"record_offset":10712,"parent_record_offset":10432,"ancestor_record_offset":9896,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":362,"record_offset":10460,"parent_record_offset":10432,"ancestor_record_offset":9896,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":362,"record_offset":10380,"parent_record_offset":9944,"ancestor_record_offset":9896,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":362,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":9896,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":362,"record_offset":10196,"parent_record_offset":10164,"ancestor_record_offset":9896,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":362,"record_offset":10100,"parent_record_offset":10076,"ancestor_record_offset":9896,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":362,"record_offset":10076,"parent_record_offset":9988,"ancestor_record_offset":9896,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":362,"record_offset":10580,"parent_record_offset":10556,"ancestor_record_offset":9896,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":362,"record_offset":10532,"parent_record_offset":10508,"ancestor_record_offset":9896,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":362,"record_offset":10304,"parent_record_offset":10272,"ancestor_record_offset":9896,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":362,"record_offset":10352,"parent_record_offset":9944,"ancestor_record_offset":9896,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":362,"record_offset":10020,"parent_record_offset":9988,"ancestor_record_offset":9896,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":362,"record_offset":10044,"parent_record_offset":10020,"ancestor_record_offset":9896,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":362,"record_offset":10164,"parent_record_offset":9944,"ancestor_record_offset":9896,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":362,"record_offset":10248,"parent_record_offset":10224,"ancestor_record_offset":9896,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":362,"record_offset":10272,"parent_record_offset":10248,"ancestor_record_offset":9896,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":362,"record_offset":10224,"parent_record_offset":10196,"ancestor_record_offset":9896,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":362,"record_offset":10668,"parent_record_offset":10644,"ancestor_record_offset":9896,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00362.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x539A
// 首条位置: {"module":362,"inlinee":21402}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8E2
// 首条位置: {"module":1226,"inlinee":59618}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1226,"record_offset":40108,"parent_record_offset":40084,"ancestor_record_offset":40084,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8D3
// 首条位置: {"module":1226,"inlinee":59603}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1226,"record_offset":20844,"parent_record_offset":20628,"ancestor_record_offset":20628,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1226,"record_offset":20668,"parent_record_offset":20628,"ancestor_record_offset":20628,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1226,"record_offset":20788,"parent_record_offset":20628,"ancestor_record_offset":20628,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1226,"record_offset":20812,"parent_record_offset":20788,"ancestor_record_offset":20628,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1226,"record_offset":20700,"parent_record_offset":20668,"ancestor_record_offset":20628,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE957
// 首条位置: {"module":1234,"inlinee":59735}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1234,"record_offset":2188,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopRange::map_slice
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1234,"record_offset":2132,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":21479,"source_line":174,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1234,"record_offset":2232,"parent_record_offset":2188,"ancestor_record_offset":2080,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1234,"record_offset":2260,"parent_record_offset":2188,"ancestor_record_offset":2080,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE956
// 首条位置: {"module":1234,"inlinee":59734}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1234,"record_offset":1912,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::LoopRange::map_slice
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1234,"record_offset":2020,"parent_record_offset":1856,"ancestor_record_offset":1856,"inlinee":21479,"source_line":174,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1234,"record_offset":1960,"parent_record_offset":1912,"ancestor_record_offset":1856,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1234,"record_offset":1988,"parent_record_offset":1912,"ancestor_record_offset":1856,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01234.jsonl.gz"}
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
// 完整结构及原始类型记录: split_wall_path_segment_into_platform_stairs.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: split_wall_path_segment_into_platform_stairs.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/platform-findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-stairs-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: split_wall_path_segment_into_platform_stairs.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>", "bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>", "bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs
// 信息 ID: 29377d5a36c2ef09f79c830f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 78575e7dd1a352176ee18642: void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0xa4577 -> 0x4dbbb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::CreateRoofCmd>,bevy_ecs::event::EventReader<country_core::resources::roofs::commands::SwitchRoofTypeCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasks>,bevy_ecs::change_detection::ResMut<system_wall_constructor::WallPathSegmentationMasksMinusStairs>,bevy_ecs::change_detection::ResMut<system_wall_constructor::PlatformStairsStorage>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x2e20e0 -> 0x4dbbb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$3
// 信息 ID: 9c1dcf6744f7c151df872331
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs; module=362, inline_record=9896; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$4
// 信息 ID: 91c8e8c289cf49d411a3be89
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs; module=362, inline_record=10900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$5
// 信息 ID: d90efbf03d65d63ef561cce3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$5 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$5> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$5> >; module=1226, inline_record=40084; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$5> >; module=1226, inline_record=40792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$6
// 信息 ID: 052669aa0a1ab2c7a3c536d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$6 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$6> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> >,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$6> >; module=1226, inline_record=20628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$7
// 信息 ID: 3a545f1d68984acaf2a7db36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7 <- utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8>; module=1234, inline_record=2080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$8
// 信息 ID: fc20662cad40bf4d1a00781e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8 <- utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8>; module=1234, inline_record=1856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
