// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/wall/shapes/shape_placement_signifiers.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 532b1f97645b845e768c2d6db5388f7ba3e60d33b33f85b7667eeee49de3da9c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shape_placement_signifiers.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 497, 1585, 1610
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
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
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":497,"record_offset":63128,"rva":9305184,"va":5378014304}
// 编译实例 RVAs: 0x8dfc60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14014ba20
//     owner=unresolved_namespace; count=3
//     target_aliases=14 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7948972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e854b586ff","size":5,"soft_fail":false,"target_va":5370067488,"va":5378016455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2933094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e89a3ed3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378014593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cc60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3918695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e867cb3b00","size":5,"soft_fail":false,"target_va":5381934176,"va":5378015476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14130e700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10675296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e860e4a200","size":5,"soft_fail":false,"target_va":5388691200,"va":5378015899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12084338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e87264b800","size":5,"soft_fail":false,"target_va":5390099200,"va":5378014857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141467300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12087951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e88f72b800","size":5,"soft_fail":false,"target_va":5390103296,"va":5378015340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414677f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12089351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e80778b800","size":5,"soft_fail":false,"target_va":5390104560,"va":5378015204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141467ce0
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12089900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e82c7ab800","size":5,"soft_fail":false,"target_va":5390105824,"va":5378015919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12096379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e87b93b800","size":5,"soft_fail":false,"target_va":5390111280,"va":5378014896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12098433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e8819bb800","size":5,"soft_fail":false,"target_va":5390113104,"va":5378014666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469af0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12098469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e8a59bb800","size":5,"soft_fail":false,"target_va":5390113520,"va":5378015046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146a2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12100928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e840a5b800","size":5,"soft_fail":false,"target_va":5390115488,"va":5378014555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a8940
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33327104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e80088fc01","size":5,"soft_fail":false,"target_va":5411342656,"va":5378015547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33832826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e87a3f0402","size":5,"soft_fail":false,"target_va":5411847952,"va":5378015121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4639360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e880ca4600","size":5,"soft_fail":false,"target_va":5382655024,"va":5378015659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t33525861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e86590ff01","size":5,"soft_fail":false,"target_va":5411542064,"va":5378016198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33525924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e8a490ff01","size":5,"soft_fail":false,"target_va":5411542296,"va":5378016367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4679710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e81e684700","size":5,"soft_fail":false,"target_va":5382694656,"va":5378014941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::as_curve3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3899110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e8e67e3b00","size":5,"soft_fail":false,"target_va":5381914128,"va":5378015013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_curve3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3904676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378014304,"raw_hex":"e8a4943b00","size":5,"soft_fail":false,"target_va":5381919216,"va":5378014535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":497,"record_offset":64480,"parent_record_offset":63128,"ancestor_record_offset":63128,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":497,"record_offset":63888,"parent_record_offset":63128,"ancestor_record_offset":63128,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":497,"record_offset":66116,"parent_record_offset":63128,"ancestor_record_offset":63128,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":63388,"parent_record_offset":63128,"ancestor_record_offset":63128,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":497,"record_offset":64892,"parent_record_offset":63128,"ancestor_record_offset":63128,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":497,"record_offset":66032,"parent_record_offset":63128,"ancestor_record_offset":63128,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":497,"record_offset":63920,"parent_record_offset":63128,"ancestor_record_offset":63128,"inlinee":24415,"source_line":118,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::wall::valid_enclosure::ShapeParameters::expand
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/valid_enclosure/mod.rs
//     evidence={"module":497,"record_offset":63352,"parent_record_offset":63128,"ancestor_record_offset":63128,"inlinee":27924,"source_line":220,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":66200,"parent_record_offset":63128,"ancestor_record_offset":63128,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":64576,"parent_record_offset":63128,"ancestor_record_offset":63128,"inlinee":9593,"source_line":169,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::try_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":64172,"parent_record_offset":63128,"ancestor_record_offset":63128,"inlinee":9594,"source_line":859,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":497,"record_offset":64112,"parent_record_offset":63128,"ancestor_record_offset":63128,"inlinee":24561,"source_line":207,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":497,"record_offset":63948,"parent_record_offset":63128,"ancestor_record_offset":63128,"inlinee":24559,"source_line":191,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":497,"record_offset":64636,"parent_record_offset":63128,"ancestor_record_offset":63128,"inlinee":24685,"source_line":31,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":63600,"parent_record_offset":63576,"ancestor_record_offset":63128,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":63576,"parent_record_offset":63484,"ancestor_record_offset":63128,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":63512,"parent_record_offset":63484,"ancestor_record_offset":63128,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":63484,"parent_record_offset":63460,"ancestor_record_offset":63128,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":63460,"parent_record_offset":63436,"ancestor_record_offset":63128,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":64212,"parent_record_offset":64172,"ancestor_record_offset":63128,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":66284,"parent_record_offset":66228,"ancestor_record_offset":63128,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":66140,"parent_record_offset":66116,"ancestor_record_offset":63128,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":497,"record_offset":64540,"parent_record_offset":64508,"ancestor_record_offset":63128,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":497,"record_offset":64508,"parent_record_offset":64480,"ancestor_record_offset":63128,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":497,"record_offset":64136,"parent_record_offset":64112,"ancestor_record_offset":63128,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_infinite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":497,"record_offset":63984,"parent_record_offset":63948,"ancestor_record_offset":63128,"inlinee":24560,"source_line":547,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":497,"record_offset":64052,"parent_record_offset":64016,"ancestor_record_offset":63128,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":497,"record_offset":66164,"parent_record_offset":66140,"ancestor_record_offset":63128,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":497,"record_offset":63544,"parent_record_offset":63512,"ancestor_record_offset":63128,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":497,"record_offset":66776,"parent_record_offset":66744,"ancestor_record_offset":63128,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":497,"record_offset":66620,"parent_record_offset":66584,"ancestor_record_offset":63128,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":63412,"parent_record_offset":63388,"ancestor_record_offset":63128,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":497,"record_offset":66080,"parent_record_offset":66056,"ancestor_record_offset":63128,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":497,"record_offset":66316,"parent_record_offset":66284,"ancestor_record_offset":63128,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":497,"record_offset":66348,"parent_record_offset":66316,"ancestor_record_offset":63128,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":497,"record_offset":66056,"parent_record_offset":66032,"ancestor_record_offset":63128,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/wall/shapes/shape_placement_signifiers.rs
//     evidence={"module":497,"record_offset":64924,"parent_record_offset":64892,"ancestor_record_offset":63128,"inlinee":27926,"source_line":170,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":497,"record_offset":66556,"parent_record_offset":66528,"ancestor_record_offset":63128,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":497,"record_offset":66584,"parent_record_offset":66556,"ancestor_record_offset":63128,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":497,"record_offset":66744,"parent_record_offset":66556,"ancestor_record_offset":63128,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":497,"record_offset":64724,"parent_record_offset":64700,"ancestor_record_offset":63128,"inlinee":14890,"source_line":154,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":66228,"parent_record_offset":66200,"ancestor_record_offset":63128,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":66256,"parent_record_offset":66228,"ancestor_record_offset":63128,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":66392,"parent_record_offset":66228,"ancestor_record_offset":63128,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":497,"record_offset":64700,"parent_record_offset":64636,"ancestor_record_offset":63128,"inlinee":24686,"source_line":90,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":497,"record_offset":66528,"parent_record_offset":66228,"ancestor_record_offset":63128,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":497,"record_offset":51772,"rva":9301104,"va":5378010224}
// 编译实例 RVAs: 0x8dec70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3334056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378010224,"raw_hex":"e85820cdff","size":5,"soft_fail":false,"target_va":5374676336,"va":5378010387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2929178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378010224,"raw_hex":"e8e64dd3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378010661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2929942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378010224,"raw_hex":"e8ea4ad3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378011441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14064f9d0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2685545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378010224,"raw_hex":"e89705d7ff","size":5,"soft_fail":false,"target_va":5375326672,"va":5378012212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3923525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378010224,"raw_hex":"e845de3b00","size":5,"soft_fail":false,"target_va":5381934528,"va":5378010998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146a060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12104027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378010224,"raw_hex":"e85bb1b800","size":5,"soft_fail":false,"target_va":5390114912,"va":5378010880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14146c880
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12113859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378010224,"raw_hex":"e8c3d7b800","size":5,"soft_fail":false,"target_va":5390125184,"va":5378011320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33835875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378010224,"raw_hex":"e8634b0402","size":5,"soft_fail":false,"target_va":5411847168,"va":5378011288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33529064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378010224,"raw_hex":"e8e89cff01","size":5,"soft_fail":false,"target_va":5411540992,"va":5378011923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4829572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378010224,"raw_hex":"e884b14900","size":5,"soft_fail":false,"target_va":5382840208,"va":5378010631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33529008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378010224,"raw_hex":"e8b09cff01","size":5,"soft_fail":false,"target_va":5411540963,"va":5378011950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33530412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378010224,"raw_hex":"e82ca2ff01","size":5,"soft_fail":false,"target_va":5411541776,"va":5378011359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33530151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378010224,"raw_hex":"e827a1ff01","size":5,"soft_fail":false,"target_va":5411542064,"va":5378011908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33530281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378010224,"raw_hex":"e8a9a1ff01","size":5,"soft_fail":false,"target_va":5411542296,"va":5378012010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2783","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378010224,"raw_hex":"e8df0a0000","size":5,"soft_fail":false,"target_va":5378014304,"va":5378011516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":497,"record_offset":53116,"parent_record_offset":51772,"ancestor_record_offset":51772,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":497,"record_offset":53200,"parent_record_offset":51772,"ancestor_record_offset":51772,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":497,"record_offset":56660,"parent_record_offset":51772,"ancestor_record_offset":51772,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":497,"record_offset":51992,"parent_record_offset":51772,"ancestor_record_offset":51772,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":497,"record_offset":56292,"parent_record_offset":51772,"ancestor_record_offset":51772,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":56064,"parent_record_offset":51772,"ancestor_record_offset":51772,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":497,"record_offset":54316,"parent_record_offset":51772,"ancestor_record_offset":51772,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_walls.rs
//     evidence={"module":497,"record_offset":52020,"parent_record_offset":51772,"ancestor_record_offset":51772,"inlinee":26470,"source_line":39,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":497,"record_offset":53416,"parent_record_offset":51772,"ancestor_record_offset":51772,"inlinee":26169,"source_line":112,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/signifier_create.rs
//     evidence={"module":497,"record_offset":56036,"parent_record_offset":51772,"ancestor_record_offset":51772,"inlinee":26171,"source_line":24,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":497,"record_offset":54776,"parent_record_offset":51772,"ancestor_record_offset":51772,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":54344,"parent_record_offset":51772,"ancestor_record_offset":51772,"inlinee":24690,"source_line":816,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":53312,"parent_record_offset":53288,"ancestor_record_offset":51772,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":53336,"parent_record_offset":53312,"ancestor_record_offset":51772,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":56240,"parent_record_offset":56216,"ancestor_record_offset":51772,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":53256,"parent_record_offset":53228,"ancestor_record_offset":51772,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":53288,"parent_record_offset":53256,"ancestor_record_offset":51772,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":56216,"parent_record_offset":56160,"ancestor_record_offset":51772,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":497,"record_offset":53228,"parent_record_offset":53200,"ancestor_record_offset":51772,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":497,"record_offset":53164,"parent_record_offset":53140,"ancestor_record_offset":51772,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":53740,"parent_record_offset":53712,"ancestor_record_offset":51772,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":56188,"parent_record_offset":56160,"ancestor_record_offset":51772,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":56160,"parent_record_offset":56136,"ancestor_record_offset":51772,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":53800,"parent_record_offset":53768,"ancestor_record_offset":51772,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":53768,"parent_record_offset":53740,"ancestor_record_offset":51772,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":56136,"parent_record_offset":56112,"ancestor_record_offset":51772,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":53836,"parent_record_offset":53800,"ancestor_record_offset":51772,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":53672,"parent_record_offset":53636,"ancestor_record_offset":51772,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":53636,"parent_record_offset":53600,"ancestor_record_offset":51772,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":53564,"parent_record_offset":53528,"ancestor_record_offset":51772,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":53600,"parent_record_offset":53564,"ancestor_record_offset":51772,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":53528,"parent_record_offset":53492,"ancestor_record_offset":51772,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":497,"record_offset":53492,"parent_record_offset":53456,"ancestor_record_offset":51772,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":497,"record_offset":53456,"parent_record_offset":53416,"ancestor_record_offset":51772,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":497,"record_offset":54184,"parent_record_offset":54160,"ancestor_record_offset":51772,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":53712,"parent_record_offset":53672,"ancestor_record_offset":51772,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":54432,"parent_record_offset":54372,"ancestor_record_offset":51772,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":497,"record_offset":53860,"parent_record_offset":53836,"ancestor_record_offset":51772,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":497,"record_offset":53884,"parent_record_offset":53860,"ancestor_record_offset":51772,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":497,"record_offset":52840,"parent_record_offset":52816,"ancestor_record_offset":51772,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":497,"record_offset":52348,"parent_record_offset":52324,"ancestor_record_offset":51772,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":497,"record_offset":52432,"parent_record_offset":52408,"ancestor_record_offset":51772,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":497,"record_offset":52372,"parent_record_offset":52348,"ancestor_record_offset":51772,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":497,"record_offset":52908,"parent_record_offset":52212,"ancestor_record_offset":51772,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":497,"record_offset":53932,"parent_record_offset":53908,"ancestor_record_offset":51772,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":497,"record_offset":53908,"parent_record_offset":53884,"ancestor_record_offset":51772,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":497,"record_offset":57088,"parent_record_offset":57060,"ancestor_record_offset":51772,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":497,"record_offset":52580,"parent_record_offset":52552,"ancestor_record_offset":51772,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":497,"record_offset":52552,"parent_record_offset":52524,"ancestor_record_offset":51772,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":497,"record_offset":54804,"parent_record_offset":54776,"ancestor_record_offset":51772,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":497,"record_offset":54552,"parent_record_offset":54528,"ancestor_record_offset":51772,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":497,"record_offset":53140,"parent_record_offset":53116,"ancestor_record_offset":51772,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":497,"record_offset":54076,"parent_record_offset":53672,"ancestor_record_offset":51772,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":54160,"parent_record_offset":53416,"ancestor_record_offset":51772,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":497,"record_offset":52700,"parent_record_offset":52676,"ancestor_record_offset":51772,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":53364,"parent_record_offset":53336,"ancestor_record_offset":51772,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":497,"record_offset":54460,"parent_record_offset":54432,"ancestor_record_offset":51772,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":497,"record_offset":54488,"parent_record_offset":54460,"ancestor_record_offset":51772,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":497,"record_offset":54220,"parent_record_offset":54184,"ancestor_record_offset":51772,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":497,"record_offset":54244,"parent_record_offset":54220,"ancestor_record_offset":51772,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":497,"record_offset":54276,"parent_record_offset":54184,"ancestor_record_offset":51772,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":497,"record_offset":52864,"parent_record_offset":52840,"ancestor_record_offset":51772,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":497,"record_offset":54692,"parent_record_offset":54660,"ancestor_record_offset":51772,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":497,"record_offset":54660,"parent_record_offset":54636,"ancestor_record_offset":51772,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":497,"record_offset":54636,"parent_record_offset":54612,"ancestor_record_offset":51772,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":497,"record_offset":54612,"parent_record_offset":54588,"ancestor_record_offset":51772,"inlinee":10290,"source_line":466,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":497,"record_offset":54528,"parent_record_offset":54372,"ancestor_record_offset":51772,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":497,"record_offset":52492,"parent_record_offset":52464,"ancestor_record_offset":51772,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":497,"record_offset":52524,"parent_record_offset":52492,"ancestor_record_offset":51772,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":497,"record_offset":52464,"parent_record_offset":52212,"ancestor_record_offset":51772,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":497,"record_offset":52324,"parent_record_offset":52212,"ancestor_record_offset":51772,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":497,"record_offset":52936,"parent_record_offset":52212,"ancestor_record_offset":51772,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":497,"record_offset":52408,"parent_record_offset":52212,"ancestor_record_offset":51772,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":497,"record_offset":52296,"parent_record_offset":52212,"ancestor_record_offset":51772,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":497,"record_offset":52816,"parent_record_offset":52792,"ancestor_record_offset":51772,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":497,"record_offset":52080,"parent_record_offset":52056,"ancestor_record_offset":51772,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":497,"record_offset":52104,"parent_record_offset":52080,"ancestor_record_offset":51772,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":497,"record_offset":52792,"parent_record_offset":52624,"ancestor_record_offset":51772,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":497,"record_offset":52136,"parent_record_offset":52104,"ancestor_record_offset":51772,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":52732,"parent_record_offset":52648,"ancestor_record_offset":51772,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":52676,"parent_record_offset":52648,"ancestor_record_offset":51772,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":53028,"parent_record_offset":52212,"ancestor_record_offset":51772,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":52648,"parent_record_offset":52624,"ancestor_record_offset":51772,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":52188,"parent_record_offset":52164,"ancestor_record_offset":51772,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":52164,"parent_record_offset":52104,"ancestor_record_offset":51772,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":52212,"parent_record_offset":52188,"ancestor_record_offset":51772,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":52268,"parent_record_offset":52212,"ancestor_record_offset":51772,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":497,"record_offset":52624,"parent_record_offset":52212,"ancestor_record_offset":51772,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":497,"record_offset":52056,"parent_record_offset":52020,"ancestor_record_offset":51772,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":497,"record_offset":54724,"parent_record_offset":54692,"ancestor_record_offset":51772,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":54372,"parent_record_offset":54344,"ancestor_record_offset":51772,"inlinee":24691,"source_line":812,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":53084,"parent_record_offset":52020,"ancestor_record_offset":51772,"inlinee":26471,"source_line":165,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":497,"record_offset":54404,"parent_record_offset":54372,"ancestor_record_offset":51772,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::normalize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":497,"record_offset":54588,"parent_record_offset":54344,"ancestor_record_offset":51772,"inlinee":25387,"source_line":68,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":497,"record_offset":59424,"rva":9303744,"va":5378012864}
// 编译实例 RVAs: 0x8df6c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2931448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378012864,"raw_hex":"e80845d3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378012931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2932096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378012864,"raw_hex":"e88042d3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378013595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089ada0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-281669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378012864,"raw_hex":"e8bbb3fbff","size":5,"soft_fail":false,"target_va":5377732000,"va":5378013664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33834093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378012864,"raw_hex":"e86d440402","size":5,"soft_fail":false,"target_va":5411847168,"va":5378013070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33527477","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378012864,"raw_hex":"e8b596ff01","size":5,"soft_fail":false,"target_va":5411540992,"va":5378013510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33333565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378012864,"raw_hex":"e83da1fc01","size":5,"soft_fail":false,"target_va":5411347040,"va":5378013470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33527465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378012864,"raw_hex":"e8a996ff01","size":5,"soft_fail":false,"target_va":5411540963,"va":5378013493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-269093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378012864,"raw_hex":"e8dbe4fbff","size":5,"soft_fail":false,"target_va":5377744208,"va":5378013296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":59636,"parent_record_offset":59424,"ancestor_record_offset":59424,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":61260,"parent_record_offset":59424,"ancestor_record_offset":59424,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":59784,"parent_record_offset":59424,"ancestor_record_offset":59424,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::EntityCommands::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":497,"record_offset":60956,"parent_record_offset":59424,"ancestor_record_offset":59424,"inlinee":9427,"source_line":1347,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":61344,"parent_record_offset":59424,"ancestor_record_offset":59424,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":497,"record_offset":60928,"parent_record_offset":59424,"ancestor_record_offset":59424,"inlinee":26796,"source_line":211,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":497,"record_offset":60376,"parent_record_offset":59424,"ancestor_record_offset":59424,"inlinee":26173,"source_line":153,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":59692,"parent_record_offset":59668,"ancestor_record_offset":59424,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":59716,"parent_record_offset":59692,"ancestor_record_offset":59424,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":61416,"parent_record_offset":61392,"ancestor_record_offset":59424,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":59668,"parent_record_offset":59636,"ancestor_record_offset":59424,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":61392,"parent_record_offset":61368,"ancestor_record_offset":59424,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":497,"record_offset":61368,"parent_record_offset":61344,"ancestor_record_offset":59424,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":61108,"parent_record_offset":61080,"ancestor_record_offset":59424,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":59968,"parent_record_offset":59940,"ancestor_record_offset":59424,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":61192,"parent_record_offset":61168,"ancestor_record_offset":59424,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":61168,"parent_record_offset":61136,"ancestor_record_offset":59424,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":61136,"parent_record_offset":61108,"ancestor_record_offset":59424,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":60028,"parent_record_offset":59996,"ancestor_record_offset":59424,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":59996,"parent_record_offset":59968,"ancestor_record_offset":59424,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":60064,"parent_record_offset":60028,"ancestor_record_offset":59424,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":59904,"parent_record_offset":59876,"ancestor_record_offset":59424,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":61284,"parent_record_offset":61260,"ancestor_record_offset":59424,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":59876,"parent_record_offset":59812,"ancestor_record_offset":59424,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":60504,"parent_record_offset":60476,"ancestor_record_offset":59424,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":60532,"parent_record_offset":60504,"ancestor_record_offset":59424,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":60476,"parent_record_offset":60448,"ancestor_record_offset":59424,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":497,"record_offset":60448,"parent_record_offset":60420,"ancestor_record_offset":59424,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":497,"record_offset":60420,"parent_record_offset":60376,"ancestor_record_offset":59424,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":59848,"parent_record_offset":59812,"ancestor_record_offset":59424,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":61308,"parent_record_offset":61284,"ancestor_record_offset":59424,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":61080,"parent_record_offset":61044,"ancestor_record_offset":59424,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":59940,"parent_record_offset":59904,"ancestor_record_offset":59424,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":59812,"parent_record_offset":59784,"ancestor_record_offset":59424,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":497,"record_offset":61012,"parent_record_offset":60984,"ancestor_record_offset":59424,"inlinee":4959,"source_line":553,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::queue_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":497,"record_offset":61044,"parent_record_offset":61012,"ancestor_record_offset":59424,"inlinee":4960,"source_line":612,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::EntityCommands::queue
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/commands/mod.rs
//     evidence={"module":497,"record_offset":60984,"parent_record_offset":60956,"ancestor_record_offset":59424,"inlinee":9420,"source_line":1798,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":497,"record_offset":60088,"parent_record_offset":60064,"ancestor_record_offset":59424,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":497,"record_offset":60112,"parent_record_offset":60088,"ancestor_record_offset":59424,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":497,"record_offset":60328,"parent_record_offset":60304,"ancestor_record_offset":59424,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":497,"record_offset":60136,"parent_record_offset":60112,"ancestor_record_offset":59424,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":497,"record_offset":60160,"parent_record_offset":60136,"ancestor_record_offset":59424,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":497,"record_offset":60304,"parent_record_offset":59904,"ancestor_record_offset":59424,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":59740,"parent_record_offset":59716,"ancestor_record_offset":59424,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":497,"record_offset":61460,"rva":9304576,"va":5378013696}
// 编译实例 RVAs: 0x8dfa00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2932276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378013696,"raw_hex":"e8cc41d3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378013759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2932732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378013696,"raw_hex":"e80440d3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378014231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14089ada0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shape_placement_signifiers.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-282289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378013696,"raw_hex":"e84fb1fbff","size":5,"soft_fail":false,"target_va":5377732000,"va":5378014284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33526855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378013696,"raw_hex":"e84794ff01","size":5,"soft_fail":false,"target_va":5411540992,"va":5378014132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33526804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378013696,"raw_hex":"e81494ff01","size":5,"soft_fail":false,"target_va":5411540963,"va":5378014154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-269865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378013696,"raw_hex":"e8d7e1fbff","size":5,"soft_fail":false,"target_va":5377744208,"va":5378014068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":61668,"parent_record_offset":61460,"ancestor_record_offset":61460,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":62928,"parent_record_offset":61460,"ancestor_record_offset":61460,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":497,"record_offset":61816,"parent_record_offset":61460,"ancestor_record_offset":61460,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":63012,"parent_record_offset":61460,"ancestor_record_offset":61460,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":497,"record_offset":62352,"parent_record_offset":61460,"ancestor_record_offset":61460,"inlinee":26173,"source_line":153,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/signifiers/mod.rs
//     evidence={"module":497,"record_offset":62900,"parent_record_offset":61460,"ancestor_record_offset":61460,"inlinee":27003,"source_line":199,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":61724,"parent_record_offset":61700,"ancestor_record_offset":61460,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":61748,"parent_record_offset":61724,"ancestor_record_offset":61460,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":63084,"parent_record_offset":63060,"ancestor_record_offset":61460,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":61700,"parent_record_offset":61668,"ancestor_record_offset":61460,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":497,"record_offset":63060,"parent_record_offset":63036,"ancestor_record_offset":61460,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":497,"record_offset":63036,"parent_record_offset":63012,"ancestor_record_offset":61460,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":62076,"parent_record_offset":62048,"ancestor_record_offset":61460,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":62136,"parent_record_offset":62104,"ancestor_record_offset":61460,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":497,"record_offset":62104,"parent_record_offset":62076,"ancestor_record_offset":61460,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":62012,"parent_record_offset":61984,"ancestor_record_offset":61460,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":62952,"parent_record_offset":62928,"ancestor_record_offset":61460,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":61984,"parent_record_offset":61956,"ancestor_record_offset":61460,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":61928,"parent_record_offset":61900,"ancestor_record_offset":61460,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":61956,"parent_record_offset":61928,"ancestor_record_offset":61460,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":497,"record_offset":61900,"parent_record_offset":61872,"ancestor_record_offset":61460,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":497,"record_offset":61872,"parent_record_offset":61844,"ancestor_record_offset":61460,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":497,"record_offset":61844,"parent_record_offset":61816,"ancestor_record_offset":61460,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":62976,"parent_record_offset":62952,"ancestor_record_offset":61460,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":497,"record_offset":62048,"parent_record_offset":62012,"ancestor_record_offset":61460,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":497,"record_offset":62288,"parent_record_offset":62264,"ancestor_record_offset":61460,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":497,"record_offset":62264,"parent_record_offset":62012,"ancestor_record_offset":61460,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":497,"record_offset":61772,"parent_record_offset":61748,"ancestor_record_offset":61460,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00497.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 170
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D16
// 首条位置: {"module":497,"inlinee":27926}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 205
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AE7
// 首条位置: {"module":1585,"inlinee":72423}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1585,"record_offset":481592,"parent_record_offset":481548,"ancestor_record_offset":481548,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1585,"record_offset":481648,"parent_record_offset":481548,"ancestor_record_offset":481548,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1585,"record_offset":481620,"parent_record_offset":481548,"ancestor_record_offset":481548,"inlinee":4588,"source_line":75,"archive":"_pdb/B/modules/01585.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 10; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

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
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line
// 信息 ID: ff399ac6b0d3abb4d38bdf54
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers @ 0x8df17c -> 0x8dfc60 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::snapping::snapping_maintain::snapping_signifiers @ 0xa6ac03 -> 0x8dfc60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line::closure$0
// 信息 ID: 5f5de6913f5328858bacda9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line; module=497, inline_record=64924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line::closure$1
// 信息 ID: 71e12c768ae804bbc221e230
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line::closure_env$1> >; module=1585, inline_record=481548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers
// 信息 ID: 95f3bfd67dd7f3888bb3e40a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3e403722793d94bbe12b4c47: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> > @ 0xb14d2b -> 0x8dec70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::shapes::shape_placement::ShapePlacement>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)> > > @ 0x14dc73d -> 0x8dec70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure$1
// 信息 ID: 15a60947c44cf1e8b0733887
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1 <- core::ptr::drop_in_place<country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x8da7d3 -> 0x8df6c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure$2
// 信息 ID: 7dec2fbab29343c5405a3f7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$2 <- core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x8da7a2 -> 0x8dfa00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
