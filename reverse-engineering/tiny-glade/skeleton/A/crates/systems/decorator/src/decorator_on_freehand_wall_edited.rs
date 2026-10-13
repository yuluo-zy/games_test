// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/decorator_on_freehand_wall_edited.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f1110e13a566c05331832e06273037fbc86372f53009e700755d18a0ad3c7450
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decorator_on_freehand_wall_edited.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 379
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
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::decorator_on_freehand_wall_edited::decorator_on_freehand_wall_edited
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":379,"record_offset":256,"rva":5227424,"va":5373936544}
// 编译实例 RVAs: 0x4fc3a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: decorator_on_freehand_wall_edited.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4323483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373936544,"raw_hex":"e86507beff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373936998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b9b40
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: decorator_on_freehand_wall_edited.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t775739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373936544,"raw_hex":"e83bd60b00","size":5,"soft_fail":false,"target_va":5374712640,"va":5373936896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_on_freehand_wall_edited.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3006653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373936544,"raw_hex":"e8bde02d00","size":5,"soft_fail":false,"target_va":5376944000,"va":5373937342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29041346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373936544,"raw_hex":"e8c222bb01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373937529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29041594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373936544,"raw_hex":"e8ba23bb01","size":5,"soft_fail":false,"target_va":5402979152,"va":5373937553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29041834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373936544,"raw_hex":"e8aa24bb01","size":5,"soft_fail":false,"target_va":5402979344,"va":5373937505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1497738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373936544,"raw_hex":"e88ada1600","size":5,"soft_fail":false,"target_va":5375434976,"va":5373937233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::from_curve_internal_coord
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8319060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373936544,"raw_hex":"e854f07e00","size":5,"soft_fail":false,"target_va":5382256448,"va":5373937383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":379,"record_offset":1364,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":379,"record_offset":1308,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":379,"record_offset":456,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":379,"record_offset":996,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":379,"record_offset":2580,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":379,"record_offset":1392,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":379,"record_offset":1500,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":379,"record_offset":2616,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":379,"record_offset":2648,"parent_record_offset":2616,"ancestor_record_offset":256,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":379,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":256,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":379,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":256,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":379,"record_offset":520,"parent_record_offset":480,"ancestor_record_offset":256,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":379,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":256,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":379,"record_offset":480,"parent_record_offset":456,"ancestor_record_offset":256,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":379,"record_offset":2308,"parent_record_offset":2284,"ancestor_record_offset":256,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":379,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":256,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":379,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":256,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":379,"record_offset":1800,"parent_record_offset":1776,"ancestor_record_offset":256,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":379,"record_offset":1072,"parent_record_offset":1048,"ancestor_record_offset":256,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":379,"record_offset":1048,"parent_record_offset":1020,"ancestor_record_offset":256,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":379,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":256,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":379,"record_offset":588,"parent_record_offset":544,"ancestor_record_offset":256,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":379,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":256,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":379,"record_offset":2976,"parent_record_offset":2916,"ancestor_record_offset":256,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":379,"record_offset":3004,"parent_record_offset":2916,"ancestor_record_offset":256,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":379,"record_offset":2948,"parent_record_offset":2916,"ancestor_record_offset":256,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":379,"record_offset":1096,"parent_record_offset":1072,"ancestor_record_offset":256,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":379,"record_offset":1128,"parent_record_offset":1048,"ancestor_record_offset":256,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":379,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":256,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":379,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":256,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":379,"record_offset":2112,"parent_record_offset":2088,"ancestor_record_offset":256,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":379,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":256,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":379,"record_offset":1452,"parent_record_offset":1392,"ancestor_record_offset":256,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":379,"record_offset":652,"parent_record_offset":544,"ancestor_record_offset":256,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":379,"record_offset":840,"parent_record_offset":544,"ancestor_record_offset":256,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":379,"record_offset":2676,"parent_record_offset":2648,"ancestor_record_offset":256,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":379,"record_offset":2704,"parent_record_offset":2676,"ancestor_record_offset":256,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":379,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":256,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":379,"record_offset":688,"parent_record_offset":652,"ancestor_record_offset":256,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":379,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":256,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":379,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":256,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":379,"record_offset":1204,"parent_record_offset":1152,"ancestor_record_offset":256,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":379,"record_offset":2332,"parent_record_offset":2308,"ancestor_record_offset":256,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":379,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":256,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":379,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":256,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":379,"record_offset":2284,"parent_record_offset":2260,"ancestor_record_offset":256,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":379,"record_offset":1564,"parent_record_offset":1532,"ancestor_record_offset":256,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":379,"record_offset":1532,"parent_record_offset":1500,"ancestor_record_offset":256,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":379,"record_offset":2260,"parent_record_offset":2040,"ancestor_record_offset":256,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":379,"record_offset":1604,"parent_record_offset":1564,"ancestor_record_offset":256,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":379,"record_offset":2172,"parent_record_offset":2148,"ancestor_record_offset":256,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":379,"record_offset":2148,"parent_record_offset":2040,"ancestor_record_offset":256,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":379,"record_offset":2088,"parent_record_offset":2064,"ancestor_record_offset":256,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":379,"record_offset":2468,"parent_record_offset":1680,"ancestor_record_offset":256,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":379,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":256,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":379,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":256,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":379,"record_offset":1632,"parent_record_offset":1564,"ancestor_record_offset":256,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":379,"record_offset":2512,"parent_record_offset":1564,"ancestor_record_offset":256,"inlinee":17280,"source_line":1571,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":379,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":256,"inlinee":17281,"source_line":1565,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":379,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":256,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":379,"record_offset":1920,"parent_record_offset":1892,"ancestor_record_offset":256,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":379,"record_offset":1948,"parent_record_offset":1920,"ancestor_record_offset":256,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":379,"record_offset":2008,"parent_record_offset":1892,"ancestor_record_offset":256,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":379,"record_offset":1892,"parent_record_offset":1680,"ancestor_record_offset":256,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":379,"record_offset":1724,"parent_record_offset":1680,"ancestor_record_offset":256,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":379,"record_offset":2040,"parent_record_offset":1680,"ancestor_record_offset":256,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":379,"record_offset":1752,"parent_record_offset":1680,"ancestor_record_offset":256,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":379,"record_offset":1836,"parent_record_offset":1680,"ancestor_record_offset":256,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":379,"record_offset":2376,"parent_record_offset":1680,"ancestor_record_offset":256,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":379,"record_offset":2744,"parent_record_offset":2616,"ancestor_record_offset":256,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":379,"record_offset":2868,"parent_record_offset":2616,"ancestor_record_offset":256,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00379.jsonl.gz"}
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
// 完整结构及原始类型记录: decorator_on_freehand_wall_edited.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: decorator_on_freehand_wall_edited.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: decorator_on_freehand_wall_edited.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::resources::events::OnFreehandWallEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::EventReader<country_core::resources::events::OnFreehandWallEdited>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::OnFreehandWallEdited>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>

// ------------------------------------------------------------------
// 函数: system_decorator::decorator_on_freehand_wall_edited::decorator_on_freehand_wall_edited
// 信息 ID: 3c83024cbe59d017a490459b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c00d54b0c34380be537bf0ad: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::resources::events::OnFreehandWallEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::resources::events::OnFreehandWallEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::resources::events::OnFreehandWallEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>)> > @ 0xbcc31 -> 0x4fc3a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::resources::events::OnFreehandWallEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventReader<country_core::resources::events::OnFreehandWallEdited>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>)> @ 0x2dbf5f -> 0x4fc3a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
