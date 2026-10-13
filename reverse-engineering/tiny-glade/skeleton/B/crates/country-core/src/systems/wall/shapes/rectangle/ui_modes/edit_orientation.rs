// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 51988613f34f08d3dec8f9ce5ffb90c863426119e0021047de1c98d718f848a3
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: edit_orientation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 476, 498, 506, 567, 1585
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":476,"record_offset":56924,"rva":8639472,"va":5377348592}
// 编译实例 RVAs: 0x83d3f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140922fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t939615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377348592,"raw_hex":"e85f560e00","size":5,"soft_fail":false,"target_va":5378289584,"va":5377349964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413047b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11301258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377348592,"raw_hex":"e88a71ac00","size":5,"soft_fail":false,"target_va":5388650416,"va":5377349153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469e00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12765583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377348592,"raw_hex":"e88fc9c200","size":5,"soft_fail":false,"target_va":5390114304,"va":5377348716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t34192231","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377348592,"raw_hex":"e867bb0902","size":5,"soft_fail":false,"target_va":5411542296,"va":5377350060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2738279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377348592,"raw_hex":"e867c82900","size":5,"soft_fail":false,"target_va":5380088032,"va":5377349748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4574175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377348592,"raw_hex":"e8dfcb4500","size":5,"soft_fail":false,"target_va":5381923136,"va":5377348956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4574356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377348592,"raw_hex":"e894cc4500","size":5,"soft_fail":false,"target_va":5381923344,"va":5377348983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4571549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377348592,"raw_hex":"e89dc14500","size":5,"soft_fail":false,"target_va":5381920256,"va":5377348702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4570858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377348592,"raw_hex":"e8eabe4500","size":5,"soft_fail":false,"target_va":5381920112,"va":5377349249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":476,"record_offset":58060,"parent_record_offset":56924,"ancestor_record_offset":56924,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":476,"record_offset":57976,"parent_record_offset":56924,"ancestor_record_offset":56924,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/widget_params.rs
//     evidence={"module":476,"record_offset":58812,"parent_record_offset":56924,"ancestor_record_offset":56924,"inlinee":26157,"source_line":92,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":476,"record_offset":58404,"parent_record_offset":56924,"ancestor_record_offset":56924,"inlinee":12702,"source_line":267,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":476,"record_offset":57948,"parent_record_offset":56924,"ancestor_record_offset":56924,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":476,"record_offset":57884,"parent_record_offset":56924,"ancestor_record_offset":56924,"inlinee":11304,"source_line":1744,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":476,"record_offset":57148,"parent_record_offset":56924,"ancestor_record_offset":56924,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":476,"record_offset":57624,"parent_record_offset":56924,"ancestor_record_offset":56924,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":57204,"parent_record_offset":57148,"ancestor_record_offset":56924,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":58084,"parent_record_offset":58060,"ancestor_record_offset":56924,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":476,"record_offset":58108,"parent_record_offset":58084,"ancestor_record_offset":56924,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":476,"record_offset":57580,"parent_record_offset":57556,"ancestor_record_offset":56924,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":476,"record_offset":57496,"parent_record_offset":57472,"ancestor_record_offset":56924,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$327::neg
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":476,"record_offset":57912,"parent_record_offset":57884,"ancestor_record_offset":56924,"inlinee":11306,"source_line":694,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":476,"record_offset":57796,"parent_record_offset":57624,"ancestor_record_offset":56924,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":476,"record_offset":58024,"parent_record_offset":58000,"ancestor_record_offset":56924,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":476,"record_offset":57232,"parent_record_offset":57204,"ancestor_record_offset":56924,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":476,"record_offset":57260,"parent_record_offset":57232,"ancestor_record_offset":56924,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":476,"record_offset":58000,"parent_record_offset":57976,"ancestor_record_offset":56924,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/math.rs
//     evidence={"module":476,"record_offset":58524,"parent_record_offset":58428,"ancestor_record_offset":56924,"inlinee":4753,"source_line":177,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":476,"record_offset":58428,"parent_record_offset":58404,"ancestor_record_offset":56924,"inlinee":12703,"source_line":198,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":476,"record_offset":57448,"parent_record_offset":57424,"ancestor_record_offset":56924,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":476,"record_offset":57472,"parent_record_offset":57448,"ancestor_record_offset":56924,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":476,"record_offset":57556,"parent_record_offset":57448,"ancestor_record_offset":56924,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":476,"record_offset":58548,"parent_record_offset":58524,"ancestor_record_offset":56924,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":476,"record_offset":57176,"parent_record_offset":57148,"ancestor_record_offset":56924,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":476,"record_offset":57300,"parent_record_offset":57148,"ancestor_record_offset":56924,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":476,"record_offset":57424,"parent_record_offset":57148,"ancestor_record_offset":56924,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$0::to_widget_name_hash
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6667
// 首条位置: {"module":506,"inlinee":26215}
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
// IPI IDs: 0x6669
// 首条位置: {"module":567,"inlinee":26217}
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
// IPI IDs: 0x6D44
// 首条位置: {"module":498,"inlinee":27972}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1585,"record_offset":35248,"parent_record_offset":35192,"ancestor_record_offset":35192,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1585,"record_offset":35304,"parent_record_offset":35192,"ancestor_record_offset":35192,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":35328,"parent_record_offset":35304,"ancestor_record_offset":35192,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1585,"record_offset":35272,"parent_record_offset":35248,"ancestor_record_offset":35192,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01585.jsonl.gz"}
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
// 首条位置: {"module":476,"record_offset":59016,"rva":8641040,"va":5377350160}
// 编译实例 RVAs: 0x83da10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2269358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377350160,"raw_hex":"e8525fddff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377350841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2269600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377350160,"raw_hex":"e8605eddff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377351099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064f9d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2024796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377350160,"raw_hex":"e8a41ae1ff","size":5,"soft_fail":false,"target_va":5375326672,"va":5377351463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4584198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377350160,"raw_hex":"e806f34500","size":5,"soft_fail":false,"target_va":5381934528,"va":5377350325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4584264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377350160,"raw_hex":"e848f34500","size":5,"soft_fail":false,"target_va":5381934560,"va":5377350291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146c880
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12774117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377350160,"raw_hex":"e8e5eac200","size":5,"soft_fail":false,"target_va":5390125184,"va":5377351062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34496152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377350160,"raw_hex":"e8985e0e02","size":5,"soft_fail":false,"target_va":5411847168,"va":5377351011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34189736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377350160,"raw_hex":"e8a8b10902","size":5,"soft_fail":false,"target_va":5411540992,"va":5377351251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5489438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377350160,"raw_hex":"e81ec35300","size":5,"soft_fail":false,"target_va":5382840208,"va":5377350765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34189684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377350160,"raw_hex":"e874b10902","size":5,"soft_fail":false,"target_va":5411540963,"va":5377351274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4572829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377350160,"raw_hex":"e89dc64500","size":5,"soft_fail":false,"target_va":5381923136,"va":5377350302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4570005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377350160,"raw_hex":"e895bb4500","size":5,"soft_fail":false,"target_va":5381920256,"va":5377350246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4533996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377350160,"raw_hex":"e8ec2e4500","size":5,"soft_fail":false,"target_va":5381884480,"va":5377350479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":476,"record_offset":59828,"parent_record_offset":59016,"ancestor_record_offset":59016,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":476,"record_offset":59940,"parent_record_offset":59016,"ancestor_record_offset":59016,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":476,"record_offset":59528,"parent_record_offset":59016,"ancestor_record_offset":59016,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":476,"record_offset":59912,"parent_record_offset":59016,"ancestor_record_offset":59016,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":61028,"parent_record_offset":59016,"ancestor_record_offset":59016,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":476,"record_offset":59440,"parent_record_offset":59016,"ancestor_record_offset":59016,"inlinee":26166,"source_line":254,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::hovered
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":476,"record_offset":59252,"parent_record_offset":59016,"ancestor_record_offset":59016,"inlinee":26159,"source_line":250,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":476,"record_offset":59340,"parent_record_offset":59016,"ancestor_record_offset":59016,"inlinee":26164,"source_line":259,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":476,"record_offset":60156,"parent_record_offset":59016,"ancestor_record_offset":59016,"inlinee":26169,"source_line":112,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":476,"record_offset":61000,"parent_record_offset":59016,"ancestor_record_offset":59016,"inlinee":26171,"source_line":24,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":60052,"parent_record_offset":60028,"ancestor_record_offset":59016,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":60076,"parent_record_offset":60052,"ancestor_record_offset":59016,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":61208,"parent_record_offset":61180,"ancestor_record_offset":59016,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":59996,"parent_record_offset":59968,"ancestor_record_offset":59016,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":60028,"parent_record_offset":59996,"ancestor_record_offset":59016,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":61180,"parent_record_offset":61124,"ancestor_record_offset":59016,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":476,"record_offset":59968,"parent_record_offset":59940,"ancestor_record_offset":59016,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":476,"record_offset":59876,"parent_record_offset":59852,"ancestor_record_offset":59016,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":60424,"parent_record_offset":60396,"ancestor_record_offset":59016,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":61152,"parent_record_offset":61124,"ancestor_record_offset":59016,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":61124,"parent_record_offset":61100,"ancestor_record_offset":59016,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":60484,"parent_record_offset":60452,"ancestor_record_offset":59016,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":60452,"parent_record_offset":60424,"ancestor_record_offset":59016,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":61100,"parent_record_offset":61076,"ancestor_record_offset":59016,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":60520,"parent_record_offset":60484,"ancestor_record_offset":59016,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":60360,"parent_record_offset":60332,"ancestor_record_offset":59016,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":60332,"parent_record_offset":60304,"ancestor_record_offset":59016,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":60276,"parent_record_offset":60248,"ancestor_record_offset":59016,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":60304,"parent_record_offset":60276,"ancestor_record_offset":59016,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":60248,"parent_record_offset":60220,"ancestor_record_offset":59016,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":476,"record_offset":60220,"parent_record_offset":60192,"ancestor_record_offset":59016,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":476,"record_offset":60192,"parent_record_offset":60156,"ancestor_record_offset":59016,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":476,"record_offset":60868,"parent_record_offset":60844,"ancestor_record_offset":59016,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":60396,"parent_record_offset":60360,"ancestor_record_offset":59016,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":476,"record_offset":60544,"parent_record_offset":60520,"ancestor_record_offset":59016,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":476,"record_offset":60568,"parent_record_offset":60544,"ancestor_record_offset":59016,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":476,"record_offset":59644,"parent_record_offset":59552,"ancestor_record_offset":59016,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":476,"record_offset":60784,"parent_record_offset":60760,"ancestor_record_offset":59016,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":476,"record_offset":60616,"parent_record_offset":60592,"ancestor_record_offset":59016,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":476,"record_offset":60592,"parent_record_offset":60568,"ancestor_record_offset":59016,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":476,"record_offset":59612,"parent_record_offset":59588,"ancestor_record_offset":59016,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":476,"record_offset":59588,"parent_record_offset":59552,"ancestor_record_offset":59016,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":476,"record_offset":59552,"parent_record_offset":59528,"ancestor_record_offset":59016,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":476,"record_offset":59852,"parent_record_offset":59828,"ancestor_record_offset":59016,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":476,"record_offset":60760,"parent_record_offset":60360,"ancestor_record_offset":59016,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":476,"record_offset":59756,"parent_record_offset":59668,"ancestor_record_offset":59016,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":60844,"parent_record_offset":60156,"ancestor_record_offset":59016,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":59780,"parent_record_offset":59756,"ancestor_record_offset":59016,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":60104,"parent_record_offset":60076,"ancestor_record_offset":59016,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":476,"record_offset":59668,"parent_record_offset":59644,"ancestor_record_offset":59016,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":476,"record_offset":59724,"parent_record_offset":59700,"ancestor_record_offset":59016,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":476,"record_offset":59700,"parent_record_offset":59668,"ancestor_record_offset":59016,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":60904,"parent_record_offset":60868,"ancestor_record_offset":59016,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":60928,"parent_record_offset":60904,"ancestor_record_offset":59016,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":476,"record_offset":60960,"parent_record_offset":60868,"ancestor_record_offset":59016,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::dragged
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":476,"record_offset":59464,"parent_record_offset":59440,"ancestor_record_offset":59016,"inlinee":26167,"source_line":150,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::hovered
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":476,"record_offset":59276,"parent_record_offset":59252,"ancestor_record_offset":59016,"inlinee":26161,"source_line":133,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::UiState::intersecting
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":476,"record_offset":59368,"parent_record_offset":59340,"ancestor_record_offset":59016,"inlinee":26165,"source_line":168,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$13::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":476,"record_offset":59304,"parent_record_offset":59276,"ancestor_record_offset":59016,"inlinee":26163,"source_line":11,"archive":"_pdb/B/modules/00476.jsonl.gz"}
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
// 首条位置: {"module":476,"record_offset":61492,"rva":8642384,"va":5377351504}
// 编译实例 RVAs: 0x83df50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2270088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377351504,"raw_hex":"e8785cddff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377351571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2270672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377351504,"raw_hex":"e8305addff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377352171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089ada0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t379899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377351504,"raw_hex":"e8fbcb0500","size":5,"soft_fail":false,"target_va":5377732000,"va":5377352096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: edit_orientation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34495498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377351504,"raw_hex":"e80a5c0e02","size":5,"soft_fail":false,"target_va":5411847168,"va":5377351665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34188995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377351504,"raw_hex":"e8c3ae0902","size":5,"soft_fail":false,"target_va":5411540992,"va":5377351992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34188939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377351504,"raw_hex":"e88bae0902","size":5,"soft_fail":false,"target_va":5411540963,"va":5377352019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t392279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377351504,"raw_hex":"e857fc0500","size":5,"soft_fail":false,"target_va":5377744208,"va":5377351924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":61704,"parent_record_offset":61492,"ancestor_record_offset":61492,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":63140,"parent_record_offset":61492,"ancestor_record_offset":61492,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":476,"record_offset":61852,"parent_record_offset":61492,"ancestor_record_offset":61492,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":63224,"parent_record_offset":61492,"ancestor_record_offset":61492,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":476,"record_offset":62536,"parent_record_offset":61492,"ancestor_record_offset":61492,"inlinee":26173,"source_line":153,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":476,"record_offset":63112,"parent_record_offset":61492,"ancestor_record_offset":61492,"inlinee":26175,"source_line":189,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::set_ui_pass
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":476,"record_offset":63084,"parent_record_offset":61492,"ancestor_record_offset":61492,"inlinee":26174,"source_line":173,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":61760,"parent_record_offset":61736,"ancestor_record_offset":61492,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":61784,"parent_record_offset":61760,"ancestor_record_offset":61492,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":63296,"parent_record_offset":63272,"ancestor_record_offset":61492,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":61736,"parent_record_offset":61704,"ancestor_record_offset":61492,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":476,"record_offset":63272,"parent_record_offset":63248,"ancestor_record_offset":61492,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":476,"record_offset":63248,"parent_record_offset":63224,"ancestor_record_offset":61492,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":62112,"parent_record_offset":62084,"ancestor_record_offset":61492,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":62172,"parent_record_offset":62140,"ancestor_record_offset":61492,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":62140,"parent_record_offset":62112,"ancestor_record_offset":61492,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":476,"record_offset":62208,"parent_record_offset":62172,"ancestor_record_offset":61492,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":62048,"parent_record_offset":62020,"ancestor_record_offset":61492,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":63164,"parent_record_offset":63140,"ancestor_record_offset":61492,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":62020,"parent_record_offset":61992,"ancestor_record_offset":61492,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":61964,"parent_record_offset":61936,"ancestor_record_offset":61492,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":61992,"parent_record_offset":61964,"ancestor_record_offset":61492,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":476,"record_offset":61936,"parent_record_offset":61908,"ancestor_record_offset":61492,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":476,"record_offset":61908,"parent_record_offset":61880,"ancestor_record_offset":61492,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":476,"record_offset":61880,"parent_record_offset":61852,"ancestor_record_offset":61492,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":63188,"parent_record_offset":63164,"ancestor_record_offset":61492,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":476,"record_offset":62084,"parent_record_offset":62048,"ancestor_record_offset":61492,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":476,"record_offset":62232,"parent_record_offset":62208,"ancestor_record_offset":61492,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":476,"record_offset":62256,"parent_record_offset":62232,"ancestor_record_offset":61492,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":476,"record_offset":62472,"parent_record_offset":62448,"ancestor_record_offset":61492,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":476,"record_offset":62304,"parent_record_offset":62280,"ancestor_record_offset":61492,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":476,"record_offset":62280,"parent_record_offset":62256,"ancestor_record_offset":61492,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":476,"record_offset":62448,"parent_record_offset":62048,"ancestor_record_offset":61492,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00476.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":476,"record_offset":61808,"parent_record_offset":61784,"ancestor_record_offset":61492,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00476.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 14; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::init_photomode_state::PhotomodeState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>,bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>,bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>", "bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>", "bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time_of_day::TodSequence>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::weather::Weather>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_settings::GladeSettings>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::new
// 信息 ID: 191f6612005dd9ac5724cfe6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate; module=567, inline_record=71472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier
// 信息 ID: b74f6d89fcd0391c0edf256d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0xad86b6 -> 0x83da10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::widget
// 信息 ID: 455381f0e6b26c116578959f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate @ 0xad8687 -> 0x83d3f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$0::to_widget_name_hash
// 信息 ID: 48398e64dab5f0049a123cc3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::resources::ui_state::UiState::add<ref$<country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit> >; module=506, inline_record=95212; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::ui_edit_rectangle_rotate; module=567, inline_record=72100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::signifier::closure$0
// 信息 ID: c9150392e2f8eb98f1383377
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::signifier", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::signifier::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::signifier::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::signifier::closure_env$0,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x83c102 -> 0x83df50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure$0
// 信息 ID: 269f523da06945007ef2abef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0 <- core::iter::adapters::flatten::impl$3::next<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::intersect::Triangle,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::flatten::impl$3::next<core::slice::iter::Iter<tuple$<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2> >,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::impl$1::widget::closure_env$0>; module=1585, inline_record=35192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
