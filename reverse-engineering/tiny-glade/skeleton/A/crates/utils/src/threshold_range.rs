// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/threshold_range.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 44d08e679eabfba5736badba798a7aae36dbd52626f7a715b422892cc2a34e67
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: threshold_range.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 374, 1234
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::threshold_range::threshold_range
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x545D
// 首条位置: {"module":374,"inlinee":21597}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/calculate_wall_path_segmentation.rs
//     evidence={"module":374,"record_offset":11372,"parent_record_offset":11324,"ancestor_record_offset":11324,"inlinee":21598,"source_line":109,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":374,"record_offset":11476,"parent_record_offset":11452,"ancestor_record_offset":11324,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":11452,"parent_record_offset":11412,"ancestor_record_offset":11324,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":374,"record_offset":11412,"parent_record_offset":11372,"ancestor_record_offset":11324,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":374,"record_offset":11540,"parent_record_offset":11372,"ancestor_record_offset":11324,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":374,"record_offset":11508,"parent_record_offset":11412,"ancestor_record_offset":11324,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1234,"record_offset":260,"rva":13351184,"va":5382060304}
// 编译实例 RVAs: 0xcbb910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12247293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382060304,"raw_hex":"e8031f45ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5382060840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cc91d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t54865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382060304,"raw_hex":"e851d60000","size":5,"soft_fail":false,"target_va":5382115792,"va":5382060922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10756102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382060304,"raw_hex":"e8fadf5bff","size":5,"soft_fail":false,"target_va":5371304880,"va":5382060977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20919186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382060304,"raw_hex":"e892333f01","size":5,"soft_fail":false,"target_va":5402980304,"va":5382061113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20919184","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382060304,"raw_hex":"e890333f01","size":5,"soft_fail":false,"target_va":5402980320,"va":5382061131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":1234,"record_offset":712,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59731,"source_line":35,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1234,"record_offset":1364,"parent_record_offset":1332,"ancestor_record_offset":260,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1234,"record_offset":1332,"parent_record_offset":1296,"ancestor_record_offset":260,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1234,"record_offset":1296,"parent_record_offset":712,"ancestor_record_offset":260,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1234,"record_offset":1244,"parent_record_offset":1216,"ancestor_record_offset":260,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1234,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":260,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1234,"record_offset":996,"parent_record_offset":960,"ancestor_record_offset":260,"inlinee":8603,"source_line":47,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1234,"record_offset":960,"parent_record_offset":916,"ancestor_record_offset":260,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1234,"record_offset":1116,"parent_record_offset":996,"ancestor_record_offset":260,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1234,"record_offset":840,"parent_record_offset":712,"ancestor_record_offset":260,"inlinee":5289,"source_line":1660,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1234,"record_offset":1080,"parent_record_offset":1032,"ancestor_record_offset":260,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1234,"record_offset":1396,"parent_record_offset":1296,"ancestor_record_offset":260,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1234,"record_offset":768,"parent_record_offset":712,"ancestor_record_offset":260,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1234,"record_offset":796,"parent_record_offset":768,"ancestor_record_offset":260,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1234,"record_offset":1032,"parent_record_offset":996,"ancestor_record_offset":260,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1234,"record_offset":1216,"parent_record_offset":916,"ancestor_record_offset":260,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::min
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1234,"record_offset":1152,"parent_record_offset":916,"ancestor_record_offset":260,"inlinee":23218,"source_line":173,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::construct_fences::process_segment::emit_fences::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/construct_fences/process_segment.rs
//     evidence={"module":1234,"record_offset":868,"parent_record_offset":840,"ancestor_record_offset":260,"inlinee":59733,"source_line":40,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::bounding_box_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1234,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":260,"inlinee":27397,"source_line":875,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::aabb::Aabb2::from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":1234,"record_offset":916,"parent_record_offset":892,"ancestor_record_offset":260,"inlinee":23220,"source_line":91,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8>
// 记录形式: compiled_procedure; instances=1
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1234,"record_offset":1436,"rva":13352032,"va":5382061152}
// 编译实例 RVAs: 0xcbbc60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c971d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-151018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382061152,"raw_hex":"e816b2fdff","size":5,"soft_fail":false,"target_va":5381910992,"va":5382062005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: threshold_range.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21532019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382061152,"raw_hex":"e8738d4801","size":5,"soft_fail":false,"target_va":5403593360,"va":5382061336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10752141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382061152,"raw_hex":"e873ef5bff","size":5,"soft_fail":false,"target_va":5371309552,"va":5382061688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::segmentation::calculate_wall_path_segmentation::process_segment
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-8155308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382061152,"raw_hex":"e8548f83ff","size":5,"soft_fail":false,"target_va":5373906368,"va":5382061671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::loop_range::LoopRange::subslice
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-32796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382061152,"raw_hex":"e8e47fffff","size":5,"soft_fail":false,"target_va":5382028752,"va":5382061543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$7
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs
//     evidence={"module":1234,"record_offset":2080,"parent_record_offset":1436,"ancestor_record_offset":1436,"inlinee":59735,"source_line":115,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure$8
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs
//     evidence={"module":1234,"record_offset":1856,"parent_record_offset":1436,"ancestor_record_offset":1436,"inlinee":59734,"source_line":134,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1234,"record_offset":1912,"parent_record_offset":1856,"ancestor_record_offset":1436,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1234,"record_offset":1960,"parent_record_offset":1912,"ancestor_record_offset":1436,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1234,"record_offset":1988,"parent_record_offset":1912,"ancestor_record_offset":1436,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01234.jsonl.gz"}
//   [pdb_inline_transitive] utils::loop_range::LoopRange::map_slice
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":1234,"record_offset":2020,"parent_record_offset":1856,"ancestor_record_offset":1436,"inlinee":21479,"source_line":174,"archive":"_pdb/A/modules/01234.jsonl.gz"}
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
// 完整结构及原始类型记录: threshold_range.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: threshold_range.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: threshold_range.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: utils::threshold_range::threshold_range
// 信息 ID: d9d6ccc95b747080c26b3e83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation; module=374, inline_record=11324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::threshold_range::threshold_range<system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>,system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0> >
// 信息 ID: d0f42713b0acb15af861112f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0 -> ["system_wall_constructor::construct_fences::construct_fences::closure_env$0"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1 -> ["system_wall_constructor::construct_fences::construct_fences::closure_env$0"]; 字段布局 = unknown。
// [类型关系] utils::threshold_range::threshold_range -> ["system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$0<system_wall_constructor::construct_fences::construct_fences::closure_env$0>", "system_wall_constructor::construct_fences::process_segment::emit_fences::closure_env$1<system_wall_constructor::construct_fences::construct_fences::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::process_segment::emit_fences<system_wall_constructor::construct_fences::construct_fences::closure_env$0> @ 0xcc90d1 -> 0xcbb910 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::threshold_range::threshold_range<system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7,system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8>
// 信息 ID: bddf5348c4044f298e7b8b30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::threshold_range::threshold_range -> ["system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$7", "system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs::closure_env$8"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs @ 0x4dc8d5 -> 0xcbbc60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
