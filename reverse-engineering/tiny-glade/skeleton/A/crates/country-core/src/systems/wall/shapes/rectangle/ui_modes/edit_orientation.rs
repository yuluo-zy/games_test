// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: bf12b3a2946b999253f89a10c0218d2ffa7bfafabb738ed11f2a57fb7fb83b9e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: edit_orientation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 389, 401, 481, 1308, 1390
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
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
// 函数符号: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1390,"record_offset":44004,"rva":16913296,"va":5385622416}
// 编译实例 RVAs: 0x1021390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140685d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10075042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385622416,"raw_hex":"e85e4466ff","size":5,"soft_fail":false,"target_va":5375548688,"va":5385623725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d7d6f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2768580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385622416,"raw_hex":"e83cc1d5ff","size":5,"soft_fail":false,"target_va":5382854384,"va":5385622959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8abc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1665105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385622416,"raw_hex":"e8af97e6ff","size":5,"soft_fail":false,"target_va":5383957440,"va":5385622540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t17355518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385622416,"raw_hex":"e8fed20801","size":5,"soft_fail":false,"target_va":5402979344,"va":5385623821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11337442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385622416,"raw_hex":"e81e0153ff","size":5,"soft_fail":false,"target_va":5374286080,"va":5385623517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7664281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385622416,"raw_hex":"e8670d8bff","size":5,"soft_fail":false,"target_va":5377958496,"va":5385622772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7664103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385622416,"raw_hex":"e8190e8bff","size":5,"soft_fail":false,"target_va":5377958704,"va":5385622802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7666899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385622416,"raw_hex":"e82d038bff","size":5,"soft_fail":false,"target_va":5377955632,"va":5385622526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat3
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7667572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385622416,"raw_hex":"e88c008bff","size":5,"soft_fail":false,"target_va":5377955488,"va":5385623055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1390,"record_offset":45120,"parent_record_offset":44004,"ancestor_record_offset":44004,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1390,"record_offset":45036,"parent_record_offset":44004,"ancestor_record_offset":44004,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":1390,"record_offset":45848,"parent_record_offset":44004,"ancestor_record_offset":44004,"inlinee":24894,"source_line":87,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1390,"record_offset":45468,"parent_record_offset":44004,"ancestor_record_offset":44004,"inlinee":36059,"source_line":253,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1390,"record_offset":45008,"parent_record_offset":44004,"ancestor_record_offset":44004,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$33::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1390,"record_offset":44944,"parent_record_offset":44004,"ancestor_record_offset":44004,"inlinee":48695,"source_line":1205,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1390,"record_offset":44228,"parent_record_offset":44004,"ancestor_record_offset":44004,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1390,"record_offset":44684,"parent_record_offset":44004,"ancestor_record_offset":44004,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1390,"record_offset":44284,"parent_record_offset":44228,"ancestor_record_offset":44004,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1390,"record_offset":45144,"parent_record_offset":45120,"ancestor_record_offset":44004,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1390,"record_offset":45168,"parent_record_offset":45144,"ancestor_record_offset":44004,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1390,"record_offset":44612,"parent_record_offset":44552,"ancestor_record_offset":44004,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1390,"record_offset":44640,"parent_record_offset":44552,"ancestor_record_offset":44004,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$286::neg
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1390,"record_offset":44972,"parent_record_offset":44944,"ancestor_record_offset":44004,"inlinee":14668,"source_line":694,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1390,"record_offset":44584,"parent_record_offset":44552,"ancestor_record_offset":44004,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1390,"record_offset":45084,"parent_record_offset":45060,"ancestor_record_offset":44004,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1390,"record_offset":44312,"parent_record_offset":44284,"ancestor_record_offset":44004,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1390,"record_offset":44340,"parent_record_offset":44312,"ancestor_record_offset":44004,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1390,"record_offset":45060,"parent_record_offset":45036,"ancestor_record_offset":44004,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1390,"record_offset":45588,"parent_record_offset":45492,"ancestor_record_offset":44004,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1390,"record_offset":45492,"parent_record_offset":45468,"ancestor_record_offset":44004,"inlinee":26368,"source_line":192,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1390,"record_offset":45812,"parent_record_offset":45492,"ancestor_record_offset":44004,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1390,"record_offset":44528,"parent_record_offset":44504,"ancestor_record_offset":44004,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1390,"record_offset":44552,"parent_record_offset":44528,"ancestor_record_offset":44004,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1390,"record_offset":45612,"parent_record_offset":45588,"ancestor_record_offset":44004,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1390,"record_offset":44256,"parent_record_offset":44228,"ancestor_record_offset":44004,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1390,"record_offset":44380,"parent_record_offset":44228,"ancestor_record_offset":44004,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1390,"record_offset":44504,"parent_record_offset":44228,"ancestor_record_offset":44004,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$0::to_widget_name_hash
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A21
// 首条位置: {"module":401,"inlinee":23073}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A1D
// 首条位置: {"module":401,"inlinee":23069}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5606
// 首条位置: {"module":389,"inlinee":22022}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1308,"record_offset":12132,"parent_record_offset":12020,"ancestor_record_offset":12020,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1308,"record_offset":12076,"parent_record_offset":12020,"ancestor_record_offset":12020,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1308,"record_offset":12100,"parent_record_offset":12076,"ancestor_record_offset":12020,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1308,"record_offset":12156,"parent_record_offset":12132,"ancestor_record_offset":12020,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier
// 记录形式: compiled_procedure; instances=1
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1390,"record_offset":46060,"rva":16914800,"va":5385623920}
// 编译实例 RVAs: 0x1021970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce150
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13974812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385623920,"raw_hex":"e8e4c22aff","size":5,"soft_fail":false,"target_va":5371650384,"va":5385625191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12874888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385623920,"raw_hex":"e8788b3bff","size":5,"soft_fail":false,"target_va":5372749696,"va":5385624579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12875116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385623920,"raw_hex":"e8948a3bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5385624823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405caa20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10842807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385623920,"raw_hex":"e8498d5aff","size":5,"soft_fail":false,"target_va":5374781984,"va":5385624786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8680116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385623920,"raw_hex":"e84c8d7bff","size":5,"soft_fail":false,"target_va":5376943968,"va":5385624079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8680050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385623920,"raw_hex":"e88e8d7bff","size":5,"soft_fail":false,"target_va":5376944000,"va":5385624045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17968616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385623920,"raw_hex":"e8e82d1201","size":5,"soft_fail":false,"target_va":5403593360,"va":5385624739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t17353173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385623920,"raw_hex":"e8d5c90801","size":5,"soft_fail":false,"target_va":5402978144,"va":5385624966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8094623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385623920,"raw_hex":"e8617c84ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5385624506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8097887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385623920,"raw_hex":"e8a16f84ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5385625002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7665565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385623920,"raw_hex":"e863088bff","size":5,"soft_fail":false,"target_va":5377958496,"va":5385624056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7668382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385623920,"raw_hex":"e862fd8aff","size":5,"soft_fail":false,"target_va":5377955632,"va":5385624009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-8682928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385623920,"raw_hex":"e850827bff","size":5,"soft_fail":false,"target_va":5376941312,"va":5385624235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1390,"record_offset":46716,"parent_record_offset":46060,"ancestor_record_offset":46060,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1390,"record_offset":46800,"parent_record_offset":46060,"ancestor_record_offset":46060,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1390,"record_offset":46508,"parent_record_offset":46060,"ancestor_record_offset":46060,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1390,"record_offset":47776,"parent_record_offset":46060,"ancestor_record_offset":46060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1390,"record_offset":46452,"parent_record_offset":46060,"ancestor_record_offset":46060,"inlinee":22964,"source_line":238,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::hovered
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1390,"record_offset":46292,"parent_record_offset":46060,"ancestor_record_offset":46060,"inlinee":23074,"source_line":234,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1390,"record_offset":46352,"parent_record_offset":46060,"ancestor_record_offset":46060,"inlinee":23070,"source_line":243,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1390,"record_offset":47016,"parent_record_offset":46060,"ancestor_record_offset":46060,"inlinee":22967,"source_line":102,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::send
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":1390,"record_offset":47748,"parent_record_offset":46060,"ancestor_record_offset":46060,"inlinee":22969,"source_line":24,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1390,"record_offset":46912,"parent_record_offset":46888,"ancestor_record_offset":46060,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1390,"record_offset":46936,"parent_record_offset":46912,"ancestor_record_offset":46060,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1390,"record_offset":47928,"parent_record_offset":47904,"ancestor_record_offset":46060,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1390,"record_offset":46856,"parent_record_offset":46828,"ancestor_record_offset":46060,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1390,"record_offset":46888,"parent_record_offset":46856,"ancestor_record_offset":46060,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1390,"record_offset":47904,"parent_record_offset":47848,"ancestor_record_offset":46060,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1390,"record_offset":46828,"parent_record_offset":46800,"ancestor_record_offset":46060,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1390,"record_offset":46764,"parent_record_offset":46740,"ancestor_record_offset":46060,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1390,"record_offset":47876,"parent_record_offset":47848,"ancestor_record_offset":46060,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1390,"record_offset":47312,"parent_record_offset":47284,"ancestor_record_offset":46060,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1390,"record_offset":47284,"parent_record_offset":47256,"ancestor_record_offset":46060,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1390,"record_offset":47468,"parent_record_offset":47284,"ancestor_record_offset":46060,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1390,"record_offset":47848,"parent_record_offset":47824,"ancestor_record_offset":46060,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1390,"record_offset":47220,"parent_record_offset":47192,"ancestor_record_offset":46060,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1390,"record_offset":47192,"parent_record_offset":47164,"ancestor_record_offset":46060,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1390,"record_offset":47136,"parent_record_offset":47108,"ancestor_record_offset":46060,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1390,"record_offset":47164,"parent_record_offset":47136,"ancestor_record_offset":46060,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1390,"record_offset":47108,"parent_record_offset":47080,"ancestor_record_offset":46060,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1390,"record_offset":47080,"parent_record_offset":47052,"ancestor_record_offset":46060,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1390,"record_offset":47052,"parent_record_offset":47016,"ancestor_record_offset":46060,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1390,"record_offset":47616,"parent_record_offset":47592,"ancestor_record_offset":46060,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1390,"record_offset":47256,"parent_record_offset":47220,"ancestor_record_offset":46060,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1390,"record_offset":46596,"parent_record_offset":46532,"ancestor_record_offset":46060,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1390,"record_offset":47532,"parent_record_offset":47508,"ancestor_record_offset":46060,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1390,"record_offset":46568,"parent_record_offset":46532,"ancestor_record_offset":46060,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1390,"record_offset":46532,"parent_record_offset":46508,"ancestor_record_offset":46060,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1390,"record_offset":46740,"parent_record_offset":46716,"ancestor_record_offset":46060,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1390,"record_offset":46668,"parent_record_offset":46644,"ancestor_record_offset":46060,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1390,"record_offset":47508,"parent_record_offset":47220,"ancestor_record_offset":46060,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1390,"record_offset":47592,"parent_record_offset":47016,"ancestor_record_offset":46060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1390,"record_offset":46964,"parent_record_offset":46936,"ancestor_record_offset":46060,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1390,"record_offset":46620,"parent_record_offset":46596,"ancestor_record_offset":46060,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1390,"record_offset":46644,"parent_record_offset":46620,"ancestor_record_offset":46060,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1390,"record_offset":47652,"parent_record_offset":47616,"ancestor_record_offset":46060,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1390,"record_offset":47676,"parent_record_offset":47652,"ancestor_record_offset":46060,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1390,"record_offset":47708,"parent_record_offset":47616,"ancestor_record_offset":46060,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1390,"record_offset":46476,"parent_record_offset":46452,"ancestor_record_offset":46060,"inlinee":22965,"source_line":134,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::hovered
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1390,"record_offset":46316,"parent_record_offset":46292,"ancestor_record_offset":46060,"inlinee":23075,"source_line":117,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::intersecting
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1390,"record_offset":46380,"parent_record_offset":46352,"ancestor_record_offset":46060,"inlinee":22961,"source_line":152,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":1390,"record_offset":46416,"parent_record_offset":46380,"ancestor_record_offset":46060,"inlinee":22151,"source_line":10,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::signifier::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 201
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1390,"record_offset":48180,"rva":16916112,"va":5385625232}
// 编译实例 RVAs: 0x1021e90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12875608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385625232,"raw_hex":"e8a8883bff","size":5,"soft_fail":false,"target_va":5372749696,"va":5385625299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12876096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385625232,"raw_hex":"e8c0863bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5385625803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140652f90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10285445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385625232,"raw_hex":"e87b0e63ff","size":5,"soft_fail":false,"target_va":5375340432,"va":5385625872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17967971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385625232,"raw_hex":"e8632b1201","size":5,"soft_fail":false,"target_va":5403593360,"va":5385625384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t17352428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385625232,"raw_hex":"e8ecc60801","size":5,"soft_fail":false,"target_va":5402978144,"va":5385625711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-8098618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385625232,"raw_hex":"e8c66c84ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5385625733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10268752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385625232,"raw_hex":"e8b04f63ff","size":5,"soft_fail":false,"target_va":5375356896,"va":5385625643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1390,"record_offset":48392,"parent_record_offset":48180,"ancestor_record_offset":48180,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1390,"record_offset":49740,"parent_record_offset":48180,"ancestor_record_offset":48180,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1390,"record_offset":48540,"parent_record_offset":48180,"ancestor_record_offset":48180,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1390,"record_offset":49824,"parent_record_offset":48180,"ancestor_record_offset":48180,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1390,"record_offset":49108,"parent_record_offset":48180,"ancestor_record_offset":48180,"inlinee":22973,"source_line":143,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1390,"record_offset":49712,"parent_record_offset":48180,"ancestor_record_offset":48180,"inlinee":26370,"source_line":179,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::set_ui_pass
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":1390,"record_offset":49684,"parent_record_offset":48180,"ancestor_record_offset":48180,"inlinee":22975,"source_line":163,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1390,"record_offset":48448,"parent_record_offset":48424,"ancestor_record_offset":48180,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1390,"record_offset":48472,"parent_record_offset":48448,"ancestor_record_offset":48180,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1390,"record_offset":49896,"parent_record_offset":49872,"ancestor_record_offset":48180,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1390,"record_offset":48424,"parent_record_offset":48392,"ancestor_record_offset":48180,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1390,"record_offset":49872,"parent_record_offset":49848,"ancestor_record_offset":48180,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1390,"record_offset":49848,"parent_record_offset":49824,"ancestor_record_offset":48180,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1390,"record_offset":48828,"parent_record_offset":48800,"ancestor_record_offset":48180,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1390,"record_offset":48800,"parent_record_offset":48772,"ancestor_record_offset":48180,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1390,"record_offset":48980,"parent_record_offset":48800,"ancestor_record_offset":48180,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1390,"record_offset":48736,"parent_record_offset":48708,"ancestor_record_offset":48180,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1390,"record_offset":49764,"parent_record_offset":49740,"ancestor_record_offset":48180,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1390,"record_offset":48708,"parent_record_offset":48680,"ancestor_record_offset":48180,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1390,"record_offset":48652,"parent_record_offset":48624,"ancestor_record_offset":48180,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1390,"record_offset":48680,"parent_record_offset":48652,"ancestor_record_offset":48180,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1390,"record_offset":48624,"parent_record_offset":48596,"ancestor_record_offset":48180,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1390,"record_offset":48596,"parent_record_offset":48568,"ancestor_record_offset":48180,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1390,"record_offset":48568,"parent_record_offset":48540,"ancestor_record_offset":48180,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1390,"record_offset":49788,"parent_record_offset":49764,"ancestor_record_offset":48180,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1390,"record_offset":48772,"parent_record_offset":48736,"ancestor_record_offset":48180,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1390,"record_offset":49044,"parent_record_offset":49020,"ancestor_record_offset":48180,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1390,"record_offset":49020,"parent_record_offset":48736,"ancestor_record_offset":48180,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01390.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1390,"record_offset":48496,"parent_record_offset":48472,"ancestor_record_offset":48180,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01390.jsonl.gz"}
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
// 完整结构及原始类型记录: edit_orientation.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "ref$<country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit>", "constructor": "ref$", "arguments": ["country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::systems::ui::resources::ui_state::UiState::add<ref$<country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit> >", "constructor": "country_core::systems::ui::resources::ui_state::UiState::add", "arguments": ["ref$<country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: edit_orientation.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: edit_orientation.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 10; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::new
// 信息 ID: e451bdd77334323dc6b2a71e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate; module=401, inline_record=9256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier
// 信息 ID: 609867d995590af7005905b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0x550a8a -> 0x1021970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget
// 信息 ID: 2159c9a9342f246b3ecf6665
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0x550a3e -> 0x1021390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$0::to_widget_name_hash
// 信息 ID: 1dc6e9acd20b4ee7a1201130
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate; module=401, inline_record=9600; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::resources::ui_state::UiState::add<ref$<country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit> >; module=481, inline_record=33296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::signifier::closure$0
// 信息 ID: daa3740be7d1fef118f9648b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::signifier", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::signifier::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::signifier::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::signifier::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x101b132 -> 0x1021e90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure$0
// 信息 ID: 96e89c8e0e5d3a76ee670bd7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::intersect::Triangle,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0>; module=1308, inline_record=12020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
