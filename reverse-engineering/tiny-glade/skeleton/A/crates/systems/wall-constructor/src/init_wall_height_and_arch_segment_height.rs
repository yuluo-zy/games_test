// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/init_wall_height_and_arch_segment_height.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 46622cec446fedc8e3531ff3492aa771a7671097be3791471f7468253a04f0a5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: init_wall_height_and_arch_segment_height.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 377, 1226, 1264
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
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
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
// 函数符号: system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height
// 记录形式: compiled_procedure; instances=1
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":377,"record_offset":508,"rva":5222528,"va":5373931648}
// 编译实例 RVAs: 0x4fb080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4318707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e80d1abeff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373932222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1182789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e8bbf3edff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373932496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1182707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e80df4edff","size":5,"soft_fail":false,"target_va":5372749744,"va":5373932446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14051aa00
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t127142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e8a6f00100","size":5,"soft_fail":false,"target_va":5374061056,"va":5373933909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1945826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e8e2b01d00","size":5,"soft_fail":false,"target_va":5375881440,"va":5373935609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3005949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e8fddd2d00","size":5,"soft_fail":false,"target_va":5376939904,"va":5373933950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cacf70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8066464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e8a0157b00","size":5,"soft_fail":false,"target_va":5382000496,"va":5373934027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8098324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e814927b00","size":5,"soft_fail":false,"target_va":5382032368,"va":5373934039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: init_wall_height_and_arch_segment_height.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29659789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e88d92c401","size":5,"soft_fail":false,"target_va":5403594240,"va":5373934446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29042766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e84e28bb01","size":5,"soft_fail":false,"target_va":5402978144,"va":5373935373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28983732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e8b441ba01","size":5,"soft_fail":false,"target_va":5402918528,"va":5373934791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3591737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e839ce3600","size":5,"soft_fail":false,"target_va":5377527120,"va":5373935378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29043515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e83b2bbb01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373935360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29043991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e8172dbb01","size":5,"soft_fail":false,"target_va":5402979344,"va":5373935348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29044695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e8d72fbb01","size":5,"soft_fail":false,"target_va":5402980128,"va":5373935428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1829118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e8fee81b00","size":5,"soft_fail":false,"target_va":5375763504,"va":5373934381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1830050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e8a2ec1b00","size":5,"soft_fail":false,"target_va":5375763568,"va":5373933513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::RoofShapeParams::eave_length_ws
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1831958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e816f41b00","size":5,"soft_fail":false,"target_va":5375765888,"va":5373933925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7999854","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e86e117a00","size":5,"soft_fail":false,"target_va":5381933120,"va":5373933261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1502718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e8feed1600","size":5,"soft_fail":false,"target_va":5375436032,"va":5373933309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t4022263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373931648,"raw_hex":"e8f75f3d00","size":5,"soft_fail":false,"target_va":5377955840,"va":5373933572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":377,"record_offset":3524,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":377,"record_offset":3556,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":5438,"source_line":2506,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":377,"record_offset":2788,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":377,"record_offset":8356,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":377,"record_offset":4112,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":377,"record_offset":1732,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":377,"record_offset":788,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":377,"record_offset":5000,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":377,"record_offset":7524,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":377,"record_offset":3460,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":377,"record_offset":1336,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":377,"record_offset":7312,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":377,"record_offset":7396,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":377,"record_offset":8452,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":377,"record_offset":3104,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":377,"record_offset":8480,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":377,"record_offset":3368,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":377,"record_offset":3340,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":377,"record_offset":7004,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":377,"record_offset":1788,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":377,"record_offset":7564,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":6118,"source_line":412,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::to_curve2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":377,"record_offset":7060,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":21343,"source_line":938,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":377,"record_offset":7368,"parent_record_offset":508,"ancestor_record_offset":508,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":377,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":508,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":377,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":508,"inlinee":6014,"source_line":171,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":377,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":508,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":377,"record_offset":2972,"parent_record_offset":2872,"ancestor_record_offset":508,"inlinee":6013,"source_line":247,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":377,"record_offset":3272,"parent_record_offset":3164,"ancestor_record_offset":508,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":377,"record_offset":3200,"parent_record_offset":3164,"ancestor_record_offset":508,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":377,"record_offset":3712,"parent_record_offset":3680,"ancestor_record_offset":508,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":377,"record_offset":3804,"parent_record_offset":3748,"ancestor_record_offset":508,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":377,"record_offset":3680,"parent_record_offset":3652,"ancestor_record_offset":508,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":377,"record_offset":2872,"parent_record_offset":2844,"ancestor_record_offset":508,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":377,"record_offset":2844,"parent_record_offset":2816,"ancestor_record_offset":508,"inlinee":6012,"source_line":169,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":377,"record_offset":3060,"parent_record_offset":2844,"ancestor_record_offset":508,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":377,"record_offset":3164,"parent_record_offset":3132,"ancestor_record_offset":508,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":377,"record_offset":3748,"parent_record_offset":3588,"ancestor_record_offset":508,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":377,"record_offset":4184,"parent_record_offset":4148,"ancestor_record_offset":508,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":377,"record_offset":3588,"parent_record_offset":3556,"ancestor_record_offset":508,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":377,"record_offset":3652,"parent_record_offset":3588,"ancestor_record_offset":508,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":377,"record_offset":7200,"parent_record_offset":7176,"ancestor_record_offset":508,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":377,"record_offset":4640,"parent_record_offset":4612,"ancestor_record_offset":508,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":377,"record_offset":4048,"parent_record_offset":3588,"ancestor_record_offset":508,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":377,"record_offset":4960,"parent_record_offset":4184,"ancestor_record_offset":508,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":377,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":508,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":377,"record_offset":4148,"parent_record_offset":4112,"ancestor_record_offset":508,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":377,"record_offset":2816,"parent_record_offset":2788,"ancestor_record_offset":508,"inlinee":11922,"source_line":23,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":377,"record_offset":5212,"parent_record_offset":5068,"ancestor_record_offset":508,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":377,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":508,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":377,"record_offset":5068,"parent_record_offset":5024,"ancestor_record_offset":508,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":377,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":508,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":377,"record_offset":852,"parent_record_offset":812,"ancestor_record_offset":508,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":377,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":508,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":377,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":508,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":377,"record_offset":5856,"parent_record_offset":5024,"ancestor_record_offset":508,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":377,"record_offset":5488,"parent_record_offset":5024,"ancestor_record_offset":508,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":377,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":508,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":377,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":508,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":377,"record_offset":5816,"parent_record_offset":5792,"ancestor_record_offset":508,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":377,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":508,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":377,"record_offset":5564,"parent_record_offset":5540,"ancestor_record_offset":508,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":377,"record_offset":5636,"parent_record_offset":5612,"ancestor_record_offset":508,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":377,"record_offset":5724,"parent_record_offset":5700,"ancestor_record_offset":508,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":377,"record_offset":5792,"parent_record_offset":5512,"ancestor_record_offset":508,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":377,"record_offset":5540,"parent_record_offset":5512,"ancestor_record_offset":508,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":377,"record_offset":5460,"parent_record_offset":5024,"ancestor_record_offset":508,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":377,"record_offset":812,"parent_record_offset":788,"ancestor_record_offset":508,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":377,"record_offset":5880,"parent_record_offset":5856,"ancestor_record_offset":508,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":377,"record_offset":2908,"parent_record_offset":2872,"ancestor_record_offset":508,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":377,"record_offset":2936,"parent_record_offset":2908,"ancestor_record_offset":508,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":377,"record_offset":5276,"parent_record_offset":5244,"ancestor_record_offset":508,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":377,"record_offset":7452,"parent_record_offset":7424,"ancestor_record_offset":508,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":377,"record_offset":5180,"parent_record_offset":5156,"ancestor_record_offset":508,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":377,"record_offset":7480,"parent_record_offset":7452,"ancestor_record_offset":508,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":377,"record_offset":3912,"parent_record_offset":3876,"ancestor_record_offset":508,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":377,"record_offset":2572,"parent_record_offset":2548,"ancestor_record_offset":508,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":377,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":508,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":377,"record_offset":2124,"parent_record_offset":2100,"ancestor_record_offset":508,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":377,"record_offset":4576,"parent_record_offset":4544,"ancestor_record_offset":508,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":377,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":508,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":377,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":508,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":377,"record_offset":1412,"parent_record_offset":1384,"ancestor_record_offset":508,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":377,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":508,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":377,"record_offset":4372,"parent_record_offset":4332,"ancestor_record_offset":508,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":377,"record_offset":4512,"parent_record_offset":4412,"ancestor_record_offset":508,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":377,"record_offset":7336,"parent_record_offset":7312,"ancestor_record_offset":508,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":377,"record_offset":3876,"parent_record_offset":3840,"ancestor_record_offset":508,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":377,"record_offset":3840,"parent_record_offset":3588,"ancestor_record_offset":508,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":377,"record_offset":7176,"parent_record_offset":7060,"ancestor_record_offset":508,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":377,"record_offset":4332,"parent_record_offset":4184,"ancestor_record_offset":508,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":377,"record_offset":4612,"parent_record_offset":4512,"ancestor_record_offset":508,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":377,"record_offset":924,"parent_record_offset":876,"ancestor_record_offset":508,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":377,"record_offset":3236,"parent_record_offset":3200,"ancestor_record_offset":508,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":377,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":508,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":377,"record_offset":5156,"parent_record_offset":5068,"ancestor_record_offset":508,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":377,"record_offset":7028,"parent_record_offset":7004,"ancestor_record_offset":508,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":377,"record_offset":7720,"parent_record_offset":7596,"ancestor_record_offset":508,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":377,"record_offset":7624,"parent_record_offset":7596,"ancestor_record_offset":508,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":377,"record_offset":1696,"parent_record_offset":1672,"ancestor_record_offset":508,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":377,"record_offset":7424,"parent_record_offset":7396,"ancestor_record_offset":508,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":377,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":508,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":377,"record_offset":5660,"parent_record_offset":5636,"ancestor_record_offset":508,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":377,"record_offset":1672,"parent_record_offset":1336,"ancestor_record_offset":508,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":377,"record_offset":1492,"parent_record_offset":1412,"ancestor_record_offset":508,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":377,"record_offset":3132,"parent_record_offset":3104,"ancestor_record_offset":508,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":377,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":508,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":377,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":508,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":377,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":508,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":377,"record_offset":1616,"parent_record_offset":1592,"ancestor_record_offset":508,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":377,"record_offset":3420,"parent_record_offset":3368,"ancestor_record_offset":508,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":377,"record_offset":3960,"parent_record_offset":3588,"ancestor_record_offset":508,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":377,"record_offset":7264,"parent_record_offset":7232,"ancestor_record_offset":508,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":377,"record_offset":5432,"parent_record_offset":5024,"ancestor_record_offset":508,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":377,"record_offset":988,"parent_record_offset":876,"ancestor_record_offset":508,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":377,"record_offset":1180,"parent_record_offset":876,"ancestor_record_offset":508,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":377,"record_offset":8384,"parent_record_offset":8356,"ancestor_record_offset":508,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":377,"record_offset":5124,"parent_record_offset":5100,"ancestor_record_offset":508,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":377,"record_offset":8412,"parent_record_offset":8384,"ancestor_record_offset":508,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":377,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":508,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":377,"record_offset":1028,"parent_record_offset":988,"ancestor_record_offset":508,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":377,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":508,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":377,"record_offset":1592,"parent_record_offset":1568,"ancestor_record_offset":508,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":377,"record_offset":4412,"parent_record_offset":4372,"ancestor_record_offset":508,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":377,"record_offset":1568,"parent_record_offset":1516,"ancestor_record_offset":508,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":377,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":508,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":377,"record_offset":5244,"parent_record_offset":5024,"ancestor_record_offset":508,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":377,"record_offset":5328,"parent_record_offset":5304,"ancestor_record_offset":508,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":377,"record_offset":5352,"parent_record_offset":5328,"ancestor_record_offset":508,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":377,"record_offset":5304,"parent_record_offset":5276,"ancestor_record_offset":508,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":377,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":508,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":377,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":508,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":377,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":508,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":377,"record_offset":2524,"parent_record_offset":2304,"ancestor_record_offset":508,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":377,"record_offset":1868,"parent_record_offset":1836,"ancestor_record_offset":508,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":377,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":508,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":377,"record_offset":2412,"parent_record_offset":2304,"ancestor_record_offset":508,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":377,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":508,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":377,"record_offset":2732,"parent_record_offset":1944,"ancestor_record_offset":508,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":377,"record_offset":2328,"parent_record_offset":2304,"ancestor_record_offset":508,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":377,"record_offset":1920,"parent_record_offset":1896,"ancestor_record_offset":508,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":377,"record_offset":1896,"parent_record_offset":1836,"ancestor_record_offset":508,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":377,"record_offset":1944,"parent_record_offset":1920,"ancestor_record_offset":508,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":377,"record_offset":2184,"parent_record_offset":2156,"ancestor_record_offset":508,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":377,"record_offset":2212,"parent_record_offset":2184,"ancestor_record_offset":508,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":377,"record_offset":2272,"parent_record_offset":2156,"ancestor_record_offset":508,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":377,"record_offset":2156,"parent_record_offset":1944,"ancestor_record_offset":508,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":377,"record_offset":1988,"parent_record_offset":1944,"ancestor_record_offset":508,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":377,"record_offset":2304,"parent_record_offset":1944,"ancestor_record_offset":508,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":377,"record_offset":2016,"parent_record_offset":1944,"ancestor_record_offset":508,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":377,"record_offset":2100,"parent_record_offset":1944,"ancestor_record_offset":508,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":377,"record_offset":2640,"parent_record_offset":1944,"ancestor_record_offset":508,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":377,"record_offset":5748,"parent_record_offset":5724,"ancestor_record_offset":508,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/init_wall_height_and_arch_segment_height.rs
//     evidence={"module":377,"record_offset":4544,"parent_record_offset":4512,"ancestor_record_offset":508,"inlinee":21645,"source_line":181,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":377,"record_offset":7232,"parent_record_offset":7060,"ancestor_record_offset":508,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/00377.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x548D
// 首条位置: {"module":377,"inlinee":21645}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":377,"record_offset":4576,"parent_record_offset":4544,"ancestor_record_offset":4544,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00377.jsonl.gz"}
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
// 完整结构及原始类型记录: init_wall_height_and_arch_segment_height.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: init_wall_height_and_arch_segment_height.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: init_wall_height_and_arch_segment_height.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >

// ------------------------------------------------------------------
// 函数: system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height
// 信息 ID: 63a3ea0cf5aff9dbf7d15548
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d166b47cf240b7ff8dc52c77: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)> > @ 0xaf060 -> 0x4fb080 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)> @ 0x250ed2 -> 0x4fb080 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height::closure$1
// 信息 ID: 5144c0f5b5f4c9c1212b75e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height; module=377, inline_record=4544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
