// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/shapes/shape_placement_signifiers.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 1d0551928f01b23b493481400b5497e583a451dd678725a48fe4c4845c1ae80e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shape_placement_signifiers.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1302, 1321
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
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1302,"record_offset":51876,"rva":13771360,"va":5382480480}
// 编译实例 RVAs: 0xd22260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12867152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e8b0a93bff","size":5,"soft_fail":false,"target_va":5369613520,"va":5382480667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce150
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10833109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e82bb35aff","size":5,"soft_fail":false,"target_va":5371650384,"va":5382483488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402da5f0
//     owner=unresolved_namespace; count=2
//     target_aliases=15 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10782875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e865775bff","size":5,"soft_fail":false,"target_va":5371700720,"va":5382483590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9731264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e840836bff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382480955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=11
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9732065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e81f806bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382481772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405caa20
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7699660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e834838aff","size":5,"soft_fail":false,"target_va":5374781984,"va":5382481639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14065de00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7096909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e8b3b593ff","size":5,"soft_fail":false,"target_va":5375385088,"va":5382481992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5537325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e8d381abff","size":5,"soft_fail":false,"target_va":5376943968,"va":5382481288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d84d20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t402095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e8af220600","size":5,"soft_fail":false,"target_va":5382884640,"va":5382482540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e87c50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1462889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e869521600","size":5,"soft_fail":false,"target_va":5383945296,"va":5382482402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e881b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1464500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e8b4581600","size":5,"soft_fail":false,"target_va":5383946672,"va":5382482167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e896b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1470154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e8ca6e1600","size":5,"soft_fail":false,"target_va":5383952048,"va":5382481889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e89df0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1471662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e8ae741600","size":5,"soft_fail":false,"target_va":5383953904,"va":5382482237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8add0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1476793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e8b9881600","size":5,"soft_fail":false,"target_va":5383957968,"va":5382481170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8d560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1486038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e8d6ac1600","size":5,"soft_fail":false,"target_va":5383968096,"va":5382482053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417f5260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11348076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e86c28ad00","size":5,"soft_fail":false,"target_va":5393830496,"va":5382482415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21111754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e8ca234201","size":5,"soft_fail":false,"target_va":5403593360,"va":5382481601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21112050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e8f2244201","size":5,"soft_fail":false,"target_va":5403594144,"va":5382482089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t20495111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e807bb3801","size":5,"soft_fail":false,"target_va":5402978144,"va":5382483028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-4951042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e8fe73b4ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5382480925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-4956005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e89b60b4ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5382483120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20495862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e8f6bd3801","size":5,"soft_fail":false,"target_va":5402978880,"va":5382483013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t20496090","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e8dabe3801","size":5,"soft_fail":false,"target_va":5402979152,"va":5382483057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20496338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e8d2bf3801","size":5,"soft_fail":false,"target_va":5402979344,"va":5382483001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve3
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4512659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e86d24bbff","size":5,"soft_fail":false,"target_va":5377969552,"va":5382482206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve3
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4527218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382480480,"raw_hex":"e88eebbaff","size":5,"soft_fail":false,"target_va":5377954656,"va":5382481869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1302,"record_offset":53224,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1302,"record_offset":53308,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1302,"record_offset":56444,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1302,"record_offset":57764,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1302,"record_offset":52128,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1302,"record_offset":54312,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":55984,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1302,"record_offset":57680,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1302,"record_offset":56504,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":21382,"source_line":101,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1302,"record_offset":54348,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":1302,"record_offset":52156,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":53524,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":22967,"source_line":102,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":1302,"record_offset":55956,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":22969,"source_line":24,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::ShapeParameters::expand
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":1302,"record_offset":56412,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":23579,"source_line":210,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1302,"record_offset":54808,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1302,"record_offset":54376,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":21578,"source_line":637,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1302,"record_offset":56900,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1302,"record_offset":56476,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1302,"record_offset":56532,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1302,"record_offset":56652,"parent_record_offset":51876,"ancestor_record_offset":51876,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":53420,"parent_record_offset":53396,"ancestor_record_offset":51876,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":53444,"parent_record_offset":53420,"ancestor_record_offset":51876,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":56136,"parent_record_offset":56112,"ancestor_record_offset":51876,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":53364,"parent_record_offset":53336,"ancestor_record_offset":51876,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":53396,"parent_record_offset":53364,"ancestor_record_offset":51876,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":56112,"parent_record_offset":56056,"ancestor_record_offset":51876,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1302,"record_offset":53336,"parent_record_offset":53308,"ancestor_record_offset":51876,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1302,"record_offset":53272,"parent_record_offset":53248,"ancestor_record_offset":51876,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":56084,"parent_record_offset":56056,"ancestor_record_offset":51876,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":53876,"parent_record_offset":53848,"ancestor_record_offset":51876,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":53848,"parent_record_offset":53820,"ancestor_record_offset":51876,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":54032,"parent_record_offset":53848,"ancestor_record_offset":51876,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":56056,"parent_record_offset":56032,"ancestor_record_offset":51876,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":53780,"parent_record_offset":53744,"ancestor_record_offset":51876,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":53744,"parent_record_offset":53708,"ancestor_record_offset":51876,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":53672,"parent_record_offset":53636,"ancestor_record_offset":51876,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":53708,"parent_record_offset":53672,"ancestor_record_offset":51876,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":53636,"parent_record_offset":53600,"ancestor_record_offset":51876,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1302,"record_offset":53600,"parent_record_offset":53564,"ancestor_record_offset":51876,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1302,"record_offset":53564,"parent_record_offset":53524,"ancestor_record_offset":51876,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1302,"record_offset":54180,"parent_record_offset":54156,"ancestor_record_offset":51876,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":53820,"parent_record_offset":53780,"ancestor_record_offset":51876,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":54464,"parent_record_offset":54404,"ancestor_record_offset":51876,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":57788,"parent_record_offset":57764,"ancestor_record_offset":51876,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1302,"record_offset":52976,"parent_record_offset":52952,"ancestor_record_offset":51876,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1302,"record_offset":52444,"parent_record_offset":52420,"ancestor_record_offset":51876,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1302,"record_offset":52528,"parent_record_offset":52504,"ancestor_record_offset":51876,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1302,"record_offset":56556,"parent_record_offset":56532,"ancestor_record_offset":51876,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1302,"record_offset":56684,"parent_record_offset":56652,"ancestor_record_offset":51876,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1302,"record_offset":52468,"parent_record_offset":52444,"ancestor_record_offset":51876,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1302,"record_offset":57812,"parent_record_offset":57788,"ancestor_record_offset":51876,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1302,"record_offset":57352,"parent_record_offset":57320,"ancestor_record_offset":51876,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1302,"record_offset":52640,"parent_record_offset":52616,"ancestor_record_offset":51876,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1302,"record_offset":54836,"parent_record_offset":54808,"ancestor_record_offset":51876,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1302,"record_offset":54584,"parent_record_offset":54560,"ancestor_record_offset":51876,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1302,"record_offset":53248,"parent_record_offset":53224,"ancestor_record_offset":51876,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1302,"record_offset":54072,"parent_record_offset":53780,"ancestor_record_offset":51876,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":54156,"parent_record_offset":53524,"ancestor_record_offset":51876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1302,"record_offset":57728,"parent_record_offset":57704,"ancestor_record_offset":51876,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1302,"record_offset":52888,"parent_record_offset":52864,"ancestor_record_offset":51876,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1302,"record_offset":52780,"parent_record_offset":52756,"ancestor_record_offset":51876,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":53472,"parent_record_offset":53444,"ancestor_record_offset":51876,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1302,"record_offset":54492,"parent_record_offset":54464,"ancestor_record_offset":51876,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1302,"record_offset":54520,"parent_record_offset":54492,"ancestor_record_offset":51876,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1302,"record_offset":57704,"parent_record_offset":57680,"ancestor_record_offset":51876,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1302,"record_offset":54216,"parent_record_offset":54180,"ancestor_record_offset":51876,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1302,"record_offset":54240,"parent_record_offset":54216,"ancestor_record_offset":51876,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1302,"record_offset":54272,"parent_record_offset":54180,"ancestor_record_offset":51876,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1302,"record_offset":53000,"parent_record_offset":52976,"ancestor_record_offset":51876,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1302,"record_offset":54724,"parent_record_offset":54692,"ancestor_record_offset":51876,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1302,"record_offset":54692,"parent_record_offset":54668,"ancestor_record_offset":51876,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1302,"record_offset":54668,"parent_record_offset":54644,"ancestor_record_offset":51876,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1302,"record_offset":54644,"parent_record_offset":54620,"ancestor_record_offset":51876,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1302,"record_offset":54560,"parent_record_offset":54404,"ancestor_record_offset":51876,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1302,"record_offset":52952,"parent_record_offset":52928,"ancestor_record_offset":51876,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1302,"record_offset":52216,"parent_record_offset":52192,"ancestor_record_offset":51876,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1302,"record_offset":52240,"parent_record_offset":52216,"ancestor_record_offset":51876,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1302,"record_offset":52928,"parent_record_offset":52708,"ancestor_record_offset":51876,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1302,"record_offset":52272,"parent_record_offset":52240,"ancestor_record_offset":51876,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":52840,"parent_record_offset":52816,"ancestor_record_offset":51876,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":52816,"parent_record_offset":52708,"ancestor_record_offset":51876,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":52756,"parent_record_offset":52732,"ancestor_record_offset":51876,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":53136,"parent_record_offset":52348,"ancestor_record_offset":51876,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":52732,"parent_record_offset":52708,"ancestor_record_offset":51876,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":52324,"parent_record_offset":52300,"ancestor_record_offset":51876,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":52300,"parent_record_offset":52240,"ancestor_record_offset":51876,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":52348,"parent_record_offset":52324,"ancestor_record_offset":51876,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1302,"record_offset":52588,"parent_record_offset":52560,"ancestor_record_offset":51876,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1302,"record_offset":52616,"parent_record_offset":52588,"ancestor_record_offset":51876,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1302,"record_offset":52676,"parent_record_offset":52560,"ancestor_record_offset":51876,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1302,"record_offset":52560,"parent_record_offset":52348,"ancestor_record_offset":51876,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":52392,"parent_record_offset":52348,"ancestor_record_offset":51876,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1302,"record_offset":52708,"parent_record_offset":52348,"ancestor_record_offset":51876,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1302,"record_offset":52420,"parent_record_offset":52348,"ancestor_record_offset":51876,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1302,"record_offset":52504,"parent_record_offset":52348,"ancestor_record_offset":51876,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1302,"record_offset":53044,"parent_record_offset":52348,"ancestor_record_offset":51876,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1302,"record_offset":52192,"parent_record_offset":52156,"ancestor_record_offset":51876,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1302,"record_offset":54756,"parent_record_offset":54724,"ancestor_record_offset":51876,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1302,"record_offset":56736,"parent_record_offset":56712,"ancestor_record_offset":51876,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1302,"record_offset":54404,"parent_record_offset":54376,"ancestor_record_offset":51876,"inlinee":21585,"source_line":633,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1302,"record_offset":53192,"parent_record_offset":52156,"ancestor_record_offset":51876,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1302,"record_offset":54436,"parent_record_offset":54404,"ancestor_record_offset":51876,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1302,"record_offset":56712,"parent_record_offset":56652,"ancestor_record_offset":51876,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1302,"record_offset":54620,"parent_record_offset":54376,"ancestor_record_offset":51876,"inlinee":22391,"source_line":65,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1302,"record_offset":60696,"rva":13775440,"va":5382484560}
// 编译实例 RVAs: 0xd23250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9734932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382484560,"raw_hex":"e8ec746bff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382484623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9735407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382484560,"raw_hex":"e811736bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382485114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7144737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382484560,"raw_hex":"e8dffa92ff","size":5,"soft_fail":false,"target_va":5375340432,"va":5382485164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20493117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382484560,"raw_hex":"e83db33801","size":5,"soft_fail":false,"target_va":5402978144,"va":5382485022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7128064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382484560,"raw_hex":"e8003c93ff","size":5,"soft_fail":false,"target_va":5375356896,"va":5382484955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":60908,"parent_record_offset":60696,"ancestor_record_offset":60696,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":62240,"parent_record_offset":60696,"ancestor_record_offset":60696,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1302,"record_offset":61056,"parent_record_offset":60696,"ancestor_record_offset":60696,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":62324,"parent_record_offset":60696,"ancestor_record_offset":60696,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":61612,"parent_record_offset":60696,"ancestor_record_offset":60696,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":62184,"parent_record_offset":60696,"ancestor_record_offset":60696,"inlinee":26370,"source_line":179,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":62212,"parent_record_offset":60696,"ancestor_record_offset":60696,"inlinee":26724,"source_line":189,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":60964,"parent_record_offset":60940,"ancestor_record_offset":60696,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":60988,"parent_record_offset":60964,"ancestor_record_offset":60696,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":62396,"parent_record_offset":62372,"ancestor_record_offset":60696,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":60940,"parent_record_offset":60908,"ancestor_record_offset":60696,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":62372,"parent_record_offset":62348,"ancestor_record_offset":60696,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1302,"record_offset":62348,"parent_record_offset":62324,"ancestor_record_offset":60696,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":61344,"parent_record_offset":61316,"ancestor_record_offset":60696,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":61316,"parent_record_offset":61288,"ancestor_record_offset":60696,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":61488,"parent_record_offset":61316,"ancestor_record_offset":60696,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":61252,"parent_record_offset":61224,"ancestor_record_offset":60696,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":62264,"parent_record_offset":62240,"ancestor_record_offset":60696,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":61224,"parent_record_offset":61196,"ancestor_record_offset":60696,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":61168,"parent_record_offset":61140,"ancestor_record_offset":60696,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":61196,"parent_record_offset":61168,"ancestor_record_offset":60696,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":61140,"parent_record_offset":61112,"ancestor_record_offset":60696,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1302,"record_offset":61112,"parent_record_offset":61084,"ancestor_record_offset":60696,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1302,"record_offset":61084,"parent_record_offset":61056,"ancestor_record_offset":60696,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":62288,"parent_record_offset":62264,"ancestor_record_offset":60696,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":61288,"parent_record_offset":61252,"ancestor_record_offset":60696,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1302,"record_offset":61548,"parent_record_offset":61524,"ancestor_record_offset":60696,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1302,"record_offset":61524,"parent_record_offset":61252,"ancestor_record_offset":60696,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":61012,"parent_record_offset":60988,"ancestor_record_offset":60696,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF139
// 首条位置: {"module":1321,"inlinee":61753}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1321,"record_offset":15248,"parent_record_offset":15204,"ancestor_record_offset":15204,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1321,"record_offset":15332,"parent_record_offset":15204,"ancestor_record_offset":15204,"inlinee":22206,"source_line":61,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1321,"record_offset":15304,"parent_record_offset":15204,"ancestor_record_offset":15204,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1321,"record_offset":15276,"parent_record_offset":15204,"ancestor_record_offset":15204,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1321,"record_offset":15644,"parent_record_offset":15616,"ancestor_record_offset":15204,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1321,"record_offset":15672,"parent_record_offset":15616,"ancestor_record_offset":15204,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1321,"record_offset":15732,"parent_record_offset":15708,"ancestor_record_offset":15204,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1321,"record_offset":15560,"parent_record_offset":15536,"ancestor_record_offset":15204,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1321,"record_offset":15504,"parent_record_offset":15480,"ancestor_record_offset":15204,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1321,"record_offset":16000,"parent_record_offset":15976,"ancestor_record_offset":15204,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1321,"record_offset":15940,"parent_record_offset":15916,"ancestor_record_offset":15204,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1321,"record_offset":15892,"parent_record_offset":15860,"ancestor_record_offset":15204,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1321,"record_offset":15916,"parent_record_offset":15892,"ancestor_record_offset":15204,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$2::load_index_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1321,"record_offset":15860,"parent_record_offset":15828,"ancestor_record_offset":15204,"inlinee":22210,"source_line":159,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$3::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":1321,"record_offset":15364,"parent_record_offset":15332,"ancestor_record_offset":15204,"inlinee":22208,"source_line":175,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1321,"record_offset":15764,"parent_record_offset":15400,"ancestor_record_offset":15204,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1321,"record_offset":15592,"parent_record_offset":15400,"ancestor_record_offset":15204,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1321,"record_offset":15616,"parent_record_offset":15592,"ancestor_record_offset":15204,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1321,"record_offset":15536,"parent_record_offset":15400,"ancestor_record_offset":15204,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1321,"record_offset":16124,"parent_record_offset":15400,"ancestor_record_offset":15204,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1321,"record_offset":15480,"parent_record_offset":15400,"ancestor_record_offset":15204,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1321,"record_offset":15708,"parent_record_offset":15400,"ancestor_record_offset":15204,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":1321,"record_offset":15800,"parent_record_offset":15400,"ancestor_record_offset":15204,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1321,"record_offset":15976,"parent_record_offset":15860,"ancestor_record_offset":15204,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":1321,"record_offset":15828,"parent_record_offset":15400,"ancestor_record_offset":15204,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":1321,"record_offset":15400,"parent_record_offset":15364,"ancestor_record_offset":15204,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1302,"record_offset":62440,"rva":13776064,"va":5382485184}
// 编译实例 RVAs: 0xd234c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9735560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382485184,"raw_hex":"e878726bff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382485251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9736160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382485184,"raw_hex":"e820706bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382485867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7145509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382485184,"raw_hex":"e8dbf792ff","size":5,"soft_fail":false,"target_va":5375340432,"va":5382485936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21107990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382485184,"raw_hex":"e816154201","size":5,"soft_fail":false,"target_va":5403593360,"va":5382485365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20492359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382485184,"raw_hex":"e847b03801","size":5,"soft_fail":false,"target_va":5402978144,"va":5382485780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20433446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382485184,"raw_hex":"e826ca3701","size":5,"soft_fail":false,"target_va":5402919072,"va":5382485621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4958687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382485184,"raw_hex":"e82156b4ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5382485802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::world::command_queue::CommandQueue::get_raw
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5239113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382485184,"raw_hex":"e8b70eb0ff","size":5,"soft_fail":false,"target_va":5377246528,"va":5382485636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7128684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382485184,"raw_hex":"e8943993ff","size":5,"soft_fail":false,"target_va":5375356896,"va":5382485575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":62652,"parent_record_offset":62440,"ancestor_record_offset":62440,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":64460,"parent_record_offset":62440,"ancestor_record_offset":62440,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":62800,"parent_record_offset":62440,"ancestor_record_offset":62440,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1302,"record_offset":63904,"parent_record_offset":62440,"ancestor_record_offset":62440,"inlinee":14636,"source_line":945,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":64544,"parent_record_offset":62440,"ancestor_record_offset":62440,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":63876,"parent_record_offset":62440,"ancestor_record_offset":62440,"inlinee":23597,"source_line":201,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1302,"record_offset":63300,"parent_record_offset":62440,"ancestor_record_offset":62440,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":62708,"parent_record_offset":62684,"ancestor_record_offset":62440,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":62732,"parent_record_offset":62708,"ancestor_record_offset":62440,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":64616,"parent_record_offset":64592,"ancestor_record_offset":62440,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":62684,"parent_record_offset":62652,"ancestor_record_offset":62440,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1302,"record_offset":64592,"parent_record_offset":64568,"ancestor_record_offset":62440,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1302,"record_offset":64568,"parent_record_offset":64544,"ancestor_record_offset":62440,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":64132,"parent_record_offset":64104,"ancestor_record_offset":62440,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":64104,"parent_record_offset":64072,"ancestor_record_offset":62440,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":64072,"parent_record_offset":64044,"ancestor_record_offset":62440,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":62976,"parent_record_offset":62948,"ancestor_record_offset":62440,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":62948,"parent_record_offset":62920,"ancestor_record_offset":62440,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1302,"record_offset":63188,"parent_record_offset":62948,"ancestor_record_offset":62440,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":62884,"parent_record_offset":62856,"ancestor_record_offset":62440,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":64484,"parent_record_offset":64460,"ancestor_record_offset":62440,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":62856,"parent_record_offset":62828,"ancestor_record_offset":62440,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":63428,"parent_record_offset":63400,"ancestor_record_offset":62440,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":62828,"parent_record_offset":62800,"ancestor_record_offset":62440,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1302,"record_offset":63400,"parent_record_offset":63372,"ancestor_record_offset":62440,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1302,"record_offset":63372,"parent_record_offset":63344,"ancestor_record_offset":62440,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1302,"record_offset":63344,"parent_record_offset":63300,"ancestor_record_offset":62440,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":64508,"parent_record_offset":64484,"ancestor_record_offset":62440,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":64184,"parent_record_offset":64016,"ancestor_record_offset":62440,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":64044,"parent_record_offset":64016,"ancestor_record_offset":62440,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1302,"record_offset":62920,"parent_record_offset":62884,"ancestor_record_offset":62440,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1302,"record_offset":63952,"parent_record_offset":63928,"ancestor_record_offset":62440,"inlinee":14638,"source_line":748,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1302,"record_offset":63980,"parent_record_offset":63952,"ancestor_record_offset":62440,"inlinee":12140,"source_line":501,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::add
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":1302,"record_offset":63928,"parent_record_offset":63904,"ancestor_record_offset":62440,"inlinee":14637,"source_line":1100,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::CommandQueue::push
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1302,"record_offset":64216,"parent_record_offset":63980,"ancestor_record_offset":62440,"inlinee":12149,"source_line":78,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::command_queue::RawCommandQueue::push
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/command_queue.rs
//     evidence={"module":1302,"record_offset":64016,"parent_record_offset":63980,"ancestor_record_offset":62440,"inlinee":12142,"source_line":157,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1302,"record_offset":63012,"parent_record_offset":62976,"ancestor_record_offset":62440,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1302,"record_offset":63036,"parent_record_offset":63012,"ancestor_record_offset":62440,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1302,"record_offset":63252,"parent_record_offset":63228,"ancestor_record_offset":62440,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1302,"record_offset":63228,"parent_record_offset":62884,"ancestor_record_offset":62440,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01302.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1302,"record_offset":62756,"parent_record_offset":62732,"ancestor_record_offset":62440,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01302.jsonl.gz"}
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
// 完整结构及原始类型记录: shape_placement_signifiers.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shape_placement_signifiers.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shape_placement_signifiers.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers
// 信息 ID: 5ae9261a0430d0a24b5f280c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3cafa32bbb39667c4803ab5b: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)> > @ 0xe7df4e -> 0xd22260 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)> @ 0x182491c -> 0xd22260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure$0
// 信息 ID: 0d8c938e46e29a5b7b37525c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$3,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xd1e542 -> 0xd23250 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure$1
// 信息 ID: 98e660a934880f596547f10a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1 <- core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec3::Vec3,core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1> > > >; module=1321, inline_record=15204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure$2
// 信息 ID: f866e90378ab0f426f22ebef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$2 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$2 <- core::ptr::drop_in_place<country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0xd1e5d3 -> 0xd234c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
