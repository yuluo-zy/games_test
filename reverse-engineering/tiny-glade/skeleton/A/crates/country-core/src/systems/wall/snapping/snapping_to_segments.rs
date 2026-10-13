// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/snapping/snapping_to_segments.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: fe94264b95ceade60dcc1ed8f65b8200b22d8770a248c9dc22b6806e2e1b295f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: snapping_to_segments.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 463, 1336, 1447
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
// 函数符号: country_core::systems::wall::snapping::snapping_to_segments::snapping_to_segments
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":463,"record_offset":18732,"rva":6560560,"va":5375269680}
// 编译实例 RVAs: 0x641b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5657284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375269680,"raw_hex":"e83cada9ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5375270799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2520403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375269680,"raw_hex":"e8ad8ad9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375270110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1673794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375269680,"raw_hex":"e8428a1900","size":5,"soft_fail":false,"target_va":5376943968,"va":5375270169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d82360
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7603975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375269680,"raw_hex":"e807077400","size":5,"soft_fail":false,"target_va":5382873952,"va":5375269972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ea9390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: snapping_to_segments.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8812301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375269680,"raw_hex":"e80d778600","size":5,"soft_fail":false,"target_va":5384082320,"va":5375270014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27707713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375269680,"raw_hex":"e841c9a601","size":5,"soft_fail":false,"target_va":5402978880,"va":5375271162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27707949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375269680,"raw_hex":"e82dcaa601","size":5,"soft_fail":false,"target_va":5402979152,"va":5375271198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_aabb_clamped
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-454416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375269680,"raw_hex":"e8f010f9ff","size":5,"soft_fail":false,"target_va":5374815504,"va":5375269915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::quat::Quat::from_rotation_arc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2078147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375269680,"raw_hex":"e8c3b51f00","size":5,"soft_fail":false,"target_va":5377348352,"va":5375270200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1669219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375269680,"raw_hex":"e863781900","size":5,"soft_fail":false,"target_va":5376939952,"va":5375270728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":463,"record_offset":23540,"parent_record_offset":18732,"ancestor_record_offset":18732,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":463,"record_offset":18924,"parent_record_offset":18732,"ancestor_record_offset":18732,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":463,"record_offset":18952,"parent_record_offset":18732,"ancestor_record_offset":18732,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/cursor_terrain_raycast.rs
//     evidence={"module":463,"record_offset":18988,"parent_record_offset":18732,"ancestor_record_offset":18732,"inlinee":22044,"source_line":28,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":463,"record_offset":20924,"parent_record_offset":18732,"ancestor_record_offset":18732,"inlinee":22206,"source_line":61,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":463,"record_offset":22436,"parent_record_offset":18732,"ancestor_record_offset":18732,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/snapping/snapping_to_segments.rs
//     evidence={"module":463,"record_offset":19052,"parent_record_offset":18732,"ancestor_record_offset":18732,"inlinee":26441,"source_line":55,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":19788,"parent_record_offset":19764,"ancestor_record_offset":18732,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":19764,"parent_record_offset":19672,"ancestor_record_offset":18732,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":19700,"parent_record_offset":19672,"ancestor_record_offset":18732,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":19672,"parent_record_offset":19648,"ancestor_record_offset":18732,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":463,"record_offset":19416,"parent_record_offset":19052,"ancestor_record_offset":18732,"inlinee":6488,"source_line":263,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":463,"record_offset":19440,"parent_record_offset":19416,"ancestor_record_offset":18732,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":19388,"parent_record_offset":19052,"ancestor_record_offset":18732,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":19136,"parent_record_offset":19052,"ancestor_record_offset":18732,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":19328,"parent_record_offset":19304,"ancestor_record_offset":18732,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":463,"record_offset":23564,"parent_record_offset":23540,"ancestor_record_offset":18732,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":463,"record_offset":23256,"parent_record_offset":23232,"ancestor_record_offset":18732,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":463,"record_offset":21384,"parent_record_offset":20988,"ancestor_record_offset":18732,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":463,"record_offset":22724,"parent_record_offset":22700,"ancestor_record_offset":18732,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":463,"record_offset":22808,"parent_record_offset":22784,"ancestor_record_offset":18732,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":463,"record_offset":21236,"parent_record_offset":21208,"ancestor_record_offset":18732,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":463,"record_offset":21264,"parent_record_offset":21208,"ancestor_record_offset":18732,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":463,"record_offset":22748,"parent_record_offset":22724,"ancestor_record_offset":18732,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":463,"record_offset":19352,"parent_record_offset":19328,"ancestor_record_offset":18732,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":463,"record_offset":19304,"parent_record_offset":19052,"ancestor_record_offset":18732,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":19728,"parent_record_offset":19700,"ancestor_record_offset":18732,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":463,"record_offset":22920,"parent_record_offset":22896,"ancestor_record_offset":18732,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":463,"record_offset":21324,"parent_record_offset":21300,"ancestor_record_offset":18732,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":463,"record_offset":21152,"parent_record_offset":21128,"ancestor_record_offset":18732,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":463,"record_offset":21096,"parent_record_offset":21072,"ancestor_record_offset":18732,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":463,"record_offset":19188,"parent_record_offset":19164,"ancestor_record_offset":18732,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":463,"record_offset":19016,"parent_record_offset":18988,"ancestor_record_offset":18732,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":463,"record_offset":19624,"parent_record_offset":19052,"ancestor_record_offset":18732,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":463,"record_offset":19268,"parent_record_offset":19244,"ancestor_record_offset":18732,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":463,"record_offset":23168,"parent_record_offset":23144,"ancestor_record_offset":18732,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":463,"record_offset":23060,"parent_record_offset":23036,"ancestor_record_offset":18732,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":463,"record_offset":19220,"parent_record_offset":19052,"ancestor_record_offset":18732,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":463,"record_offset":19244,"parent_record_offset":19220,"ancestor_record_offset":18732,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$2::load_index_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":463,"record_offset":21504,"parent_record_offset":21468,"ancestor_record_offset":18732,"inlinee":22210,"source_line":159,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::terrain_height::impl$3::sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/terrain_height.rs
//     evidence={"module":463,"record_offset":20956,"parent_record_offset":20924,"ancestor_record_offset":18732,"inlinee":22208,"source_line":175,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":463,"record_offset":23280,"parent_record_offset":23256,"ancestor_record_offset":18732,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":463,"record_offset":21356,"parent_record_offset":20988,"ancestor_record_offset":18732,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":463,"record_offset":21184,"parent_record_offset":20988,"ancestor_record_offset":18732,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":463,"record_offset":19472,"parent_record_offset":19052,"ancestor_record_offset":18732,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":463,"record_offset":19552,"parent_record_offset":19496,"ancestor_record_offset":18732,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":463,"record_offset":21208,"parent_record_offset":21184,"ancestor_record_offset":18732,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":463,"record_offset":21128,"parent_record_offset":20988,"ancestor_record_offset":18732,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":463,"record_offset":19164,"parent_record_offset":19052,"ancestor_record_offset":18732,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":463,"record_offset":21072,"parent_record_offset":20988,"ancestor_record_offset":18732,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":463,"record_offset":21300,"parent_record_offset":20988,"ancestor_record_offset":18732,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":463,"record_offset":21440,"parent_record_offset":20988,"ancestor_record_offset":18732,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$65::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":463,"record_offset":21408,"parent_record_offset":21384,"ancestor_record_offset":18732,"inlinee":22963,"source_line":1123,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":463,"record_offset":23232,"parent_record_offset":23208,"ancestor_record_offset":18732,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":463,"record_offset":22496,"parent_record_offset":22472,"ancestor_record_offset":18732,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":463,"record_offset":22520,"parent_record_offset":22496,"ancestor_record_offset":18732,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":463,"record_offset":23208,"parent_record_offset":22988,"ancestor_record_offset":18732,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":463,"record_offset":22552,"parent_record_offset":22520,"ancestor_record_offset":18732,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":463,"record_offset":23120,"parent_record_offset":23096,"ancestor_record_offset":18732,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":463,"record_offset":23096,"parent_record_offset":22988,"ancestor_record_offset":18732,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":463,"record_offset":23036,"parent_record_offset":23012,"ancestor_record_offset":18732,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":463,"record_offset":23416,"parent_record_offset":22628,"ancestor_record_offset":18732,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":463,"record_offset":23012,"parent_record_offset":22988,"ancestor_record_offset":18732,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":463,"record_offset":22604,"parent_record_offset":22580,"ancestor_record_offset":18732,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":463,"record_offset":22580,"parent_record_offset":22520,"ancestor_record_offset":18732,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":463,"record_offset":22628,"parent_record_offset":22604,"ancestor_record_offset":18732,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":463,"record_offset":22868,"parent_record_offset":22840,"ancestor_record_offset":18732,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":463,"record_offset":22896,"parent_record_offset":22868,"ancestor_record_offset":18732,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":463,"record_offset":22956,"parent_record_offset":22840,"ancestor_record_offset":18732,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":463,"record_offset":22840,"parent_record_offset":22628,"ancestor_record_offset":18732,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":463,"record_offset":22672,"parent_record_offset":22628,"ancestor_record_offset":18732,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":463,"record_offset":22988,"parent_record_offset":22628,"ancestor_record_offset":18732,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":463,"record_offset":22700,"parent_record_offset":22628,"ancestor_record_offset":18732,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":463,"record_offset":22784,"parent_record_offset":22628,"ancestor_record_offset":18732,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":463,"record_offset":23324,"parent_record_offset":22628,"ancestor_record_offset":18732,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":463,"record_offset":21536,"parent_record_offset":21504,"ancestor_record_offset":18732,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":463,"record_offset":22472,"parent_record_offset":22436,"ancestor_record_offset":18732,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":463,"record_offset":19584,"parent_record_offset":19552,"ancestor_record_offset":18732,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":463,"record_offset":23472,"parent_record_offset":22436,"ancestor_record_offset":18732,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":463,"record_offset":21468,"parent_record_offset":20988,"ancestor_record_offset":18732,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":463,"record_offset":20988,"parent_record_offset":20956,"ancestor_record_offset":18732,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6749
// 首条位置: {"module":463,"inlinee":26441}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":463,"record_offset":19416,"parent_record_offset":19052,"ancestor_record_offset":19052,"inlinee":6488,"source_line":263,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":19388,"parent_record_offset":19052,"ancestor_record_offset":19052,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":19136,"parent_record_offset":19052,"ancestor_record_offset":19052,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":463,"record_offset":19304,"parent_record_offset":19052,"ancestor_record_offset":19052,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":463,"record_offset":19624,"parent_record_offset":19052,"ancestor_record_offset":19052,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":463,"record_offset":19220,"parent_record_offset":19052,"ancestor_record_offset":19052,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":463,"record_offset":19472,"parent_record_offset":19052,"ancestor_record_offset":19052,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":463,"record_offset":19164,"parent_record_offset":19052,"ancestor_record_offset":19052,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":19788,"parent_record_offset":19764,"ancestor_record_offset":19052,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":463,"record_offset":19764,"parent_record_offset":19672,"ancestor_record_offset":19052,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":19700,"parent_record_offset":19672,"ancestor_record_offset":19052,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":463,"record_offset":19672,"parent_record_offset":19648,"ancestor_record_offset":19052,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":463,"record_offset":19440,"parent_record_offset":19416,"ancestor_record_offset":19052,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":463,"record_offset":19328,"parent_record_offset":19304,"ancestor_record_offset":19052,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":463,"record_offset":19352,"parent_record_offset":19328,"ancestor_record_offset":19052,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":463,"record_offset":19728,"parent_record_offset":19700,"ancestor_record_offset":19052,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":463,"record_offset":19188,"parent_record_offset":19164,"ancestor_record_offset":19052,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":463,"record_offset":19648,"parent_record_offset":19624,"ancestor_record_offset":19052,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":463,"record_offset":19268,"parent_record_offset":19244,"ancestor_record_offset":19052,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":463,"record_offset":19244,"parent_record_offset":19220,"ancestor_record_offset":19052,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":463,"record_offset":19552,"parent_record_offset":19496,"ancestor_record_offset":19052,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":463,"record_offset":19496,"parent_record_offset":19472,"ancestor_record_offset":19052,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":463,"record_offset":19584,"parent_record_offset":19552,"ancestor_record_offset":19052,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00463.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10560
// 首条位置: {"module":1447,"inlinee":66912}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1447,"record_offset":1016,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1447,"record_offset":2608,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1447,"record_offset":3436,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::in_progress_curve::InProgressCurve::raw_input_curve_length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/in_progress_curve.rs
//     evidence={"module":1447,"record_offset":748,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":61999,"source_line":140,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1447,"record_offset":720,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":1447,"record_offset":1544,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":22381,"source_line":33,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1447,"record_offset":3180,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1447,"record_offset":3124,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1447,"record_offset":2644,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1447,"record_offset":3960,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1447,"record_offset":4276,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":22389,"source_line":615,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1447,"record_offset":3368,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1447,"record_offset":804,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1447,"record_offset":3900,"parent_record_offset":580,"ancestor_record_offset":580,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1447,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":580,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1447,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":580,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1447,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":580,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1447,"record_offset":1312,"parent_record_offset":1220,"ancestor_record_offset":580,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1447,"record_offset":3608,"parent_record_offset":3508,"ancestor_record_offset":580,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1447,"record_offset":3540,"parent_record_offset":3508,"ancestor_record_offset":580,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1447,"record_offset":1220,"parent_record_offset":1192,"ancestor_record_offset":580,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1447,"record_offset":1192,"parent_record_offset":1164,"ancestor_record_offset":580,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1447,"record_offset":1432,"parent_record_offset":1192,"ancestor_record_offset":580,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1447,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":580,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1447,"record_offset":1124,"parent_record_offset":1088,"ancestor_record_offset":580,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1447,"record_offset":1088,"parent_record_offset":1052,"ancestor_record_offset":580,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1447,"record_offset":1052,"parent_record_offset":1016,"ancestor_record_offset":580,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1447,"record_offset":1164,"parent_record_offset":1124,"ancestor_record_offset":580,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1447,"record_offset":2700,"parent_record_offset":2644,"ancestor_record_offset":580,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1447,"record_offset":1256,"parent_record_offset":1220,"ancestor_record_offset":580,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1447,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":580,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1447,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":580,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1447,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":580,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1447,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":580,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1447,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":580,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1447,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":580,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1447,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":580,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1447,"record_offset":956,"parent_record_offset":804,"ancestor_record_offset":580,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1447,"record_offset":984,"parent_record_offset":804,"ancestor_record_offset":580,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1447,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":580,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1447,"record_offset":3572,"parent_record_offset":3540,"ancestor_record_offset":580,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1447,"record_offset":2024,"parent_record_offset":2000,"ancestor_record_offset":580,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1447,"record_offset":3052,"parent_record_offset":2992,"ancestor_record_offset":580,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1447,"record_offset":3080,"parent_record_offset":2992,"ancestor_record_offset":580,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1447,"record_offset":3024,"parent_record_offset":2992,"ancestor_record_offset":580,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1447,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":580,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1447,"record_offset":844,"parent_record_offset":804,"ancestor_record_offset":580,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1447,"record_offset":1472,"parent_record_offset":1124,"ancestor_record_offset":580,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1447,"record_offset":3460,"parent_record_offset":3436,"ancestor_record_offset":580,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1447,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":580,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1447,"record_offset":2164,"parent_record_offset":2140,"ancestor_record_offset":580,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1447,"record_offset":1388,"parent_record_offset":1360,"ancestor_record_offset":580,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1447,"record_offset":3396,"parent_record_offset":3368,"ancestor_record_offset":580,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1447,"record_offset":2732,"parent_record_offset":2700,"ancestor_record_offset":580,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1447,"record_offset":2764,"parent_record_offset":2732,"ancestor_record_offset":580,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1447,"record_offset":2384,"parent_record_offset":2360,"ancestor_record_offset":580,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1447,"record_offset":3264,"parent_record_offset":3232,"ancestor_record_offset":580,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1447,"record_offset":3232,"parent_record_offset":3204,"ancestor_record_offset":580,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1447,"record_offset":3204,"parent_record_offset":3180,"ancestor_record_offset":580,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1447,"record_offset":4552,"parent_record_offset":4528,"ancestor_record_offset":580,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1447,"record_offset":4472,"parent_record_offset":4276,"ancestor_record_offset":580,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1447,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":580,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1447,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":580,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1447,"record_offset":2336,"parent_record_offset":2312,"ancestor_record_offset":580,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1447,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":580,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1447,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":580,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1447,"record_offset":2312,"parent_record_offset":2092,"ancestor_record_offset":580,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1447,"record_offset":1656,"parent_record_offset":1628,"ancestor_record_offset":580,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1447,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":580,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1447,"record_offset":2200,"parent_record_offset":2092,"ancestor_record_offset":580,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1447,"record_offset":2140,"parent_record_offset":2116,"ancestor_record_offset":580,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1447,"record_offset":2520,"parent_record_offset":1732,"ancestor_record_offset":580,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1447,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":580,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1447,"record_offset":1708,"parent_record_offset":1684,"ancestor_record_offset":580,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1447,"record_offset":1684,"parent_record_offset":1628,"ancestor_record_offset":580,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1447,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":580,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1447,"record_offset":1972,"parent_record_offset":1944,"ancestor_record_offset":580,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1447,"record_offset":2000,"parent_record_offset":1972,"ancestor_record_offset":580,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1447,"record_offset":2060,"parent_record_offset":1944,"ancestor_record_offset":580,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1447,"record_offset":1944,"parent_record_offset":1732,"ancestor_record_offset":580,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1447,"record_offset":1776,"parent_record_offset":1732,"ancestor_record_offset":580,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1447,"record_offset":2092,"parent_record_offset":1732,"ancestor_record_offset":580,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1447,"record_offset":1804,"parent_record_offset":1732,"ancestor_record_offset":580,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1447,"record_offset":1888,"parent_record_offset":1732,"ancestor_record_offset":580,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1447,"record_offset":2428,"parent_record_offset":1732,"ancestor_record_offset":580,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1447,"record_offset":1580,"parent_record_offset":1544,"ancestor_record_offset":580,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1447,"record_offset":3296,"parent_record_offset":3264,"ancestor_record_offset":580,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_curve_segment_from_u
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1447,"record_offset":4316,"parent_record_offset":4276,"ancestor_record_offset":580,"inlinee":22390,"source_line":519,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1447,"record_offset":3988,"parent_record_offset":3960,"ancestor_record_offset":580,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1447,"record_offset":2576,"parent_record_offset":1544,"ancestor_record_offset":580,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1447,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":580,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1447,"record_offset":2672,"parent_record_offset":2644,"ancestor_record_offset":580,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1447,"record_offset":2808,"parent_record_offset":2644,"ancestor_record_offset":580,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1447,"record_offset":4528,"parent_record_offset":4276,"ancestor_record_offset":580,"inlinee":22391,"source_line":65,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1447,"record_offset":2944,"parent_record_offset":2644,"ancestor_record_offset":580,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF32D
// 首条位置: {"module":1336,"inlinee":62253}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1336,"record_offset":37548,"parent_record_offset":37208,"ancestor_record_offset":37208,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01336.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1336,"record_offset":37244,"parent_record_offset":37208,"ancestor_record_offset":37208,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/01336.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1336,"record_offset":37296,"parent_record_offset":37268,"ancestor_record_offset":37208,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01336.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1336,"record_offset":37324,"parent_record_offset":37268,"ancestor_record_offset":37208,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01336.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1336,"record_offset":37268,"parent_record_offset":37244,"ancestor_record_offset":37208,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01336.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1336,"record_offset":37356,"parent_record_offset":37324,"ancestor_record_offset":37208,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01336.jsonl.gz"}
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
// 完整结构及原始类型记录: snapping_to_segments.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>", "constructor": "alloc::slice::impl$0::sort_by::closure_env$0", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$0", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$1<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$1", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$2<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$2", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::slice::stable_sort::closure_env$3<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "alloc::slice::stable_sort::closure_env$3", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::merge_sort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>,alloc::slice::stable_sort::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> > >", "constructor": "core::slice::sort::merge_sort", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>", "alloc::slice::stable_sort::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "alloc::slice::stable_sort::closure_env$1<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "alloc::slice::stable_sort::closure_env$2<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "alloc::slice::stable_sort::closure_env$3<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::sort::insertion_sort_shift_left<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >", "constructor": "core::slice::sort::insertion_sort_shift_left", "arguments": ["country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal", "alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: snapping_to_segments.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: snapping_to_segments.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<alloc::string::String> >,ref$<ref$<alloc::string::String> >)
// 候选参数类型: ["ref$<ref$<alloc::string::String> >", "ref$<ref$<alloc::string::String> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell)
// 候选参数类型: ["country_core::render::pass_builder::draw_builder::DrawBuilder", "ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>", "bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<std::path::PathBuf>,ref$<std::path::PathBuf>)
// 候选参数类型: ["ref$<std::path::PathBuf>", "ref$<std::path::PathBuf>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to
// 信息 ID: 4c879e7ac5df33070f009bc9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_to_segments::snapping_to_segments; module=463, inline_record=19052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure$0
// 信息 ID: 9a7c8becd0e480b9acc8f903
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> > >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> > >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0>; module=1447, inline_record=580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure$1
// 信息 ID: 72740adcfb9b36ed1b031ea0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::merge_sort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>,alloc::slice::stable_sort::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::merge_sort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>,alloc::slice::stable_sort::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::merge_sort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>,alloc::slice::stable_sort::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::merge_sort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>,alloc::slice::stable_sort::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::merge_sort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>,alloc::slice::stable_sort::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1 <- core::slice::sort::insertion_sort_shift_left<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>,alloc::slice::stable_sort::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> > >; module=1336, inline_record=37208; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>,alloc::slice::stable_sort::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> > >; module=1336, inline_record=37652; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>,alloc::slice::stable_sort::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> > >; module=1336, inline_record=38116; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>,alloc::slice::stable_sort::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> > >; module=1336, inline_record=39508; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1>,alloc::slice::stable_sort::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$1<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$2<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >,alloc::slice::stable_sort::closure_env$3<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> > >; module=1336, inline_record=40156; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=1336, inline_record=118932; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::SnapProposal,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$1> >; module=1336, inline_record=119556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::snapping::snapping_to_segments::snapping_to_segments
// 信息 ID: 9e2fa6bb0b289cc2426487cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 0d0da29d4c5914b244ed0164: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)> > @ 0xc9fae -> 0x641b30 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>)> @ 0x2f433c -> 0x641b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
